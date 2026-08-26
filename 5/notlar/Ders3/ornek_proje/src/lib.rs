//! Gun 5 / Ders 3 - cok dosyali proje ornegi.
//! `//!` icinde bulundugu seyi (burada crate'i) belgeler.
//!
//! Duzen:
//!   src/lib.rs                 crate koku - mantik burada
//!   src/telemetri.rs           mod telemetri
//!   src/telemetri/dogrulama.rs mod telemetri::dogrulama
//!   src/main.rs                ince kabuk, kutuphaneyi kullanir
//!   tests/entegrasyon.rs       sadece PUBLIC API'yi gorur

mod telemetri;                       // icerigi src/telemetri.rs dosyasinda

// pub use ile re-export: kullanici ic yapiyi bilmek zorunda degil
pub use telemetri::dogrulama::araligda;
pub use telemetri::{parse, Reading};
