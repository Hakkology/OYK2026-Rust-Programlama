# Gün 6 · Ders 1 (v1) — Generics ve Monomorphization

> Bu, Ders 1'in **ikinci anlatımıdır**. Konular birebir aynı; dünya farklı.
> Oyun dünyası yerine **şehir ulaşım ağı**: vapur, metro, tramvay.
> İkisinden hangisi size daha iyi oturuyorsa onu okuyun.

## Problem

Aynı fonksiyonu iki kere yazdık, tek fark tip:

```rust
fn longest_i32(sureler: &[i32]) -> i32 { ... }
fn longest_f64(sureler: &[f64]) -> f64 { ... }
```

Gövdeleri harfi harfine aynı. Üçüncü bir tip gelince üçüncü kopyayı mı yazacaksınız?

## Generic — ama tek başına yetmez

```rust
fn longest_bozuk<T>(sureler: &[T]) -> T {
    let mut en = sureler[0];
    for &x in sureler {
        if x > en { en = x; }
    }
    en
}
```

```
error[E0369]: binary operation `>` cannot be applied to type `T`
```

Derleyici `T`'nin ne olduğunu bilmiyor, dolayısıyla **ne yapabileceğini** de bilmiyor.
`>` her tipte tanımlı değil. Çözüm, `T`'den ne beklediğinizi söylemek:

```rust
fn longest<T: PartialOrd + Copy>(sureler: &[T]) -> T
```

- `PartialOrd` → "karşılaştırılabilir olacak"
- `Copy` → "kopyalanabilir olacak" (`for &x in sureler` bunu istiyor)

Buna **trait bound** denir: yarısı kısıt, yarısı garanti. İmzaya bakan kişi fonksiyonun
ne yapabileceğini görür.

```
25 dk / 12 TL
hat harflerinde bile: F
```

> **C++ ile temel fark burada.** C++ template'inde hata *kullanım yerinde*, sayfalarca
> mesajla çıkar. Rust'ta hata *imzada*, tek satırda: bound yok.

## Generic struct

Peron: içine hangi aracı koyduğunuz sizin seçiminiz.

```rust
struct Platform<T> { label: String, vehicle: T }
```

`Platform<Ferry>`, `Platform<Metro>`, `Platform<&str>` — hepsi ayrı tip.

### `impl<T>` ile `impl Platform<Ferry>` farkı

```rust
impl<T> Platform<T>        { fn new(...) -> Self }       // BÜTÜN T'ler için
impl    Platform<Ferry>    { fn lower_ramp(&self) }      // SADECE vapur peronunda
impl    Platform<Metro>    { fn open_doors(&self) }      // SADECE metro peronunda
impl<T: Debug> Platform<T> { fn inspect(&self) }         // sadece Debug olan T'ler
```

```
Konak: rampa indi, 2 guverte
Halkapinar: 5 vagonun kapilari acildi
```

```rust
metro_platform.lower_ramp();
```

```
error[E0599]: no method named `lower_ramp` found for struct `Platform<Metro>`
```

Metro peronuna rampa indiremezsiniz — ve bunu **derleyici** engelliyor, çalışma zamanında
bir kontrol değil.

Bu "koşullu impl" çok güçlü bir araçtır: *"bu metot yalnızca şu koşulu sağlayan tiplerde
olsun."* std bunu her yerde kullanır — `Option<T>` üzerindeki `unwrap_or_default` sadece
`T: Default` ise vardır.

## Birden çok generic parametre

Aktarma: iki ayrı hat.

```rust
struct Transfer<A, B> { first: A, second: B }

fn reverse(self) -> Transfer<B, A>
```

```
Transfer { first: Ferry {...}, second: Metro {...} }
ters: Transfer { first: Metro {...}, second: Ferry {...} }
```

`Transfer<Ferry, Metro>` → `reverse()` → `Transfer<Metro, Ferry>`. Tipler derleme
zamanında takip ediliyor.

## `where`

```rust
fn report<T>(items: &[T]) -> String
where
    T: Debug + Clone,
```

Bound sayısı artınca imzayı okunur tutmak için vardır, başka farkı yoktur.

## Const generics

Uzunluk da generic olabilir (Rust 1.51+):

```rust
struct Fleet<const N: usize> { plates: [u32; N] }
```

```
3 araclik filo, plaka toplami 153
5 araclik filo, plaka toplami 150
```

`N` bir **değer**, ama derleme zamanında bilinen bir değer. `Fleet<3>` ile `Fleet<5>`
ayrı tiplerdir. Öncesinde std'de her uzunluk için ayrı `impl` yazılıyordu — dizilerde
32 elemana kadar elle, sonrası desteksizdi.

## Monomorphization — günün asıl konusu

Derleyici generic kodu çalışma zamanına taşımaz. Her somut tip için **ayrı bir kopya**
üretir:

```
sizin yazdığınız              derleyicinin ürettiği
fn longest<T>(...)      →     longest için i32 sürümü
                              longest için f64 sürümü
                              longest için char sürümü
```

Gözle görebilirsiniz:

```bash
rustc main_v1.rs && nm -C main_v1 | grep longest
```

```
main_v1::longest_f64
main_v1::longest_i32
main_v1::longest   (i32)
main_v1::longest   (f64)
main_v1::longest   (char)
```

Üç ayrı `longest` — üç ayrı makine kodu. Yani **çalışma zamanında generic diye bir şey
yoktur**; `if x > en` satırı doğrudan `i32` karşılaştırmasına derlenir.

Gün 1'de "zero-cost abstraction" demiştik; borcu burada ödüyoruz. `-O` ile derlerseniz
bu fonksiyonlar tamamen **inline** olur ve `nm` çıktısında hiç görünmez.

### Bedeli ne

Bedava değil, ama bedeli **derleme zamanında** ödüyorsunuz:

- Derleme süresi uzar (her tip için yeniden kod üretimi)
- İkili dosya büyür — C++'ta buna *code bloat* denir
- `serde` gibi ağır generic kullanan crate'ler derleme süresini gözle görülür artırır

### Karşıtı

Tek bir kod üretip tipi çalışma zamanında çözmek de mümkün: **dinamik dispatch**
(`dyn Trait`). Ders 2'de trait'i tanımlayınca ikisini yan yana koyacağız. Şimdilik
bilinmesi gereken: Rust'ın **varsayılanı monomorphization**, yani statik dispatch.

## Bound'suz generic ne işe yarar

```rust
fn identity<T>(x: T) -> T { x }
```

Bu fonksiyon `x` ile hiçbir şey yapamaz: karşılaştıramaz, yazdıramaz, toplayamaz.
Sadece taşıyabilir ve geri verebilir. Kısıt gibi görünüyor ama aslında **garanti**:
imzasına bakan biri, bu fonksiyonun değerinizi bozamayacağını bilir.
