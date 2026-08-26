//! Tek satirlik ozet uretir.

// KARDES MODULE erisim: crate:: ile mutlak yol
use crate::telemetry::{calibrate, validation, Reading};

/// Rapor basligi. pub(crate): kutuphane icinde kullanilir, DISARIYA acilmaz.
pub(crate) fn internal_label() -> &'static str {
    "TELEMETRI"
}

/// Olcumlerin sayisini, ortalamasini ve kalibreli halini ozetler.
pub fn summary(readings: &[Reading]) -> String {
    if readings.is_empty() {
        return String::from("olcum yok");
    }
    let mut total = 0.0;
    for r in readings {
        total += r.value();
    }
    let avg = total / readings.len() as f64;
    format!(
        "[{}] {} olcum | ortalama {:.2} | kalibreli {:.2} | {} (kalibreli ust sinir {:.2})",
        internal_label(),
        readings.len(),
        avg,
        calibrate(avg),                    // disariya acilmadi ama crate icinde kullanilabiliyor
        validation::description(),         // pub(crate)
        validation::calibrated_upper()     // icinde super::calibrate cagiriyor
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::parse;     // testte de kardes modulu kullanabiliyoruz

    #[test]
    fn empty_input() {
        assert_eq!(summary(&[]), "olcum yok");
    }

    #[test]
    fn counts_and_averages() {
        let readings = vec![
            parse("sicaklik=-60").unwrap(),
            parse("sicaklik=-40").unwrap(),
        ];
        let s = summary(&readings);
        assert!(s.contains("2 olcum"));
        assert!(s.contains("-50.00"));
    }
}
