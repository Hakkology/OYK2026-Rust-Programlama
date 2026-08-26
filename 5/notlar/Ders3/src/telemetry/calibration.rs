//! Kalibrasyon. mod.rs'te is mantigi tutmuyoruz; her is kendi dosyasinda.

/// Ham olcumu kalibre eder.
pub fn calibrate(raw: f64) -> f64 {
    raw * correction_factor()
}

// pub yok: sadece bu modul gorur
fn correction_factor() -> f64 {
    0.98
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_correction_factor() {
        assert!((calibrate(100.0) - 98.0).abs() < 0.0001);
    }
}
