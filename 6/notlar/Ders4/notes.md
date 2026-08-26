# Gün 6 · Ders 4 — Supertrait, Orphan Rule, Newtype, Blanket impl

## Supertrait — "önce şu olacaksın"

```rust
trait Boss: Unit + Display {
    fn phase(&self) -> u8;

    fn intro(&self) -> String {
        format!("[FAZ {}] {} sahneye cikti ({} can)", self.phase(), self, self.hp())
    }
}
```

`: Unit + Display` kısmı şunu söyler: **`Boss` olmak için önce `Unit` ve `Display`
olmalısın.** Yani her patron bir birimdir, ama her birim patron değildir.

Pratik faydası varsayılan gövdede görünür: `self`'i `{}` ile yazdırabiliyor **ve**
`self.hp()` diyebiliyoruz — ikisi de garanti altında. Garanti olmasaydı derleyici bu
satırı kabul etmezdi.

Sözleşme sağlanmazsa:

```rust
struct Slime { hp: i32 }
impl Boss for Slime { ... }
```
```
error[E0277]: `Slime` doesn't implement `std::fmt::Display`
```

Çok seviyeli de olur:

```rust
trait Boss: Unit + Display { ... }     // iki supertrait birden
```

Ders 3'teki `Ord: Eq + PartialOrd` zinciri de tam olarak budur — `CritMultiplier`'da
`Eq` kopunca `Ord` da kopuyordu.

## Orphan rule

> `impl Trait for Type` yazabilmek için **`Trait` ya da `Type` sizin olmalı.**

| Yazım | Durum |
|---|---|
| `impl Display for Dragon` | ✓ `Dragon` benim |
| `impl Describe for u32` | ✓ `Describe` benim |
| `impl Display for Vec<i32>` | ✗ ikisi de başkasının |

```
error[E0117]: only traits defined in the current crate can be implemented
              for types defined outside of the crate
```

**Neden var:** iki ayrı crate `impl Display for Vec<i32>` yazsaydı, ikisini birden
kullanan üçüncü crate hangi implementasyonu seçeceğini bilemezdi. Buna **coherence**
denir. Kural sert ama ekosistemi ayakta tutan şey bu.

Java/C#'ta bu sorun yoktur çünkü zaten başkasının tipine sonradan interface
ekleyemezsiniz. Rust'ta ekleyebilirsiniz — buna *retroactive implementation* denir ve
trait sisteminin en güçlü yanıdır. Orphan rule, o gücün faturasıdır.

Kendi trait'inizi std'nin tipine uygulamak serbesttir:

```rust
impl Describe for u32  { ... }     // 55u32.describe()      -> "55 hasar"
impl Describe for &str { ... }     // "alev topu".describe()
```

## Newtype — iki ayrı fayda

### 1. Orphan rule'un etrafından dolaşmak

`Vec<&str>`'e `Display` yazamıyorduk. Sarmalayınca tip bizim oluyor:

```rust
struct Party(Vec<&'static str>);
impl Display for Party { ... }        // artık serbest
```

```
Birlik[Archer + Knight + Healer]
```

### 2. Tip güvenliği — asıl faydası

Ders 3'teki `Hp`/`Mana` ayrımı da bir newtype'tı. İkisi de `i32` tutuyordu ama
karışmıyorlardı.

Bedeli sıfırdır: `struct Party(Vec<&str>)` bellekte `Vec<&str>` ile aynı yeri kaplar.

## Blanket implementation

```rust
impl<T: Display> Taunt for T {
    fn taunt(&self) -> String { format!("{} seni korkutmuyor mu?", self) }
}
```

"**`Display` olan her tip** `Taunt` kazansın." Tek satır, milyonlarca tip:

```
42.taunt()       -> 42 seni korkutmuyor mu?
"goblin".taunt() -> goblin seni korkutmuyor mu?
dragon.taunt()   -> Dragon (ofke 15) seni korkutmuyor mu?
party.taunt()    -> Birlik[Archer + Knight + Healer] seni korkutmuyor mu?
```

std bunu çok kullanır ve dün gördüğümüz iki "bedava"nın kaynağı budur:

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

pub trait DamageType: sealed::Seal {
    fn multiplier(&self) -> f64;
}
```

Başka bir crate `DamageType`'ı implemente etmek isterse önce `sealed::Seal`'i
implemente etmesi gerekir; ama o trait'e erişemez. Sonuç: yeni hasar tiplerini
**yalnızca siz** ekleyebilirsiniz.

Ne işe yarar: trait'e ileride yeni metot eklerseniz kimsenin kodu kırılmaz, çünkü
implemente eden herkes sizsiniz. std bunu API'sini kilitlemek için kullanır.

## Object safety (dyn compatibility) — her trait `dyn` olamaz

Ders 2'de `Box<dyn Unit>` yazabildik. Ama her trait bunu kaldırmaz:

```rust
trait Summon {
    fn summon() -> Self;             // self almıyor, Self döndürüyor
}

let s: Box<dyn Summon> = ...;        // E0038
//  error[E0038]: the trait `Summon` is not dyn compatible
//  ...because associated function `summon` has no `self` parameter
```

(Kuralın eski adı *object safety*, derleyicinin bugünkü dilinde *dyn compatibility*.
İkisini de duyacaksınız, aynı şey.)

Bir trait'in `dyn` olabilmesi için metotlarının **nesne güvenli** olması gerekir:

| Kural | Neden |
|---|---|
| generic metot olmayacak — `fn f<T>(&self)` | vtable'da metot başına **tek** adres var; her `T` için ayrı kod ister, hangisinin adresini koyacağız? |
| `Self` döndürmeyecek — `fn summon() -> Self` | çağıran `Self`'in ne olduğunu bilmiyor; boyutu belirsiz |
| `self` almayan metot olmayacak | vtable'a bakabilmek için elde bir nesne olmalı |
| associated const olmayacak — `const MAX: u32;` | vtable bir **fonksiyon** tablosu; sabitin orada yeri yok |

Dördünün de tek bir sebebi var: **vtable sabit boyutlu bir tablodur.** İçine ancak
derleme zamanında sayısı ve imzası belli olan girdiler konabilir.

Trait tasarımıyla ilgili bir kısıt, bu yüzden burada: trait'i yazarken `dyn` olarak
kullanılabilmesini isteyip istemediğinize karar veriyorsunuz.

### Kaçış yolu: `where Self: Sized`

Sorunlu metodu vtable'ın dışında bırakabilirsiniz:

```rust
trait Summonable {
    fn name(&self) -> &str;                       // vtable'a girer

    fn summon() -> Self where Self: Sized;        // vtable'a GIRMEZ
}
```

Artık `Box<dyn Summonable>` derleniyor. Bedeli: `summon()`'ı yalnızca somut tip
üzerinden çağırabilirsiniz (`Dragon::summon()`), `dyn` üzerinden değil.

Bugünkü `Boss: Unit + Display` nesne güvenlidir — üç trait'in de tüm metotları
`&self` alıyor ve `Self` döndürmüyor. Yani `Vec<Box<dyn Boss>>` yazılabilir.

## Toparlarsak

| Araç | Ne için |
|---|---|
| supertrait | "önce şu olmalısın" — varsayılan gövdede o davranışa güvenmek |
| orphan rule | coherence: aynı impl'in iki kere yazılmasını engellemek |
| newtype | orphan rule'u aşmak **ve** tipleri karıştırmamak |
| blanket impl | bir koşulu sağlayan tüm tiplere tek satırla davranış vermek |
| sealed trait | trait'i dışarıya kapatmak, ileride genişletebilmek |
| object safety | trait'in `dyn` olarak kullanılıp kullanılamayacağını belirleyen kurallar |
