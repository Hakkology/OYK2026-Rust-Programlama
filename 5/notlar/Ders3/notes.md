# Gün 5 · Ders 3 — Modüller, Görünürlük ve Proje Organizasyonu

## Üç seviye: paket, crate, modül

| | Nedir |
|---|---|
| **Paket** (package) | `cargo new` ile oluşan şey. `Cargo.toml` ile yönetilir. |
| **Crate** | Derleme birimi. İkili (`main.rs`) ya da kütüphane (`lib.rs`). |
| **Modül** (`mod`) | Crate içinde isim alanı ve görünürlük sınırı. |

Bir paket **en fazla bir kütüphane crate'i**, istediği kadar ikili crate barındırır
(`src/bin/` altında). En yaygın kurulum: bir `lib.rs` + bir `main.rs`.

## Varsayılan private

Rust'ta **her şey private**. Modül ağacında görünürlük tek yönlüdür:

> İç modül dış modülün her şeyini görür; dış modül iç modülün sadece `pub` olanını görür.

Bu iyi bir varsayılan: API yüzeyiniz kaza eseri büyümez. Dışarı ne verdiğinize
bilinçli karar verirsiniz.

## Görünürlük seviyeleri

| Yazım | Nereden görünür |
|---|---|
| (yok) | sadece bu modül ve altı |
| `pub` | her yerden |
| `pub(crate)` | sadece bu crate içinde |
| `pub(super)` | sadece bir üst modülden |
| `pub(self)` | (yok) ile aynı, açıkça yazılmış hâli |

`pub(crate)` en çok kullanılanıdır: "projemin her yerinde lazım ama dışarıya açmıyorum".

**Dikkat:** modülü `pub` yapmak içindekileri public yapmaz. Her öğe ayrı ayrı işaretlenir.
Aynı şekilde `pub struct` demek alanları public yapmaz:

```rust
pub struct Reading { deger: f64 }   // tip public, alan private
```

Bu bilinçli bir tasarımdır: dışarıdan `Reading` oluşturulamaz, sadece sizin verdiğiniz
kurucu fonksiyonla oluşturulur — böylece geçersiz bir `Reading` üretilemez. Gün 4'ün
"geçersiz durumu temsil edilemez kıl" cümlesinin modül seviyesindeki karşılığı.

## Yollar

```rust
crate::telemetri::Reading    // mutlak — crate kökünden
self::dogrulama::kontrol     // bu modülden
super::Reading               // bir üst modülden
telemetri::Reading           // göreli
```

`use` bunları kısaltır: `use crate::telemetri::Reading;` yazdıktan sonra sadece
`Reading` dersiniz. `use` bir "kopyalama" değil, sadece ismi bu kapsama getirmedir.

## Dosya düzeni

```
src/
  main.rs              ikili crate kökü
  lib.rs               kütüphane crate kökü
  telemetri.rs         -> lib.rs içinde:      mod telemetri;
  telemetri/
    dogrulama.rs       -> telemetri.rs içinde: pub mod dogrulama;
```

`mod telemetri;` satırı "bu modülün içeriği `telemetri.rs` dosyasında" demektir.
Eski yöntem `telemetri/mod.rs` da hâlâ geçerlidir; ikisi karışmaz. Yeni projelerde
`telemetri.rs` + `telemetri/` tercih edilir, çünkü editörde on tane `mod.rs` sekmesi
açık kalmaz.

## En pratik tavsiye: `lib.rs` + `main.rs`

- `src/lib.rs` → **mantık burada**. Test edilebilir, başkası kullanabilir.
- `src/main.rs` → ince kabuk: argümanları alır, kütüphaneyi çağırır, çıktıyı basar.

Sebebi teknik: `tests/` dizinindeki entegrasyon testleri **sadece kütüphane crate'ini
görür**. Her şeyi `main.rs`'e yazan proje entegrasyon testi yazamaz.

`main.rs` kütüphaneyi paket adıyla kullanır:

```rust
use benim_paketim::telemetri::parse;
```

## `pub use` ile re-export

İçeride derin, dışarıda düz:

```rust
// lib.rs
mod telemetri;
pub use telemetri::dogrulama::kontrol;   // artık crate::kontrol
```

Kullanıcı sizin klasör yapınızı bilmek zorunda değil. **API yüzeyini siz tasarlarsınız**;
iç düzeni sonra değiştirseniz bile dışarıdaki isim sabit kalır.

## Doküman yorumları

- `///` → hemen altındaki öğeyi belgeler
- `//!` → içinde bulunduğu modülü/crate'i belgeler (dosyanın en üstünde)
- `cargo doc --open` ile HTML üretir
- İçindeki örnekler **test olarak çalıştırılır** — yani dokümanınız çürümez

## Dış bağımlılıklar

```
cargo add rand          Cargo.toml'a ekler
cargo tree              bağımlılık ağacını gösterir
cargo update            uyumlu en yeni sürümlere çeker
```

Bağımlılık eklemek bedava değildir: derleme süresi, ikili boyut, lisans ve güvenlik
yüzeyi. Gün 1'deki ölçüt burada da geçerli — son güncelleme, indirme sayısı,
`cargo tree` derinliği.

## Workspace

```toml
[workspace]
members = ["core", "cli", "web"]
```

Ortak `Cargo.lock`, ortak `target/`, tek `cargo build`. Birden çok crate'i birlikte
geliştiren projelerde standart düzendir.
