# Gün 6 · Ders 4 (v1) — Supertrait, Orphan Rule, Newtype, Blanket impl

> Ders 4'ün **ikinci anlatımı**. Konular aynı; dünya farklı: **ulaşım ağı**.

## Supertrait — "önce şu olacaksın"

```rust
trait Express: Vehicle + Display {
    fn skipped_stops(&self) -> u8;

    fn banner(&self) -> String {
        format!("[EKSPRES] {} - {} durak atliyor, {} kisilik",
            self, self.skipped_stops(), self.capacity())
    }
}
```

`: Vehicle + Display` kısmı şunu söyler: **`Express` olmak için önce `Vehicle` ve
`Display` olmalısın.** Yani her ekspres bir araçtır, ama her araç ekspres değildir.

Pratik faydası varsayılan gövdede görünür: `self`'i `{}` ile yazdırabiliyor **ve**
`self.capacity()` diyebiliyoruz — ikisi de garanti altında. Garanti olmasaydı derleyici
bu satırı kabul etmezdi.

```
[EKSPRES] M1 Fahrettin Altay (5 vagon) - 4 durak atliyor, 1100 kisilik
[EKSPRES] M2 Bornova (3 vagon) - 2 durak atliyor, 660 kisilik
```

Sözleşme sağlanmazsa:

```rust
impl Express for Minibus { ... }    // Minibus'ta Display yok
```

```
error[E0277]: `Minibus` doesn't implement `std::fmt::Display`
```

Ders 3'teki `Ord: Eq + PartialOrd` zinciri de tam olarak budur — `Discount`'ta `Eq`
kopunca `Ord` da kopuyordu.

## Orphan rule

Kural tek cümle: **`impl Trait for Type` yazabilmek için trait ya da tip sizin olmalı.**

| | |
|---|---|
| `impl Display for Metro` | ✓ `Metro` benim |
| `impl Announce for u32` | ✓ `Announce` benim |
| `impl Display for Vec<i32>` | ✗ ikisi de başkasının |

```
error[E0117]: only traits defined in the current crate can be implemented
              for types defined outside of the crate
```

Neden: iki ayrı crate aynı impl'i yazsaydı, ikisini birden kullanan üçüncü crate hangi
implementasyonu seçeceğini bilemezdi. Buna **coherence** denir. Kural sert ama ekosistemi
ayakta tutan şey bu.

Öte yandan kendi trait'inizi std'nin tipine uygulamak **serbesttir**:

```rust
impl Announce for u32 { ... }
impl Announce for &str { ... }
```

```
34 numarali sefer
'son sefer 23:40' anonsu (15 harf)
```

Java'da `Integer`'a metot ekleyemezsiniz. Rust'ta ekleyebilirsiniz — buna *retroactive
implementation* denir ve trait sisteminin en güçlü yanıdır. Orphan rule, o gücün
faturasıdır.

## Newtype — iki ayrı fayda

### 1. Orphan rule'un etrafından dolaşmak

`Vec<String>`'e `Display` yazamıyorduk. Sarmalayınca tip bizim oluyor:

```rust
struct Route(Vec<String>);
impl Display for Route { ... }        // artık serbest
```

```
Guzergah[Konak > Cankaya > Basmane]
```

### 2. Tip güvenliği — asıl faydası

Ders 3'teki `Fare`/`Distance` ayrımı da bir newtype'tı. İkisi de sayı tutuyordu ama
karışmıyorlardı.

Bedeli sıfırdır: `struct Route(Vec<String>)` bellekte `Vec<String>` ile aynı yeri kaplar.

## Blanket implementation

```rust
trait Loudspeaker { fn over_speaker(&self) -> String; }

impl<T: Display> Loudspeaker for T {
    fn over_speaker(&self) -> String { format!(">> {} <<", self) }
}
```

"**`Display` olan her tip** `Loudspeaker` kazansın." Tek satır, milyonlarca tip:

```
>> 42 <<
>> peron degisikligi <<
>> M1 Fahrettin Altay (5 vagon) <<
>> Guzergah[Konak > Cankaya > Basmane] <<
```

std bunu çok kullanır ve Ders 3'te gördüğümüz iki "bedava"nın kaynağı budur:

```rust
impl<T: Display> ToString for T          // Display yazınca to_string() bedava
impl<T, U: From<T>> Into<U> for T        // From yazınca into() bedava
```

Yani "bedava gelme" diye bir sihir yok; birileri oturup bu tek satırı yazmış.

## Sealed trait — dışarıdan implemente edilemeyen trait

```rust
mod sealed {
    pub trait Seal {}          // modülün dışına çıkmıyor
}

pub trait TicketKind: sealed::Seal {
    fn multiplier(&self) -> f64;
}
```

Başka bir crate `TicketKind`'ı implemente etmek isterse önce `sealed::Seal`'i implemente
etmesi gerekir; ama o trait'e erişemez. Sonuç: yeni bilet türlerini **yalnızca siz**
ekleyebilirsiniz.

Ne işe yarar: trait'e ileride yeni metot eklerseniz kimsenin kodu kırılmaz, çünkü
implemente eden herkes sizsiniz. std bunu API'sini kilitlemek için kullanır.

## Object safety (dyn compatibility) — her trait `dyn` olamaz

Ders 2'de `Box<dyn Vehicle>` yazabildik. Ama her trait bunu kaldırmaz:

```rust
trait Spawnable {
    fn spawn() -> Self;              // self almıyor, Self döndürüyor
}

let s: Box<dyn Spawnable> = ...;     // E0038
```

```
error[E0038]: the trait `Spawnable` is not dyn compatible
...because associated function `spawn` has no `self` parameter
```

(Kuralın eski adı *object safety*, derleyicinin bugünkü dilinde *dyn compatibility*.
İkisini de duyacaksınız, aynı şey.)

Bir trait'in `dyn` olabilmesi için metotlarının **nesne güvenli** olması gerekir:

| Kural | Neden |
|---|---|
| generic metot olmayacak — `fn f<T>(&self)` | vtable'da metot başına **tek** adres var |
| `Self` döndürmeyecek — `fn spawn() -> Self` | çağıran `Self`'in boyutunu bilmiyor |
| `self` almayan metot olmayacak | vtable'a bakabilmek için elde bir nesne olmalı |
| associated const olmayacak | vtable bir **fonksiyon** tablosu |

Dördünün de tek bir sebebi var: **vtable sabit boyutlu bir tablodur.**

### Kaçış yolu: `where Self: Sized`

```rust
trait Deployable {
    fn label(&self) -> String;                    // vtable'a girer
    fn deploy() -> Self where Self: Sized;        // vtable'a GİRMEZ
}
```

Artık `Box<dyn Deployable>` derleniyor. Bedeli: `deploy()`'u yalnızca somut tip
üzerinden çağırabilirsiniz:

```rust
hazir.deploy();     // E0599: no method named `deploy` found for `Box<dyn Deployable>`
```

Bugünkü `Express: Vehicle + Display` nesne güvenlidir — üç trait'in de tüm metotları
`&self` alıyor. Yani `Vec<Box<dyn Express>>` yazılabilir:

```
[EKSPRES] M1 Fahrettin Altay (5 vagon) - 4 durak atliyor, 1100 kisilik
[EKSPRES] M2 Bornova (3 vagon) - 2 durak atliyor, 660 kisilik
```

## Toparlarsak

| Araç | Ne için |
|---|---|
| supertrait | "önce şu olmalısın" — varsayılan gövdede o davranışa güvenmek |
| orphan rule | coherence: aynı impl'in iki kere yazılmasını engellemek |
| newtype | orphan rule'u aşmak **ve** tipleri karıştırmamak |
| blanket impl | bir koşulu sağlayan tüm tiplere tek satırla davranış vermek |
| sealed trait | trait'i dışarıya kapatmak, ileride genişletebilmek |
| object safety | trait'in `dyn` olarak kullanılıp kullanılamayacağı |
