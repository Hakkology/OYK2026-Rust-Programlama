//! Telemetri hatalari. Ders 1'deki kural: hata tipi bir ENUM olmali ki
//! cagiran `match` ile ayirt edebilsin.

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TelemetryError {
    MissingSeparator,
    UnknownField(String),
    NotANumber(String),
    OutOfRange { value: f64 },
}

// Display: kullaniciya gosterilen hali. Elle yazilir.
impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TelemetryError::MissingSeparator => write!(f, "'=' isareti yok"),
            TelemetryError::UnknownField(a) => write!(f, "beklenmeyen alan: {}", a),
            TelemetryError::NotANumber(h) => write!(f, "sayiya cevrilemedi: {}", h),
            TelemetryError::OutOfRange { value } => {
                write!(f, "aralik disinda: {}", value)
            }
        }
    }
}

// Bu satir hata tipimizi std'nin hata ekosistemine baglar (Box<dyn Error> vb.)
impl std::error::Error for TelemetryError {}
