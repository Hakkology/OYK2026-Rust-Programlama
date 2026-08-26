//! Telemetri satirlarini ayristirir.

use super::error::TelemetryError;   // kardes alt modul
use super::validation;              // super:: = bir ust modul (telemetry)

/// Dogrulanmis bir olcum.
#[derive(Debug, PartialEq)]
pub struct Reading {
    value: f64,                 // private: gecersiz Reading uretilemesin
}

impl Reading {
    pub fn value(&self) -> f64 {
        self.value
    }
}

/// "sicaklik=-63.2" satirini ayristirir.
///
/// Bu ornek `cargo test` ile CALISTIRILIR:
/// ```
/// let r = ders3::parse("sicaklik=-63.2").unwrap();
/// assert!(r.value() < 0.0);
/// ```
pub fn parse(line: &str) -> Result<Reading, TelemetryError> {
    let eq = line.find('=').ok_or(TelemetryError::MissingSeparator)?;

    let alan = &line[..eq];
    if alan != "sicaklik" {
        return Err(TelemetryError::UnknownField(alan.to_string()));
    }

    let ham = &line[eq + 1..];
    let value: f64 = ham
        .parse()
        .map_err(|_| TelemetryError::NotANumber(ham.to_string()))?;

    if !validation::in_range(value) {
        return Err(TelemetryError::OutOfRange { value });
    }
    Ok(Reading { value })       // ayni modulde oldugumuz icin private alani doldurabiliriz
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_line() {
        let r = parse("sicaklik=-63.2").unwrap();
        assert!((r.value() - (-63.2)).abs() < 0.0001);
    }

    #[test]
    fn rejects_wrong_field() {
        // hata tipi enum oldugu icin HANGI hata oldugunu da dogrulayabiliyoruz
        assert_eq!(
            parse("nem=40"),
            Err(TelemetryError::UnknownField(String::from("nem")))
        );
    }

    #[test]
    fn rejects_out_of_range() {
        assert_eq!(
            parse("sicaklik=999"),
            Err(TelemetryError::OutOfRange { value: 999.0 })
        );
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse("sicaklik"), Err(TelemetryError::MissingSeparator));
    }
}
