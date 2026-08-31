# Gün 5 · Ders 3 — Modüller, Görünürlük ve Proje Organizasyonu

## Üç seviye: paket, crate, modül

| | Nedir | Sınırı ne belirler |
|---|---|---|
| **Paket** (package) | `cargo new` ile oluşan proje | `Cargo.toml` |
| **Crate** | tek seferde derlenen birim | `main.rs` ya da `lib.rs` |
| **Modül** (`mod`) | crate içinde isim alanı ve görünürlük sınırı | `mod` satırları |

Sıralama şöyle: **paket bir veya daha çok crate barındırır, crate modüllerden oluşur.**

Bir paket **en fazla bir kütüphane crate'i** barındırabilir, ama istediği kadar ikili
crate barındırabilir. Cargo bunları dosya adına bakarak anlar:

| Dosya | Ne olur |
|---|---|
| `src/main.rs` | ikili crate kökü, paket adıyla aynı isimde program üretir |
| `src/lib.rs` | kütüphane crate kökü |
| `src/bin/rapor.rs` | ikinci bir ikili crate — `cargo run --bin rapor` |

### İkili crate mi, kütüphane crate mi?

- **İkili (binary)** çalıştırılabilir program üretir. `fn main()` ister. Başkası
  `use` ile içindekilere erişemez.
- **Kütüphane (library)** çalıştırılamaz; başka crate'ler tarafından **kullanılmak**
  için vardır. `fn main()` yoktur.

```
cargo new deneme          -> src/main.rs  (ikili)
cargo new deneme --lib    -> src/lib.rs   (kütüphane)
```

Kütüphane yapmanın üç somut kazancı var, üçü de teknik:

1. **Entegrasyon testi yazabilirsiniz.** `tests/` dizini sadece kütüphane crate'ini
   görür; `main.rs`'e gömülü kod oradan erişilemez.
2. **Doküman testleri çalışır.** `///` içindeki örnekler yalnızca kütüphanede test edilir.
3. **Başkası kullanabilir.** Başka bir ikili, başka bir paket ya da gelecekteki siz.

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

## Yollar — başka modüldeki bir şeye nasıl erişilir

Dört yazım var; dördü de bu dersin projesinde gerçekten kullanılıyor:

| Yazım | Anlamı | Projede nerede |
|---|---|---|
| `crate::...` | **mutlak** — crate kökünden başlar | `report/summary.rs` → `use crate::telemetry::Reading;` |
| `super::...` | **bir üst** modül | `telemetry/parser.rs` → `use super::validation;` |
| `self::...` | bu modül (genelde yazılmaz, örtüktür) | `telemetry/mod.rs` → `pub use parser::parse;` |
| `isim::...` | **göreli** — bu modülün altındaki | `parser.rs` içinde `validation::in_range(...)` |

Aynı fonksiyona üç ayrı yoldan gidilebilir:

```rust
crate::telemetry::validation::in_range(x)   // mutlak — nereden yazarsanız yazın çalışır
super::validation::in_range(x)              // telemetry::parser içinden
validation::in_range(x)                     // telemetry içinden, göreli
```

**Hangisi ne zaman?** Kardeş bir modüle gidiyorsanız `crate::` en okunaklısıdır: dosya
yer değiştirse bile yol aynı kalır. Bir adım yukarı çıkıyorsanız `super::` kısadır.
Kendi altınızdaki bir şeye göreli isim yeter.

`use` bunları kısaltır: `use crate::telemetry::Reading;` yazdıktan sonra sadece
`Reading` dersiniz. `use` bir "kopyalama" değil, ismi bu kapsama getirmedir.

### Görünürlük tek yönlüdür

Alt modül üstü **her zaman** görür — `pub` olmasa bile:

```rust
// telemetry/validation.rs
pub fn calibrated_upper() -> f64 {
    super::calibrate(UPPER)      // üst modülün fonksiyonu
}
```

Tersi doğru değildir: üst modül, alt modülün yalnızca `pub` olanını görür. Bu yüzden
`parser.rs` içindeki private bir yardımcıya `telemetry/mod.rs`'ten erişemezsiniz.

## `mod` ile `use` aynı şey değil

En çok karıştırılan ikili budur:

| | Ne yapar |
|---|---|
| `mod telemetri;` | modülü **var eder** — dosyanın içeriğini crate'e dâhil eder |
| `use telemetri::Reading;` | var olan bir şeyin **kısa adını** bu kapsama getirir |

`mod` bir kere yazılır (crate kökünde ya da üst modülde). `use` istediğiniz kadar
dosyada, sadece yazım kolaylığı için yazılır. `use` yazmasanız da uzun yolla
(`crate::telemetri::Reading`) her şeye erişebilirsiniz; `mod` yazmazsanız dosya
**derlemeye hiç girmez** — kimse onu bulamaz.

> Bir dosya oluşturup kod yazdınız, "neden görünmüyor" diyorsanız: `mod` satırını
> unutmuşsunuzdur. Rust klasörü tarayıp dosyaları kendiliğinden dâhil etmez;
> modül ağacını **siz** bildirirsiniz.

## Dosya düzeni

Modül ağacı ile klasör ağacı birbirine benzer ama **klasör yapısı değil, `mod`
satırları belirler**:

```
src/
  lib.rs                    mod telemetri;              -> crate::telemetri
  telemetri.rs              pub mod dogrulama;          -> crate::telemetri::dogrulama
  telemetri/
    dogrulama.rs
  main.rs                   ayrı crate: use paket_adi::...;
  bin/
    rapor.rs                ayrı bir ikili
tests/
  entegrasyon.rs            sadece kütüphaneyi görür
```

Bir modülü tanımlamanın üç yolu vardır, üçü de aynı ağacı üretir:

```rust
mod telemetri { ... }        // 1) satır içi, aynı dosyada
mod telemetri;               // 2) telemetri.rs        (yeni ve tercih edilen)
mod telemetri;               // 3) telemetri/mod.rs    (eski, hâlâ geçerli)
```

2 ve 3 aynı anda kullanılamaz — Cargo hangisini okuyacağını bilemez ve hata verir.
Yeni projelerde `telemetri.rs` + `telemetri/` tercih edilir; editörde on tane `mod.rs`
sekmesi açık kalmaz.

## En pratik tavsiye: `lib.rs` + `main.rs`

- `src/lib.rs` → **mantık burada**. Test edilebilir, belgelenebilir, tekrar kullanılabilir.
- `src/main.rs` → **ince kabuk**: argümanları okur, kütüphaneyi çağırır, çıktıyı basar,
  hata olursa çıkış kodunu ayarlar.

Bu ikisi **ayrı crate'lerdir** — aynı paketin içinde olsalar bile. `main.rs`,
kütüphaneye tıpkı dışarıdan bir kullanıcı gibi, **paket adıyla** erişir:

```rust
// Cargo.toml -> name = "ornek_proje"
use ornek_proje::parse;      // crate:: değil, paket adı
```

Bu ayrımın somut sonucu şu: `main.rs` içinden kütüphanenin **private** kısımlarına
erişemezsiniz. Yani `pub` işaretlemesi kendi programınıza karşı da geçerlidir; API'nizi
ilk kullanan kişi sizsiniz.

Bir kütüphanede iş mantığını topladığınızda kazandıklarınız:

```
cargo test          unit + entegrasyon + doküman testleri hepsi çalışır
cargo doc --open    kütüphanenin dokümanı üretilir
cargo run           ince kabuk yine çalışır
```

Her şeyi `main.rs`'e yazan proje bunların ikisini kaybeder.

### Ne kütüphaneye, ne kabuğa?

| Kütüphaneye (`lib.rs`) | Kabuğa (`main.rs`) |
|---|---|
| ayrıştırma, doğrulama, hesap | komut satırı argümanları |
| tipler ve hata tipleri | ekrana yazma, biçimlendirme |
| iş kuralları | çıkış kodu, kullanıcıya mesaj |

Ölçüt basit: **testini yazmak isteyeceğiniz her şey kütüphaneye gider.**

## Bir projeyi üç aşamada büyütmek

Gerçekte kimse boş sayfaya modül ağacı çizmez; proje büyüdükçe bölünür. Sıra şudur:

**1. Aşama — tek dosya.** Her şey `main.rs` içinde. Yüz satıra kadar tamamen doğru
tercih; erken bölmek sadece gürültü çıkarır.

**2. Aşama — modüllere bölme.** Dosya büyüyünce ilgili şeyler `mod` altında gruplanır.
Önce aynı dosyada satır içi `mod`, sonra ayrı dosyalara taşınır:

```rust
// main.rs
mod telemetri;      // artık telemetri.rs dosyasında
mod rapor;
```

Bu aşamada hâlâ tek crate var, `tests/` hâlâ yazılamaz.

**3. Aşama — kütüphaneye ayırma.** İş mantığı `lib.rs`'e taşınır, `main.rs` ince kabuk olur:

```
src/lib.rs      mod telemetry;  pub use telemetry::parse;
src/main.rs     use paket_adi::parse;
tests/          artık yazılabilir
```

Bu aşama Cargo gerektirir (`rustc` tek başına `lib.rs` + `tests/` düzenini kurmaz).
**Bu dersin projesi 3. aşamanın çalışan hâlidir.**

Ne zaman geçilir: **test yazmak istediğinizde ya da ikinci bir giriş noktası
gerektiğinde.** İkisi de olmuyorsa 2. aşamada kalmak sorun değil.

## Bu proje nasıl kurulur

```bash
cargo new ders3 --lib      # kütüphane crate'i: src/lib.rs üretir
cd ders3
```

`--lib` demezseniz Cargo `src/main.rs` üretir, yani ikili crate kurar. İkisi birden
gerekiyorsa ikinci dosyayı **elle** eklersiniz:

```bash
cargo new ders3 --lib                       # src/lib.rs geldi
touch src/main.rs                           # ikili crate'i elle ekledik
mkdir -p src/telemetry src/report src/bin tests
```

| Ne istiyorsunuz | Nasıl |
|---|---|
| kütüphane | `cargo new ad --lib` → `src/lib.rs` |
| çalıştırılabilir program | `cargo new ad` → `src/main.rs` |
| ikisi birden | önce biri, sonra öbür dosyayı elle ekleyin |
| ikinci, üçüncü program | `src/bin/report_cli.rs`, `src/bin/check.rs` … |

Cargo klasörleri tarayıp modül bulmaz: dosyaları siz açar, `mod` satırlarını siz
yazarsınız. Tek istisna `src/bin/` — oradaki her `.rs` kendiliğinden ayrı bir ikili olur.

## Bu dersin projesi

Ders 3'ün kodu tam bir proje: **iki alan (`telemetry`, `report`), her biri kendi
klasöründe, kendi alt modülleri ve kendi testleriyle.**

```
Ders3/
  Cargo.toml
  src/
    lib.rs                    KÜTÜPHANE KÖKÜ — mod telemetry; mod report; pub use ...
    telemetry/
      mod.rs                  modül kökü — pub mod parser; pub mod validation;
      parser.rs               Reading, parse()      + 3 test
      validation.rs           in_range(), sınırlar  + 3 test
    report/
      mod.rs                  modül kökü — pub mod summary; pub mod table;
      summary.rs              summary()             + 2 test
      table.rs                table()               + 2 test
    main.rs                   İNCE KABUK — use ders3::...
    bin/
      report_cli.rs           İKİNCİ İKİLİ — cargo run --bin report_cli
  tests/
    integration.rs            sadece public API     + 3 test
```

```bash
cargo run                  # src/main.rs
cargo run --bin report_cli # src/bin/report_cli.rs — raporu çalıştırır
cargo test                 # 10 unit + 3 entegrasyon + 1 doküman testi
cargo doc --open
```

İki ikili olduğu için düz `cargo run` hangisini çalıştıracağını bilemez ve
*"could not determine which binary to run"* der. `Cargo.toml`'daki tek satır çözer:

```toml
default-run = "ders3"
```

`src/bin/report_cli.rs` kütüphaneyi **dışarıdan** kullanır (`use ders3::...`), tıpkı
`main.rs` gibi. Aynı paketteki iki program ortak kütüphaneyi paylaşıyor; tek satır kod
kopyalanmıyor.

### İkili, `report` modülünü nasıl kullanıyor

Projede iki program var ve **ikisi de aynı kütüphaneyi** kullanıyor, ama farklı
yollardan — bu, `pub mod` ile `pub use` arasındaki farkı gösteriyor:

```rust
// src/bin/report_cli.rs  — MODÜL YOLUYLA
use ders3::report::{summary, table};
use ders3::telemetry::{parse, Reading};

// src/main.rs  — KISA YOLLA (lib.rs'teki pub use sayesinde)
use ders3::{in_range, parse, summary, table, Reading};
```

İkisi de aynı fonksiyonları çağırıyor. Farkı `lib.rs` belirliyor:

```rust
pub mod report;                    // modülü dışarı AÇAR -> ders3::report::summary
pub use report::{summary, table};  // aynı şeye KISA ad   -> ders3::summary
```

`pub mod` yazmasaydık `report_cli.rs` derlenmezdi:

```
error[E0603]: module `report` is private
```

Yani ikilinin modüle erişebilmesinin tek sebebi, `lib.rs`'te o modülü açmış olmanız.
**Kütüphanenizin kapısını siz açarsınız; ikili de dışarıdan gelen herkes gibi o kapıdan girer.**

### Açtığınız modülün içindeki her şey açık değildir

`report/summary.rs` içinde `pub(crate)` bir yardımcı var:

```rust
pub(crate) fn internal_label() -> &'static str { "TELEMETRI" }
```

`summary()` bunu kullanıyor (çıktıdaki `[TELEMETRI]` etiketi oradan geliyor), ama ikili
çağıramıyor:

```
error[E0603]: function `internal_label` is private
```

Modül açık, öğe kapalı. Görünürlük **yolun her adımında** ayrı ayrı sorulur:
`ders3` → `report` → `summary` → `internal_label`. Zincirde bir halka kapalıysa erişim yok.

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
