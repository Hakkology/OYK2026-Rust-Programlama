//! Gun 5 / Ders 3 - Moduller ve Proje Organizasyonu
//!
//! KUTUPHANE KOKU. Modul agaci burada baslar ve DISARIYA ACILIR:
//!
//!   src/lib.rs                    pub mod telemetry;  pub mod report;
//!   src/telemetry/mod.rs          sadece bildirim + re-export
//!   src/telemetry/error.rs        TelemetryError enum + Display
//!   src/telemetry/parser.rs       Reading, parse()        + testleri
//!   src/telemetry/validation.rs   in_range(), sinirlar    + testleri
//!   src/telemetry/calibration.rs  calibrate()             + testleri
//!   src/report/mod.rs             pub mod summary;  pub mod table;
//!   src/report/summary.rs         summary()               + testleri
//!   src/report/table.rs           table()                 + testleri
//!   src/main.rs                   ikili: kisa yollari kullanir
//!   src/bin/report_cli.rs         ikili: report MODULUNU dogrudan kullanir
//!   tests/integration.rs          sadece public API

// pub mod = modulu var eder VE disariya acar.
// Sadece `mod` yazsaydik ikililer bu modulleri goremezdi.
pub mod report;
pub mod telemetry;

// pub use = ayni seylere KISA yoldan erisim.
// Kullanici isterse uzun yolu (ders3::report::summary), isterse kisasini (ders3::summary) yazar.
pub use report::{summary, table};
pub use telemetry::{in_range, parse, Reading, TelemetryError};
