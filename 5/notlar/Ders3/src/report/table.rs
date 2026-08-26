//! Olcumleri hizalanmis tablo olarak yazar.

use crate::telemetry::Reading;

/// Basliklari ve satirlari olan bir tablo uretir.
pub fn table(readings: &[Reading]) -> String {
    let mut out = String::from("  # |   deger\n----+---------\n");
    for (i, r) in readings.iter().enumerate() {
        out.push_str(&format!("{:>3} | {:>7.1}\n", i + 1, r.value()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::parse;

    #[test]
    fn has_header() {
        assert!(table(&[]).starts_with("  # |"));
    }

    #[test]
    fn one_row_per_reading() {
        let readings = vec![
            parse("sicaklik=-60").unwrap(),
            parse("sicaklik=-40").unwrap(),
        ];
        // 2 baslik satiri + 2 veri satiri
        assert_eq!(table(&readings).lines().count(), 4);
    }
}
