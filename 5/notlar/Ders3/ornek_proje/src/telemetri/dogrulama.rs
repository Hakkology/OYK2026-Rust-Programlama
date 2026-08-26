pub const ALT: f64 = -125.0;
pub const UST: f64 = 20.0;

/// Deger Mars yuzeyi icin makul araligta mi?
pub fn araligda(d: f64) -> bool {
    d >= ALT && d <= UST
}

// sadece bu crate icinde gorunur - disariya acilmaz
pub(crate) fn aciklama() -> String {
    format!("gecerli aralik: {}..{}", ALT, UST)
}

#[cfg(test)]
mod tests {
    use super::*;                    // ust modulun her seyi, PRIVATE'lar dahil

    #[test]
    fn sinir_degerleri() {
        assert!(araligda(ALT));
        assert!(araligda(UST));
        assert!(!araligda(ALT - 0.1));
        assert!(!araligda(UST + 0.1));
    }

    #[test]
    fn private_fonksiyon_da_test_edilir() {
        assert!(aciklama().contains("-125"));
    }
}
