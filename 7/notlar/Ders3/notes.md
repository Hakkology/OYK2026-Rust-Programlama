# Gün 7 · Ders 3 — Struct'larda Lifetime, `'static`, Çoklu Ömür

Tanık ifadesinin tam metni bir yerde duruyor. Ayrıştırıcı onu **kopyalamadan** dilimliyor
— ama o zaman metinden uzun yaşayamaz. Bugünün konusu bu takas.

## Referans tutan struct

```rust
struct Transcript { source: &str }
```

```
error[E0106]: missing lifetime specifier
```

Derleyicinin sorusu yine aynı: "bu referans ne kadar yaşıyor?" Struct'ın cevabı olmalı:

```rust
struct Transcript<'a> {
    source: &'a str,
}
```

Anlamı: **`Transcript`, içinde tuttuğu referanstan uzun yaşayamaz.** `'a` artık tipin
parçasıdır — `Transcript<'a>` diye okunur, `Vec<T>`'deki `T` gibi.

Bu yüzden `impl` bloğu da ömrü ilan etmek zorundadır:

```rust
impl<'a> Transcript<'a> {
    fn new(source: &'a str) -> Transcript<'a> { ... }
}
```

### Kazanç ölçülebilir

```
kaynak 75 bayt, struct 16 bayt (sadece pointer + uzunluk)
OwnedTranscript 24 bayt (String: ptr + len + cap)
```

`Transcript` metni kopyalamıyor: içinde sadece bir `&str` var (ptr + uzunluk = 16 bayt).
75 baytlık metin tek bir yerde duruyor. Büyük dosyalarda bu ciddi kazanç.

## Metotlarda `'a` yazmıyoruz

```rust
fn first_line(&self) -> &str
```

`&self` olduğu için çıkışa `self`'in ömrü atanır — Ders 2'deki üçüncü kural. Yani bu
struct'ın bütün metotları `'a`'dan muaf; ömrü **bir kez**, struct tanımında yazıyorsunuz.

## İki ömürlü metot

Bazen bir metotta iki farklı ömür olur:

```rust
fn replace_source<'b>(&'b mut self, new_source: &'a str) -> &'b str {
    let previous = self.source;
    self.source = new_source;
    previous
}
```

- `'a` — **metnin** ömrü, struct'ın tipinden geliyor
- `'b` — bu **ödünç almanın** ömrü, sadece bu çağrı boyunca

Yeni kaynak `'a` kadar yaşamak zorunda (struct onu saklayacak); dönen referans ise
yalnızca ödünç süresince geçerli.

```
eski kaynagin ilk satiri: tanik A: araba maviydi
yeni ilk satir          : tanik A duzeltme: araba lacivertti
```

### Aynı kalıp, dilim üzerinde

Referans tutan struct yalnızca `&str` için değil — her dilim için aynı:

```rust
struct EvidenceLog<'a> { entries: &'a [u32] }

impl<'a> EvidenceLog<'a> {
    fn update_entries<'b>(&'b mut self, new_entries: &'a [u32]) -> &'b [u32] { ... }
}
```

Burada `'b`'nin ne işe yaradığı **canlı** görünüyor. Dönen `old`, `log`'dan alınan
**mut ödüncü** taşıyor; o ödünç `old`'un son kullanımına kadar sürüyor. İkisini aynı
satırda yazdıramazsınız:

```rust
println!("{:?} {:?}", old, log.entries);
```

```
error[E0502]: cannot borrow `log.entries` as immutable because it is also borrowed as mutable
```

Ayrı satırlara bölünce çalışır — NLL devreye girer, `old`'un ömrü son kullanımında biter
(Ders 2). `'b`, "bu ödünç ne kadar sürecek" sorusunun cevabıdır.

```
eski kayit: [1041, 1042, 1055]
yeni kayit: [2001, 2002]
```

## Karar: referans mı tut, sahiplen mi?

| | referans tut (`&'a str`) | sahiplen (`String`) |
|---|---|---|
| kopya | yok | var |
| hız | yüksek | tahsis maliyeti |
| ömür | kısıtlı — kaynağa bağlı | bağımsız |
| imza | `'a` taşır, yayılır | temiz |

**Pratik kural:** kütüphane API'sinde sahiplen, iç hesaplamada referans tut.

Sınır şurada net görünüyor — fonksiyon içinde `String` üretip ondan referans tutan bir
struct **döndüremezsiniz**:

```rust
fn build_broken() -> Transcript<'static> {
    let text = String::from("ifade metni");
    Transcript { source: &text }
}
```

```
error[E0515]: cannot return value referencing local variable `text`
```

Gün 2'deki sarkan referansın ta kendisi. Çözüm: sahiplenen sürümü döndürün.

## `'static`'in iki anlamı

Sınıfın en çok karıştırdığı yer burası.

| | ne demek |
|---|---|
| `&'static T` | bu **referans** program boyu geçerli |
| `T: 'static` | bu **tip** ödünç referans içermiyor |

```rust
static AGENCY: &str = "Gece Vardiyasi Burosu";   // (a) ikilinin içinde duruyor

fn archive<T: 'static>(item: T) -> T { item }    // (b) bound
```

İkincisi çok daha yaygın ve **çok daha zayıf** bir şart:

```rust
archive(String::from("dosya arsive kaldirildi"));   // çalışır
```

`String` `'static`'tir — içinde başkasına ait referans yok. Program boyu yaşaması
gerekmiyor, sadece **kimseden ödünç almamış** olması yeterli.

```rust
let local = String::from("gecici");
archive(&local);
```

```
error[E0597]: `local` does not live long enough
```

`&local`'ın tipi `&'x String` ve `'x` program boyu değil — bu yüzden `T: 'static`
sağlanmıyor.

> Kısaca: `'static` **"sonsuza kadar yaşar"** demek değildir. Bound olarak
> **"ödünç içermiyor"** demektir.

## Çoklu ömür parametresi

İki girdi farklı ömürlere sahip olabilir; dönen yalnızca birine bağlıysa bunu yazarsınız:

```rust
fn cross_check<'a, 'b>(primary: &'a str, secondary: &'b str) -> (&'a str, bool) {
    let confirmed = secondary.contains(primary);
    (primary, confirmed)
}
```

`secondary` kısa yaşayan bir değer olabilir; dönen dilim `primary`'ye bağlı olduğu için
sorun çıkmaz:

```
'araba maviydi' dogrulandi mi: true
'araba maviydi' dogrulandi mi: false
```

Açık yazmamızın sebebi ikisinin **ayrı** ömürler olduğunun görülmesi; elision bu imzayı
zaten böyle çıkarırdı.
