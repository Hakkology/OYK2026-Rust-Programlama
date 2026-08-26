# Gün 5 · Ek Not — Modül Yapısı (genişletilmiş "backyard")

Rust Book'un `backyard` örneğinin büyütülmüş hâli. Amaç: **modül ağacının kurallarını
tek bir projede baştan sona görmek.**

```
backyard/
  Cargo.toml
  src/
    main.rs                    ◄── CRATE KÖKÜ
    garden.rs                      mod garden;
    garden/
      vegetables.rs                pub mod vegetables;
      flowers.rs                   pub mod flowers;
      tools/
        mod.rs                     pub mod tools;        (klasör + mod.rs stili)
        shovel.rs                  pub mod shovel;
```

```bash
cargo run
```

## Derleyici modülleri nasıl buluyor

Sıra hep aynıdır, ezberlenecek dört adım:

**1. Crate kökünden başlar.** İkili crate'te `src/main.rs`, kütüphanede `src/lib.rs`.

**2. Kökte `mod garden;` görürse şu üç yere bakar:**

| Sıra | Nereye bakar |
|---|---|
| 1 | `mod garden { ... }` — süslü parantez varsa içeriği oradadır |
| 2 | `src/garden.rs` |
| 3 | `src/garden/mod.rs` (eski stil) |

**3. Alt modülde de aynı kural işler.** `garden.rs` içinde `pub mod vegetables;` varsa
derleyici `src/garden/vegetables.rs` dosyasına bakar. Yani **klasör yapısı modül ağacını
belirlemez; `mod` satırları belirler, dosyalar sadece içeriği taşır.**

**4. Bir dosya yazıp `mod` satırını unutursanız** o dosya derlemeye hiç girmez. "Kodumu
neden görmüyor" sorusunun cevabı hep budur.

Bu projede iki stil bir arada: `garden` → `garden.rs` + `garden/` klasörü;
`tools` → `garden/tools/mod.rs`. İkisi de geçerlidir, **aynı modül için ikisi birden
kullanılamaz.**

## Yollar

```rust
use crate::garden::vegetables::Asparagus;              // mutlak
use crate::garden::tools::shovel::Shovel as Kurek;     // as ile takma ad
crate::garden::vegetables::Tomato::new("Cherry")       // use yazmadan tam yol
super::vegetables::COUNT                               // bir üst modüle çık (flowers.rs)
self::shovel::Shovel::new()                            // bu modül (tools/mod.rs)
vegetables::COUNT                                      // göreli (garden.rs içinden)
```

`use` işlevsel bir şey yapmaz — sadece uzun yolu kısaltır. Yazmasanız da tam yolla
her şeye erişirsiniz (`main.rs`'te ikisi de var, 1. ve 2. satırlar aynı işi yapıyor).

## Görünürlük seviyeleri, hepsi bu projede

| Yazım | Nerede | Kimden görünür |
|---|---|---|
| (yok) | `soil_check` başlangıçta | sadece kendi modülü ve altı |
| `pub` | `Asparagus`, `Tomato`, `Rose`… | her yerden |
| `pub(crate)` | — | sadece bu crate |
| `pub(super)` | `tools::inventory`, `vegetables::soil_check` | **bir üst modül** — `garden` görür, `main` görmez |

`main.rs`'in sonunda üç satır bilerek yorumlu; hepsi test edildi:

```rust
// garden::vegetables::soil_check();     error[E0603]  pub(super), main göremez
// let _ = Asparagus { height_cm: 5 };   error[E0451]  alan private
// garden::tools::inventory();           error[E0603]  pub(super), main göremez
```

## Struct alanı private, enum varyantı public

Bu ikisi karıştırılır:

```rust
pub struct Asparagus { height_cm: u32 }      // alan YAZILMADIKÇA private
pub struct Tomato { pub variety: &'static str }  // bu alan public

pub enum Color { Red, White, Yellow }        // varyantlar OTOMATİK public
```

`pub struct` demek "tip dışarıdan görünür" demektir, "alanları da görünür" demek değildir.
Enum'da böyle bir ayrım yoktur: enum public ise bütün varyantları public'tir. Sebebi
mantıklı — varyantı gizlenmiş bir enum'la `match` yazılamazdı.

Sonuç: `Asparagus` ancak `Asparagus::new()` ile üretilir; siz geçersiz bir bitki
üretilmesini engellemiş olursunuz.

## `pub use` — dışarısı için kısa yol

`garden.rs` şunu yazıyor:

```rust
pub use vegetables::Asparagus;
```

Bu sayede iki yol da çalışıyor:

```rust
garden::vegetables::Asparagus::new(30)   // gerçek yer
garden::Asparagus::new(12)               // re-export edilmiş kısa yol
```

Kullanıcı iç yapıyı bilmek zorunda kalmaz; yarın `vegetables.rs`'i ikiye bölseniz onun
kodu bozulmaz.

## Çalıştırınca ne görüyorsunuz

```
kuskonmaz  : Asparagus { height_cm: 30 } (30 cm)   use ile kısaltılmış
domates    : Tomato { variety: "Cherry" }          tam yol
gul        : Rose { color: Red }                   enum varyantı
alet       : Shovel { size: "orta" }               as ile takma ad
kisa yol   : Asparagus { height_cm: 12 }           pub use re-export
cesit      : Cherry                                pub alan okunabiliyor
Red White Yellow                                   varyantlar otomatik pub
toprak   : uygun                                   pub(super), garden çağırdı
bahce    : 2 sebze, 1 cicek
alet     : Shovel { size: "orta" }                 tools::inventory
```

Her satır yukarıdaki kurallardan birinin karşılığı — sırayla okutmak, modül anlatımını
tek ekranda toparlar.
