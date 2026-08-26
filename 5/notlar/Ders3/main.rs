// Gun 5 / Ders 3 - Moduller ve Gorunurluk
// rustc main.rs && ./main
//
// Tek dosyada ic ice moduller. Cok dosyali gercek karsiligi yanindaki
// ornek_proje/ klasorunde duruyor:  cd ornek_proje && cargo run

mod telemetri {
    // modul icindeki her sey VARSAYILAN OLARAK private

    /// Dogrulanmis bir olcum. Alan private oldugu icin disaridan
    /// dogrudan uretilemez - sadece new() ile uretilir.
    pub struct Reading {
        deger: f64,               // pub DEGIL: disaridan gorunmez
    }

    impl Reading {
        pub fn new(deger: f64) -> Option<Reading> {
            if dogrulama::araligda(deger) {
                Some(Reading { deger })
            } else {
                None
            }
        }

        pub fn deger(&self) -> f64 {
            self.deger
        }
    }

    // alt modul: telemetri::dogrulama
    pub mod dogrulama {
        pub const ALT: f64 = -125.0;
        pub const UST: f64 = 20.0;

        pub fn araligda(d: f64) -> bool {
            d >= ALT && d <= UST
        }

        // sadece bu crate icinde gorunur
        pub(crate) fn aciklama() -> String {
            // ic modul UST modulun her seyini gorur - private olsa bile
            format!("gecerli aralik: {}..{}", ALT, UST)
        }
    }

    // sadece bu modul ve alti gorur
    fn gizli_kalibrasyon() -> f64 {
        0.98
    }

    pub fn kalibre_et(ham: f64) -> f64 {
        ham * gizli_kalibrasyon()      // ayni modul, erisebiliyoruz
    }
}

// baska bir modul, ustteki modulu KULLANIYOR
mod rapor {
    // mutlak yol: crate kokunden
    use crate::telemetri::{dogrulama, Reading};

    pub fn ozet(olcumler: &[Reading]) -> String {
        let mut toplam = 0.0;
        for o in olcumler {
            toplam += o.deger();       // alan private, ama pub metot var
        }
        format!(
            "{} olcum, ortalama {:.2} ({})",
            olcumler.len(),
            toplam / olcumler.len() as f64,
            dogrulama::aciklama()
        )
    }
}

// pub use ile re-export: icerisi derin, disarisi duz
pub use telemetri::dogrulama::araligda;

fn main() {
    // Reading::new disinda Reading uretmenin yolu yok
    let mut olcumler: Vec<telemetri::Reading> = Vec::new();
    for d in [-63.2, -70.0, -10.0, 999.0] {
        match telemetri::Reading::new(d) {
            Some(r) => olcumler.push(r),
            None => println!("{} araligin disinda, alinmadi", d),
        }
    }

    println!("{}", rapor::ozet(&olcumler));

    // re-export sayesinde uzun yolu yazmiyoruz
    println!("araligda(-63.2) = {}", araligda(-63.2));
    println!("araligda(999)   = {}", araligda(999.0));

    println!("kalibre: {:.3}", telemetri::kalibre_et(-63.2));

    // ASAGIDAKILER DERLENMEZ - gorunurluk sinirlarini gosteriyor:

    // let r = telemetri::Reading { deger: 5.0 };
    //   E0451: field `deger` of struct `Reading` is private
    //   -> alan private, disaridan gecersiz bir Reading uretemezsiniz

    // telemetri::gizli_kalibrasyon();
    //   E0603: function `gizli_kalibrasyon` is private
    //   -> pub yazilmadigi icin modul disina cikmiyor

    // println!("{}", olcumler[0].deger);
    //   E0616: field `deger` is private
    //   -> alana degil, metoda erisebilirsiniz
}
