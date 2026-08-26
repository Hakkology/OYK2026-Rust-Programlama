//! telemetry modulunun koku.
//!
//! IDIOMATIK KURAL: mod.rs sadece BILDIRIM ve RE-EXPORT tutar.
//! Is mantigi alt modullerin kendi dosyalarinda durur.

pub mod calibration;
pub mod error;
pub mod parser;
pub mod validation;

// Alt modullerden gelenleri bu seviyede yeniden yayinliyoruz:
// telemetry::parser::parse yerine telemetry::parse
pub use calibration::calibrate;
pub use error::TelemetryError;
pub use parser::{parse, Reading};
pub use validation::in_range;
