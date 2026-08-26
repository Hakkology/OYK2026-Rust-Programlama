# Gün 6 · Ders 1 — Generics ve Monomorphization

## Problem

Aynı işi yapan iki fonksiyon, tek fark tip:

```rust
fn strongest_i32(stats: &[i32]) -> i32 { ... }   // vuruş güçleri
fn strongest_f64(stats: &[f64]) -> f64 { ... }   // kritik çarpanları
```

Gövdeleri harfi harfine aynı. Üçüncü bir tip gelince üçüncü kopyayı yazacak mısınız?

## Generic — ama tek başına yetmez

```rust
fn strongest<T>(stats: &[T]) -> T {
    if x > en { ... }
}
```

```
error[E0369]: binary operation `>` cannot be applied to type `T`
```

Derleyici `T`'nin ne olduğunu bilmiyor, dolayısıyla **ne yapabileceğini** de bilmiyor.
`>` her tipte tanımlı değil. Çözüm, `T`'den ne beklediğinizi söylemek:

```rust
fn strongest<T: PartialOrd + Copy>(stats: &[T]) -> T
```

- `PartialOrd` → "karşılaştırılabilir olacak"
- `Copy` → "kopyalanabilir olacak" (`for &x in stats` bunu istiyor)

Buna **trait bound** denir: generic tipe konulan söz. Yarısı kısıt, yarısı garanti —
imzaya bakan kişi fonksiyonun ne yapabileceğini görür.

> **C++ ile temel fark buradadır.** C++ template'inde hata *kullanım yerinde*, sayfalarca
> mesajla çıkar: "no operator> for class Foo" ve arkasından 40 satır instantiation izi.
> Rust'ta hata *imzada*, tek satırda: bound yok. Kütüphane yazarı sözü baştan verir.

## Generic struct

Envanter yuvası: içine ne koyduğunuz sizin seçiminiz.

```rust
struct Slot<T> { label: &'static str, item: T }
```

`Slot<Potion>`, `Slot<Sword>`, `Slot<&str>` — hepsi ayrı tip.

### `impl<T>` ile `impl Slot<Potion>` farkı

```rust
impl<T> Slot<T>          { fn new(...) -> Self }   // BÜTÜN T'ler için
impl    Slot<Potion>     { fn drink(&self)     }   // SADECE iksir yuvasında
impl    Slot<Sword>      { fn swing(&self)     }   // SADECE kılıç yuvasında
impl<T: Debug> Slot<T>   { fn inspect(&self)   }   // sadece Debug olan T'ler için
```

Sonuç:

```rust
iksir.drink()    // çalışır
kilic.swing()    // çalışır
kilic.drink()    // E0599: no method named `drink` found for struct `Slot<Sword>`
```

Kılıcı içemezsiniz — ve bunu **derleyici** engelliyor, çalışma zamanında bir kontrol değil.

Bu "koşullu impl" çok güçlü bir araçtır: *"bu metot yalnızca şu koşulu sağlayan tiplerde
olsun."* std bunu her yerde kullanır — örneğin `Option<T>` üzerindeki `unwrap_or_default`
sadece `T: Default` ise vardır.

## Birden çok generic parametre

Kuşanım: bir elde silah, bir elde zırh.

```rust
struct Loadout<W, A> { weapon: W, armor: A }

impl<W, A> Loadout<W, A> {
    fn swap_hands(self) -> Loadout<A, W> { ... }   // dönüş tipi bile değişiyor
}
```

`Loadout<Sword, &str>` → `swap_hands()` → `Loadout<&str, Sword>`. Tipler derleme
zamanında takip ediliyor.

## `where`

```rust
fn party_report<T>(stats: &[T]) -> String
where
    T: Debug + PartialOrd + Copy,
```

Davranış olarak `fn party_report<T: Debug + PartialOrd + Copy>(...)` ile **birebir aynı**.
Bound sayısı artınca imzayı okunur tutmak için vardır, başka farkı yoktur.

## Const generics

Uzunluk da generic olabilir (Rust 1.51+):

```rust
fn first_member<T: Copy, const N: usize>(party: &[T; N]) -> T { party[0] }
fn party_size<T, const N: usize>(_party: &[T; N]) -> usize { N }
```

`N` bir **değer**, ama derleme zamanında bilinen bir değer. Öncesinde std'de her uzunluk
için ayrı `impl` yazılıyordu — dizilerde 32 elemana kadar elle, sonrası desteksizdi.

## Monomorphization — günün asıl konusu

Derleyici generic kodu çalışma zamanına taşımaz. Her somut tip için **ayrı bir kopya**
üretir:

```
sizin yazdığınız               derleyicinin ürettiği
fn strongest<T>(...)     →     strongest için i32 sürümü
                               strongest için f64 sürümü
                               strongest için char sürümü
```

Bunu gözle görebilirsiniz:

```bash
rustc main.rs && nm -C main | grep strongest
```

```
main::strongest_f64
main::strongest_i32
main::strongest        ← i32
main::strongest        ← f64
main::strongest        ← char
```

Üç ayrı `strongest` — üç ayrı makine kodu. Yani **çalışma zamanında generic diye bir şey
yoktur**; `if x > en` satırı doğrudan `i32` karşılaştırmasına derlenir.

Gün 1'de "zero-cost abstraction" demiştik; borcu burada ödüyoruz. Soyutlama derleme
zamanında var, çalışma zamanında yok.

`-O` ile derlerseniz bu fonksiyonlar tamamen **inline** olur ve `nm` çıktısında hiç
görünmez. O da aynı şeyin başka bir kanıtı: soyutlamanın çalışma zamanında bedeli yok.

### Bedeli ne

Bedava değil, ama bedeli **derleme zamanında** ödüyorsunuz:

- Derleme süresi uzar (her tip için yeniden kod üretimi)
- İkili dosya büyür — C++'ta buna *code bloat* denir
- `serde` gibi ağır generic kullanan crate'ler derleme süresini gözle görülür artırır

### Karşıtı

Tek bir kod üretip tipi çalışma zamanında çözmek de mümkün: **dinamik dispatch**
(`dyn Trait`). O zaman ikili küçülür ama her çağrıda dolaylı bir atlama maliyeti çıkar.
Ders 2'de trait'i tanımlayınca ikisini yan yana koyacağız. Şimdilik bilinmesi gereken:
Rust'ın **varsayılanı monomorphization**, yani statik dispatch.

## Bound'suz generic ne işe yarar

```rust
fn kimlik<T>(x: T) -> T { x }
```

Bu fonksiyon `x` ile hiçbir şey yapamaz: karşılaştıramaz, yazdıramaz, toplayamaz.
Sadece taşıyabilir ve geri verebilir. Kısıt gibi görünüyor ama aslında **garanti**:
imzasına bakan biri, bu fonksiyonun değerinizi bozamayacağını bilir.
