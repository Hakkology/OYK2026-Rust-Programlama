//! Kucuk bir hesap kutuphanesi.
//!
//! Bu crate Gun 9'un ilk dersi icin var: amaci ozellik degil, **test edilebilirlik**.
//!
//! ```
//! use ders1::Bill;
//!
//! let mut bill = Bill::new();
//! bill.add("corba", 120);
//! bill.add("pilav", 80);
//! assert_eq!(bill.total(), 200);
//! ```

/// Bir hesap fisi. Tutarlar **kurus** cinsinden tam sayidir.
///
/// Para neden `f64` degil: `0.1 + 0.2 != 0.3`. Gun 6'da konusmustuk.
#[derive(Debug, Default, PartialEq)]
pub struct Bill {
    items: Vec<(String, u32)>,
}

impl Bill {
    /// Bos bir fis olusturur.
    pub fn new() -> Bill {
        Bill { items: Vec::new() }
    }

    /// Fise bir kalem ekler.
    ///
    /// ```
    /// let mut bill = ders1::Bill::new();
    /// bill.add("baklava", 250);
    /// assert_eq!(bill.len(), 1);
    /// ```
    pub fn add(&mut self, name: &str, kurus: u32) {
        self.items.push((name.to_string(), kurus));
    }

    /// Toplam tutar.
    pub fn total(&self) -> u32 {
        self.items.iter().map(|(_, k)| k).sum()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Hesabi `kisi` kisiye boler.
    ///
    /// # Panics
    ///
    /// `kisi` sifirsa panikler.
    ///
    /// ```
    /// let mut bill = ders1::Bill::new();
    /// bill.add("corba", 100);
    /// assert_eq!(bill.split(4), 25);
    /// ```
    pub fn split(&self, kisi: u32) -> u32 {
        assert!(kisi > 0, "kisi sayisi sifir olamaz");
        self.total() / kisi
    }
}

/// Yuzde bahsis ekler.
///
/// Yalnizca `tips` feature'i acikken derlenir:
/// `cargo test --features tips`
#[cfg(feature = "tips")]
pub fn with_tip(total: u32, yuzde: u32) -> u32 {
    total + total * yuzde / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bos_fis_sifir() {
        let bill = Bill::new();
        assert!(bill.is_empty());
        assert_eq!(bill.total(), 0);
    }

    #[test]
    fn kalemler_toplaniyor() {
        let mut bill = Bill::new();
        bill.add("corba", 120);
        bill.add("pilav", 80);
        // assert_eq! basarisiz olunca IKI DEGERI de yazdirir; assert! sadece "false" der.
        assert_eq!(bill.total(), 200);
    }

    #[test]
    #[should_panic(expected = "kisi sayisi sifir olamaz")]
    fn sifira_bolunmez() {
        // expected: panic MESAJINI da kontrol eder. Yanlis sebeple panikleyen
        // bir test yanlislikla gecmez.
        Bill::new().split(0);
    }

    #[test]
    #[ignore = "yavas: elle calistirin -> cargo test -- --ignored"]
    fn cok_kalemli_fis() {
        let mut bill = Bill::new();
        for i in 0..100_000 {
            bill.add("kalem", i % 10);
        }
        assert_eq!(bill.len(), 100_000);
    }

    #[cfg(feature = "tips")]
    #[test]
    fn bahsis_ekleniyor() {
        assert_eq!(with_tip(200, 10), 220);
    }
}
