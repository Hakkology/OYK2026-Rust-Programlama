//! Olcum araligi kurallari.

pub const LOWER: f64 = -125.0;
pub const UPPER: f64 = 20.0;

/// Deger Mars yuzeyi icin makul araligta mi?
pub fn in_range(value: f64) -> bool {
    value >= LOWER && value <= UPPER
}

/// pub(crate): projenin her yerinde gorunur, disariya acilmaz.
pub(crate) fn description() -> String {
    format!("gecerli aralik: {}..{}", LOWER, UPPER)
}

/// Kardes modulu super:: ile cagiriyoruz: telemetry::calibration::calibrate
pub fn calibrated_upper() -> f64 {
    super::calibration::calibrate(UPPER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_values() {
        assert!(in_range(LOWER));
        assert!(in_range(UPPER));
        assert!(!in_range(LOWER - 0.1));
        assert!(!in_range(UPPER + 0.1));
    }

    #[test]
    fn super_call_reaches_parent_module() {
        // calibrated_upper icinde super::calibrate cagriliyor
        assert!((calibrated_upper() - 19.6).abs() < 0.0001);
    }

    #[test]
    fn pub_crate_fn_is_testable() {
        assert!(description().contains("-125"));
    }
}
