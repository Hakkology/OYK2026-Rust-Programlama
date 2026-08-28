# Gün 9 · Ders 1 — Test, Clippy ve Cargo

Bugün proje yazacaksınız. Bu ders yeni bir dil özelliği anlatmıyor; **projeyi ayakta
tutan aletleri** anlatıyor. Hepsi `rustup` ile birlikte zaten kurulu.

Bu dersin klasörü çalışan bir crate: `9/notlar/Ders1/`. Aşağıdaki her çıktı oradan
alındı, kendiniz de çalıştırabilirsiniz.

## Dört tür test

```
cargo test
```

```
running 4 tests
test tests::cok_kalemli_fis ... ignored, yavas: elle calistirin -> cargo test -- --ignored
test tests::bos_fis_sifir ... ok
test tests::kalemler_toplaniyor ... ok
test tests::sifira_bolunmez - should panic ... ok

     Running tests/integration.rs
test disaridan_kullanim ... ok
test varsayilan_bos ... ok

   Doc-tests ders1
test src/lib.rs - Bill::split (line 58) ... ok
test src/lib.rs - Bill::add (line 30) ... ok
test src/lib.rs - (line 5) ... ok
```

Tek komut üç ayrı yerde test çalıştırdı:

| tür | nerede | ne görür |
|---|---|---|
| birim testi | `src/` içinde `#[cfg(test)] mod tests` | **private** dahil her şeyi |
| entegrasyon | `tests/*.rs` | yalnızca `pub` olanları — dışarıdan bakar |
| doc testi | `///` içindeki ` ``` ` blokları | dokümantasyonun **doğru** olduğunu |

`#[cfg(test)]` önemli: test modülü yalnızca `cargo test` sırasında derlenir, yayınladığınız
ikiliye girmez.

Doc testleri Rust'ın en sevilen özelliklerinden biri: **dokümanınızdaki örnek yanlışsa
testiniz kırmızı yanar.** Belge çürümesi diye bir şey kalmıyor.

## `assert!` mi `assert_eq!` mi

İkisi de aynı hatayı yakalar, ama biri size ne olduğunu söyler:

```
---- bool_ile ----
assertion failed: topla(2, 3) == 5

---- esitlik_ile ----
assertion `left == right` failed
  left: 6
 right: 5
```

**Kural: karşılaştırıyorsanız `assert_eq!` / `assert_ne!` kullanın.** `assert!` yalnızca
gerçekten bir `bool` koşulu içindir.

## `#[should_panic]`

```rust
#[test]
#[should_panic(expected = "kisi sayisi sifir olamaz")]
fn sifira_bolunmez() {
    Bill::new().split(0);
}
```

`expected` olmadan da yazabilirsiniz — ama yazmayın. `expected` panik **mesajını** da
kontrol eder; yoksa yanlış sebeple panikleyen bir test yanlışlıkla geçer.

## `#[ignore]` ve test seçme

Yavaş testleri işaretleyin:

```rust
#[test]
#[ignore = "yavas: elle calistirin"]
fn cok_kalemli_fis() { ... }
```

```
cargo test kalem            # adında "kalem" geçen testler
   1 passed; 1 ignored; 2 filtered out

cargo test -- --ignored     # SADECE ignore edilenler
   1 passed; 3 filtered out

cargo test -- --nocapture   # println! çıktısını göster
```

Varsayılan olarak testler **paralel** çalışır ve geçen testlerin `println!` çıktısı
yutulur. `--nocapture` bunu açar, `--test-threads=1` sırayı zorlar.

## Feature'lar — isteğe bağlı parçalar

```toml
[features]
default = []
tips = []
```

```rust
#[cfg(feature = "tips")]
pub fn with_tip(total: u32, yuzde: u32) -> u32 { ... }
```

```
cargo test                    # 3 test
cargo test --features tips    # 4 test  (bahsis_ekleniyor eklendi)
cargo test --all-features
```

Ne işe yarar: kullanıcı istemediği parçayı **derlemez bile**. `serde`, `tokio` gibi büyük
crate'ler bu yüzden onlarca feature taşır (`tokio = { version = "1", features = ["full"] }`
yazarken yaptığınız şey budur).

## Profiller — `dev` ve `release`

```toml
[profile.dev]     opt-level = 0    # hızlı derleme, yavaş kod
[profile.release] opt-level = 3    # yavaş derleme, hızlı kod
```

```
cargo run              -> [dev profili] debug_assertions ACIK
cargo run --release    -> [release profili] debug_assertions KAPALI
```

İki sonucu var:

- **Ölçüm yaparken `--release` şart.** Gün 8'de thread ölçümlerini `dev`'de yaptık; release'te tablo değişir.
- `debug_assertions` release'te kapanır: taşma kontrolü (`overflow`) dev'de panikler, release'te sarmalar. Bu bilinçli bir tasarım kararıdır, sürpriz değil.

## Clippy — derleyicinin söylemediklerini söyler

`rustc` "bu kod **geçerli mi**" diye bakar; `clippy` "bu kod **iyi mi**" diye.

```
cargo clippy --all-targets
```

Kasten kötü yazılmış bir dosyada gerçek çıktı:

```
warning: unneeded `return` statement                      [clippy::needless_return]
warning: length comparison to zero                        [clippy::len_zero]
warning: equality checks against true are unnecessary     [clippy::bool_comparison]
warning: match can be simplified with `.unwrap_or_default()`  [clippy::manual_unwrap_or_default]
warning: useless use of `vec!`                            [clippy::useless_vec]
```

Kampta öğrendiklerimizle birebir örtüşen bir tanesi:

```rust
fn uzunluk(ad: &String) -> usize { ad.len() }
```

```
warning: writing `&String` instead of `&str` involves a new object where a slice will do
help: change this to: `&str`                              [clippy::ptr_arg]
```

Gün 3'te "parametrede `&str` al" demiştik, Gün 7'de sebebini (deref coercion) öğrendik —
clippy aynı şeyi size hatırlatıyor.

Faydalı komutlar:

```
cargo clippy --fix              # düzeltilebilenleri otomatik düzeltir
cargo clippy -- -D warnings     # CI'da: uyarı varsa build kırılsın
#![allow(clippy::needless_range_loop)]   # tek bir lint'i kapatmak
```

## `cargo fmt` — tartışmayı bitirir

```rust
fn main(){let x=vec![1,2,3];for i in &x{println!("{}",i);}}
```

`cargo fmt` sonrası:

```rust
fn main() {
    let x = vec![1, 2, 3];
    for i in &x {
        println!("{}", i);
    }
}
```

Tek bir resmî stil var; girinti tartışması Rust'ta yapılmaz. `cargo fmt --check`
değiştirmeden sadece kontrol eder — CI'da bunu kullanın.

## Günlük komut listesi

| komut | ne yapar |
|---|---|
| `cargo check` | derler ama ikili üretmez — **en hızlı geri bildirim** |
| `cargo test` | üç tür testi birden çalıştırır |
| `cargo clippy --all-targets` | testler dahil bütün hedefleri denetler |
| `cargo fmt` | biçimlendirir |
| `cargo doc --open` | dokümantasyonu üretip tarayıcıda açar |
| `cargo build --release` | dağıtılacak sürüm |
| `cargo add <crate>` | `Cargo.toml`'a bağımlılık ekler |
| `cargo tree` | bağımlılık ağacı — "bu crate nereden geldi?" |

Çalışma sırası: `check` → `test` → `clippy` → `fmt`. İlk üçü yeşilse commit edin.

## Workspace — birden çok crate

Gün 5'teki proc-macro örneğinde kullanmıştık:

```toml
[workspace]
members = ["kutuphane", "cli"]
```

Tek `target/` klasörü, tek `Cargo.lock`, tek `cargo test` bütün üyeleri çalıştırır.
Proje büyüdüğünde ilk yapacağınız şey budur.

## Sürüm numaraları

`Cargo.toml`'da `serde = "1.0"` yazmak `^1.0` demektir: 1.x'in her sürümü kabul, 2.0 değil.
Semver sözü: **majör değişmediyse kodunuz kırılmaz.** `Cargo.lock` tam olarak hangi
sürümün kullanıldığını sabitler — ikili projelerde commit'leyin, kütüphanelerde etmeyin.
