# Gün 4 · Ders 3 — Enum'lar

Günün cümlesi: **geçersiz durumu temsil edilemez kıl.**

## 1. Enum nedir: sınırlı seçenekler

Bir değerin alabileceği ihtimaller sayılıysa, enum onları **tipin içine** yazar:

```rust
enum Isik {
    Kirmizi,
    Sari,
    Yesil,
}

let isik = Isik::Kirmizi;
```

`Isik` tipinde bir değer bu üçünden biridir. Dördüncü bir ihtimal **yoktur** —
uydurmak isteseniz de derleyici izin vermez.

## 2. `match` ile okumak

Enum'un içinden bilgi almanın yolu `match`:

```rust
match isik {
    Isik::Kirmizi => "dur",
    Isik::Sari    => "hazırlan",
    Isik::Yesil   => "geç",
}
```

Üç varyantı da yazmak zorundasınız; biri eksikse kod derlenmez.

## 3. `impl` — enum'lara da metot yazılır

Struct'lardakiyle aynı, `self` seçimi de aynı:

```rust
impl Isik {
    fn saniye(&self) -> u32 {
        match self {
            Isik::Kirmizi => 45,
            Isik::Sari    => 4,
            Isik::Yesil   => 30,
        }
    }
}
```

## 4. Durum makinesi

Enum'un en doğal işi: bir şeyin **hangi durumda** olduğunu tutmak ve geçişleri tek
yerde toplamak.

```rust
impl Isik {
    fn sonraki(&self) -> Isik {
        match self {
            Isik::Kirmizi => Isik::Yesil,
            Isik::Yesil   => Isik::Sari,
            Isik::Sari    => Isik::Kirmizi,
        }
    }
}
```

Geçiş tablosu koda dönüştü. "Yanlış duruma geçme" diye bir ihtimal kalmadı.

## 5. Sıçrama: varyantlar veri taşır

Buraya kadar olan kısım C'nin enum'u ile aynı. Rust'ın farkı: **her varyant kendi
verisini taşıyabilir** ve varyantlar birbirine benzemek zorunda değildir.

```rust
enum Sekil {
    Nokta,                              // veri yok
    Cember { r: f64 },                  // isimli alan
    Dikdortgen { en: f64, boy: f64 },   // iki isimli alan
    Ucgen(f64, f64, f64),               // isimsiz üçlü
}
```

Tek cümleyle: **`struct` bir "ve"dir** (hem `x` hem `y`), **`enum` bir "veya"dır**
(ya `Cember` ya `Ucgen`). Bu yüzden enum'a *sum type* deniyor.

Veriyi çıkarmak yine `match` ile olur; desen hem hangisi olduğunu söyler hem içindekini verir:

```rust
fn alan(&self) -> f64 {
    match self {
        Sekil::Nokta                  => 0.0,
        Sekil::Cember { r }           => 3.14159 * r * r,
        Sekil::Dikdortgen { en, boy } => en * boy,
        Sekil::Ucgen(a, b, c)         => { ... }
    }
}
```

Bu yapı Haskell, OCaml, F# ve Swift'te var; Java 17'ye (sealed interface) ve C# 9'a
(record + pattern matching) sonradan geldi, Go'da hâlâ yok.

## 6. Geçersiz durum temsil edilemez

DNA dizisi tutuyorsunuz diyelim. Metinle:

```rust
let baz = "X";        // derlenir, çalışır, sessizce saçmalar
```

Enum'la:

```rust
enum Baz { A, T, G, C }
let baz = Baz::X;     // E0599 — böyle bir varyant yok
```

Fark, hatanın **ne zaman** yakalandığı: metinde çalışma zamanında (belki de hiç),
enum'da derleme zamanında. Kendi kodunuzda "aslında enum olması gereken kaç string var"
diye bakın; genelde sandığınızdan fazladır.

## 7. `Option` — null'un yerine geçen enum

Gün 3'ten beri gördüğünüz `Option`, std'de tanımlı **sıradan bir enum**:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Hepsi bu. `Some` ve `None` iki varyant; `match` ile açtığınız şey de bu. Dilin
ayrıcalıklı bir parçası değil — isteseniz aynısını siz yazabilirdiniz.

### Rust'ta null yok, peki yerine ne var?

> **Kontrolsüz null referansları yerine Rust, bir değerin varlığını veya yokluğunu
> tip seviyesinde zorunlu bir sözleşme hâline getirir.**

Çoğu dilde her referans gizliden gizliye "ya da null" taşır. Java'da `String ad`
yazdığınızda `ad` bir metin **ya da** null'dur; hangisi olduğunu tipe bakarak
anlayamazsınız. Sözleşme yok, temenni var: kontrol etmeyi unutursanız program
çalışma zamanında patlar.

Rust'ta bu bir temenni değil, derleyicinin dayattığı bir sözleşme. Üç kurala iniyor:

**1. Tipi `T` olan bir değer kesinlikle vardır.** Bu, derleme zamanında garanti
altındadır. `let n: i32` yazdıysanız `n` bir sayıdır; "belki de yoktur" ihtimali
diye bir şey yoktur, çünkü yazacak bir null yok.

```rust
let n: i32 = None;    // E0308 — böyle bir şey yazamazsınız
```

**2. Bir değerin bulunmama ihtimali varsa, tipi `Option<T>` olmak zorundadır.**
Yani "olmayabilir" bilgisi yorumda, dokümanda ya da isim geleneğinde değil,
**imzada** durur:

```rust
fn ilk_negatif(s: &[i32]) -> Option<i32>
```

Bu imzayı okuyan herkes sonucun boş gelebileceğini bilir. `-> i32` yazsaydınız
boş dönmenin yolu yoktu.

**3. `Option<T>`'yi açmadan içindeki `T`'ye erişemezsiniz.** Sarmalayıcı bir kutu
gibidir; kutuyu açmadan içindekini kullanamazsınız:

```rust
let d: Option<i32> = Some(5);
let e: i32 = d;        // E0308 — Option<i32>, i32 değildir
let f = Some(5) + 1;   // E0369 — Option'a toplama yapılmaz
```

Bu üçü birleşince şu sonuç çıkıyor: **null kontrolünü unutmak mümkün değil.**
Unutursanız kod derlenmiyor; hata çalışma zamanına kalmıyor.

Gün 3'te "imza bir sözleşmedir" demiştik: `&str` mi `&mut String` mi yazdığınız,
fonksiyonun ne yapacağını okuyucuya söylüyordu. `Option<T>` aynı fikrin devamı —
bu sefer sözleşmenin konusu **değerin var olup olmadığı.**

### Hangi durumda hangi tip?

Kural 2'nin pratikteki hâli: modelleme yaparken her alan için "bu **olmayabilir** mi?"
diye sorarsınız. Cevap hayırsa `T`, evetse `Option<T>`.

| Durum | Mantıksal karşılık | Rust tipi | C# / C++ karşılığı |
|---|---|---|---|
| Kullanıcı ID | Her kullanıcının ID'si olmak zorundadır | `u64` | `long` / `int` |
| İkinci isim | Bazı insanların ikinci ismi yoktur | `Option<String>` | `string?` (nullable) |
| Dizi eleman sayısı | Bir listenin eleman adedi her zaman vardır | `usize` | `int` |
| Arama sonucu | Aranan nesne bulunamayabilir | `Option<&Kayit>` | `Kayit` (null dönebilir) |
| Karakter canı | Can eksik olamaz, ama 0 olabilir | `i32` | `int` |

Sağdaki sütunun tamamına dikkat edin: C# ve C++ tarafında **ikinci ve dördüncü satır
diğerlerinden ayırt edilemiyor.** `Kayit` dönen bir fonksiyonun null dönüp dönmediğini
imzadan anlayamazsınız; öğrenmenin tek yolu dokümanı okumak ya da çökmektir.

Son satır en çok karıştırılan yer: **0 ile "yok" aynı şey değildir.** Canı 0 olan bir
karakter vardır ve ölmüştür; canı `None` olan karakterin canı *bilinmiyordur*. İkisi
farklı sorular olduğu için farklı tipler kullanılır. Gereksiz yere `Option` sarmak,
sonradan her yerde açmak zorunda kalmak demektir.

### Kutuyu açmanın yolları

```rust
match bulunan {                     // en açık yol, iki durumu da yazarsınız
    Some(n) => println!("{}", n),
    None => println!("değer yok"),
}

if let Some(n) = bulunan { }        // sadece dolu hâliyle ilgileniyorsanız

bulunan.unwrap()                    // doluysa değeri verir, BOŞSA PANİKLER
bulunan.expect("ölçüm bekleniyordu")// aynısı, ama panik mesajını siz yazarsınız
bulunan.unwrap_or(0)                // boşsa varsayılan döner, panik yok
```

`unwrap()` "ben bunun dolu olduğunu biliyorum" demektir. Haklıysanız sorun yok;
haksızsanız program orada durur. Bu kötü bir şey değil — kötü olan, yanlış değerle
sessizce devam etmektir. Yine de üretim kodunda `expect` daha iyidir, çünkü panik
mesajı size **neyin** beklendiğini söyler.

> Null referansını 1965'te Tony Hoare icat etti, 2009'da buna *"benim milyar dolarlık
> hatam"* dedi: "Basitçe uygulanabilir olduğu için koydum; sayısız hataya, açığa ve
> çökmeye yol açtı." `Option` aynı ihtiyacı karşılıyor ama derleyicinin gözü önünde.

## Kenar not 1 — bellekte enum

Bir enum'un boyutu **etiket + en büyük varyant + hizalama**. Tüm varyantlar aynı yeri
paylaşır, en büyüğü belirler.

İlginç kısım, **niche optimization**:

```rust
size_of::<Box<i32>>()          // 8
size_of::<Option<Box<i32>>>()  // 8   — None bedava
size_of::<i32>()               // 4
size_of::<Option<i32>>()       // 8   — burada etiket için yer gerekti
```

`Box` asla null olamaz, yani "sıfır" bit deseni boşta duruyor; derleyici `None`'ı
oraya yerleştiriyor. `i32`'de her bit deseni geçerli bir sayı olduğu için boşluk yok.

Aynı şey referanslar için de geçerli — safe Rust'ta `&T` asla null olamaz:

```rust
size_of::<&i32>()               // 8
size_of::<Option<&i32>>()       // 8    — None bedava
size_of::<*const i32>()         // 8    — ham işaretçi, null OLABİLİR
size_of::<Option<*const i32>>() // 16   — boş desen yok, etikete yer gerekti
```

Son iki satır garantinin kanıtı: ham işaretçi null olabildiği için derleyicinin
çalabileceği boş desen kalmıyor ve boyut ikiye katlanıyor. Referansta katlanmıyor,
çünkü "geçerli bir `T`'yi gösterir" garantisi tipin içinde yazılı.

Sonuç: Rust'ta nullable pointer bedavadır.

## Kenar not 2 — sayısal değer

```rust
enum HttpDurum { Tamam = 200, Bulunamadi = 404 }
HttpDurum::Bulunamadi as i32      // 404
```

Sadece **veri taşımayan** enum'larda çalışır, C uyumluluğu için vardır.
