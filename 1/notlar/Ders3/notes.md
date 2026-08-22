# Gün 1 · Ders 3 — Cargo ve Ekosistem

## Açılış

`main.rs`'i önce `rustc` ile, sonra bir Cargo projesi içinde çalıştırın. Çıktı değişiyor.
Cargo'nun ne yaptığını görmenin en hızlı yolu bu.

## Komutlar

```
cargo new <ad>          yeni binary
cargo new <ad> --lib    yeni kütüphane
cargo check             SADECE tip denetimi, en hızlısı — gün boyu bu
cargo build             derle
cargo build --release   optimize derle
cargo run               derle + çalıştır
cargo run -- a b        argüman geçir
cargo test              testler
cargo fmt               biçimlendir
cargo clippy            iyileştirme önerileri
cargo add <crate>       bağımlılık ekle
cargo doc --open        doküman üret + aç
cargo tree              bağımlılık ağacı
```

- Günlük döngü: yaz → `cargo check` → yaz → `cargo check`
- Her seferinde `cargo build` çalıştırmak gereksiz

## Cargo.toml ve Cargo.lock

- `Cargo.toml` sizin yazdığınız → ne istediğinizi söyler
- `Cargo.lock` Cargo'nun yazdığı → ne kurulduğunu kaydeder
- `[dependencies]` vs `[dev-dependencies]`
- `license = "MIT OR Apache-2.0"`
- Lock dosyası git'e girer mi: binary → evet, kütüphane → hayır

## Semver

```
1.4.2 → MAJOR.MINOR.PATCH
"1.4"    1.4.0 <= x < 2.0.0     en yaygın
"~1.4.2" 1.4.2 <= x < 1.5.0     sadece patch
"=1.4.2" tam                     sabitle
"*"      her şey                 yapmayın
```

- 0.x özel: 0.4 → 0.5 kıran değişiklik sayılır

## Profiller

| | dev | release |
|---|---|---|
| opt-level | 0 | 3 |
| debug_assertions | açık | kapalı |
| **tamsayı taşması** | **panic** | **sessizce sarar** |
| derleme / çalışma | hızlı / yavaş | yavaş / hızlı |

- Taşma satırı kritik: aynı kod dev'de panikler, release'te sessizce sarar
- Ölçüm daima `--release` ile yapılır

## Bağımlılık seçmek

- Son güncelleme, indirme sayısı, lisans, `cargo tree` derinliği, docs.rs'te örnek var mı
- Bir bağımlılık eklerken lisansını da projenize alıyorsunuz

## Dokümantasyon

- doc.rust-lang.org/std · docs.rs/\<crate\> · `cargo doc --open`
- Bir tipe bakarken sıra: açıklama → Implementations → **Trait Implementations** → `[src]`
- Doküman örnekleri test olarak çalıştırılır, yani derleniyor demektir

## Uygulama

Her adımı kendi makinenizde deneyin.

1. `main.rs`'i iki şekilde çalıştırın, farkı görün
2. `cargo new deneme && cargo add rand` → Cargo.toml ne oldu, `cargo tree` kaç satır
3. `cargo doc --open` → `rand::random()` fonksiyonunu bulun
4. `cargo clippy` → uyarı var mı
5. `cargo build` vs `--release` → süre ve ikili dosya boyutu farkı
6. `Cargo.toml`'a lisans satırı ekleyin
