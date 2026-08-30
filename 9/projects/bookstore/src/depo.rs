//! Gun 7: trait + dyn.  Gun 9: Arc<Mutex<T>>.
//!
//! Verinin nerede durdugunu API katmani BILMIYOR - sadece `dyn Depo` goruyor.
//! Bugun bellekte; yarin veritabani yazsaniz API'ye dokunmazsiniz.

use crate::kitap::{Kitap, KitapHatasi, YeniKitap};
use std::collections::HashMap;
use std::sync::Mutex;

// Send + Sync: bu tip thread'ler arasinda paylasilacak (Gun 9 Lesson 1).
// axum her istegi ayri bir task'ta calistirdigi icin sart.
pub trait Depo: Send + Sync {
    fn hepsi(&self) -> Vec<Kitap>;
    fn bul(&self, id: u32) -> Result<Kitap, KitapHatasi>;
    fn ekle(&self, y: YeniKitap) -> Result<Kitap, KitapHatasi>;
    fn sil(&self, id: u32) -> Result<Kitap, KitapHatasi>;
    fn satis(&self, id: u32, adet: u32) -> Result<f64, KitapHatasi>;

    fn sayi(&self) -> usize {
        self.hepsi().len()
    }
}

pub struct BellekDepo {
    // Mutex icinde: birden cok istek AYNI ANDA gelebilir
    kayitlar: Mutex<HashMap<u32, Kitap>>,
    sonraki_id: Mutex<u32>,
}

impl BellekDepo {
    pub fn yeni() -> BellekDepo {
        BellekDepo {
            kayitlar: Mutex::new(HashMap::new()),
            sonraki_id: Mutex::new(1),
        }
    }

    pub fn ornek_veriyle() -> BellekDepo {
        let d = BellekDepo::yeni();
        let kitaplar = [
            ("Rust Programlama Dili", "Klabnik & Nichols", 450.0, 12u32),
            ("Programming Rust", "Blandy & Orendorff", 890.0, 5),
            ("Rust for Rustaceans", "Jon Gjengset", 720.0, 0),
        ];
        for (baslik, yazar, fiyat, stok) in kitaplar {
            let _ = d.ekle(YeniKitap {
                baslik: baslik.to_string(),
                yazar: yazar.to_string(),
                fiyat,
                stok,
            });
        }
        d
    }
}

impl Default for BellekDepo {
    fn default() -> Self {
        Self::yeni()
    }
}

impl Depo for BellekDepo {
    fn hepsi(&self) -> Vec<Kitap> {
        let k = self.kayitlar.lock().unwrap();
        let mut v: Vec<Kitap> = k.values().cloned().collect();
        v.sort_by_key(|x| x.id); // HashMap sirasiz - Gun 3
        v
    }

    fn bul(&self, id: u32) -> Result<Kitap, KitapHatasi> {
        self.kayitlar
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(KitapHatasi::Bulunamadi(id))
    }

    fn ekle(&self, y: YeniKitap) -> Result<Kitap, KitapHatasi> {
        Kitap::dogrula(&y)?; // Gun 5: ? ile erken donus

        let id = {
            let mut sayac = self.sonraki_id.lock().unwrap();
            let simdiki = *sayac;
            *sayac += 1;
            simdiki
        }; // kilit BURADA birakiliyor - digerini almadan once

        let kitap = Kitap {
            id,
            baslik: y.baslik,
            yazar: y.yazar,
            fiyat: y.fiyat,
            stok: y.stok,
        };

        self.kayitlar.lock().unwrap().insert(id, kitap.clone());
        Ok(kitap)
    }

    fn sil(&self, id: u32) -> Result<Kitap, KitapHatasi> {
        self.kayitlar
            .lock()
            .unwrap()
            .remove(&id)
            .ok_or(KitapHatasi::Bulunamadi(id))
    }

    fn satis(&self, id: u32, adet: u32) -> Result<f64, KitapHatasi> {
        let mut kayitlar = self.kayitlar.lock().unwrap();
        let kitap = kayitlar.get_mut(&id).ok_or(KitapHatasi::Bulunamadi(id))?;

        if kitap.stok < adet {
            // Hicbir sey DEGISTIRMEDEN hata donuyoruz - kismi degisiklik yok
            return Err(KitapHatasi::YetersizStok {
                id,
                istenen: adet,
                mevcut: kitap.stok,
            });
        }

        kitap.stok -= adet;
        Ok(kitap.fiyat * adet as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn yeni(baslik: &str, fiyat: f64, stok: u32) -> YeniKitap {
        YeniKitap {
            baslik: baslik.to_string(),
            yazar: String::from("Y"),
            fiyat,
            stok,
        }
    }

    #[test]
    fn ekleme_id_uretir() {
        let d = BellekDepo::yeni();
        assert_eq!(d.ekle(yeni("A", 10.0, 1)).unwrap().id, 1);
        assert_eq!(d.ekle(yeni("B", 20.0, 1)).unwrap().id, 2);
        assert_eq!(d.sayi(), 2);
    }

    #[test]
    fn olmayan_kitap_bulunamaz() {
        let d = BellekDepo::yeni();
        assert_eq!(d.bul(99), Err(KitapHatasi::Bulunamadi(99)));
    }

    #[test]
    fn hepsi_id_sirali_doner() {
        let d = BellekDepo::ornek_veriyle();
        let ids: Vec<u32> = d.hepsi().iter().map(|k| k.id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn satis_stogu_dusurur() {
        let d = BellekDepo::yeni();
        let k = d.ekle(yeni("A", 100.0, 10)).unwrap();
        assert_eq!(d.satis(k.id, 3).unwrap(), 300.0);
        assert_eq!(d.bul(k.id).unwrap().stok, 7);
    }

    #[test]
    fn yetersiz_stokta_stok_degismez() {
        let d = BellekDepo::yeni();
        let k = d.ekle(yeni("A", 100.0, 2)).unwrap();
        assert!(d.satis(k.id, 5).is_err());
        assert_eq!(d.bul(k.id).unwrap().stok, 2); // DEGISMEDI
    }

    #[test]
    fn tam_stok_satilabilir() {
        let d = BellekDepo::yeni();
        let k = d.ekle(yeni("A", 100.0, 2)).unwrap();
        assert!(d.satis(k.id, 2).is_ok());
        assert_eq!(d.bul(k.id).unwrap().stok, 0);
    }

    #[test]
    fn gecersiz_kitap_eklenmez() {
        let d = BellekDepo::yeni();
        assert!(d.ekle(yeni("", 10.0, 1)).is_err());
        assert_eq!(d.sayi(), 0);
    }

    #[test]
    fn dyn_uzerinden_calisir() {
        let d: Box<dyn Depo> = Box::new(BellekDepo::ornek_veriyle());
        assert_eq!(d.sayi(), 3);
    }
}
