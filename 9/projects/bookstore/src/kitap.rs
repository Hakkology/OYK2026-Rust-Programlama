//! Gun 4: struct + enum.  Gun 5: hata tipi.
//! serde'nin Serialize/Deserialize'i de birer derive - Gun 4'te gorduk.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kitap {
    pub id: u32,
    pub baslik: String,
    pub yazar: String,
    pub fiyat: f64,
    pub stok: u32,
}

/// POST govdesi - id'yi sunucu uretiyor, istemci gondermiyor
#[derive(Debug, Deserialize)]
pub struct YeniKitap {
    pub baslik: String,
    pub yazar: String,
    pub fiyat: f64,
    pub stok: u32,
}

#[derive(Debug, PartialEq)]
pub enum KitapHatasi {
    Bulunamadi(u32),
    GecersizFiyat(f64),
    BosBaslik,
    YetersizStok { id: u32, istenen: u32, mevcut: u32 },
}

impl fmt::Display for KitapHatasi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KitapHatasi::Bulunamadi(id) => write!(f, "kitap bulunamadi: {}", id),
            KitapHatasi::GecersizFiyat(p) => write!(f, "gecersiz fiyat: {}", p),
            KitapHatasi::BosBaslik => write!(f, "baslik bos olamaz"),
            KitapHatasi::YetersizStok {
                id,
                istenen,
                mevcut,
            } => write!(
                f,
                "kitap {} icin yetersiz stok: {} istendi, {} mevcut",
                id, istenen, mevcut
            ),
        }
    }
}

impl std::error::Error for KitapHatasi {}

impl Kitap {
    pub fn dogrula(y: &YeniKitap) -> Result<(), KitapHatasi> {
        if y.baslik.trim().is_empty() {
            return Err(KitapHatasi::BosBaslik);
        }
        if y.fiyat < 0.0 {
            return Err(KitapHatasi::GecersizFiyat(y.fiyat));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ornek(baslik: &str, fiyat: f64) -> YeniKitap {
        YeniKitap {
            baslik: baslik.to_string(),
            yazar: String::from("Yazar"),
            fiyat,
            stok: 1,
        }
    }

    #[test]
    fn gecerli_kitap_kabul_edilir() {
        assert!(Kitap::dogrula(&ornek("Rust", 100.0)).is_ok());
    }

    #[test]
    fn bos_baslik_reddedilir() {
        assert_eq!(
            Kitap::dogrula(&ornek("   ", 100.0)),
            Err(KitapHatasi::BosBaslik)
        );
    }

    #[test]
    fn negatif_fiyat_reddedilir() {
        assert!(matches!(
            Kitap::dogrula(&ornek("Rust", -1.0)),
            Err(KitapHatasi::GecersizFiyat(_))
        ));
    }

    #[test]
    fn sifir_fiyat_gecerli() {
        assert!(Kitap::dogrula(&ornek("Bedava", 0.0)).is_ok());
    }
}
