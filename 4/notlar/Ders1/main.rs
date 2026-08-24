// Gun 4 / Ders 1 - Struct'lar ve impl Bloklari
// rustc main.rs && ./main

use std::mem::size_of;

// klasik struct - isimli alanlar
struct Nokta {
    x: f64,
    y: f64,
}

// tuple struct - alanlar isimsiz, .0 ile erisilir
struct Metre(f64);
struct Ayak(f64);

// unit-like struct - hic alani yok, 0 bayt
struct Baslangic;

// gercek veri: gezegenler (yercekimi Dunya = 1.0)
struct Gezegen {
    ad: String,
    yaricap_km: f64,
    uydu: u32,
    yercekimi: f64,
}

impl Nokta {
    // associated function - self ALMAZ, Tip::fonksiyon() ile cagrilir
    fn yeni(x: f64, y: f64) -> Nokta {
        Nokta { x, y }                  // field init shorthand: x: x yazmaya gerek yok
    }

    fn merkez() -> Nokta {
        Nokta { x: 0.0, y: 0.0 }
    }

    // &self - sadece OKUR
    fn uzunluk(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn uzaklik(&self, digeri: &Nokta) -> f64 {
        let dx = self.x - digeri.x;
        let dy = self.y - digeri.y;
        (dx * dx + dy * dy).sqrt()
    }

    // &mut self - DEGISTIRIR
    fn otele(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    // self - TUKETIR, nesne bir daha kullanilamaz
    fn metne_cevir(self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Gezegen {
    fn yeni(ad: &str, yaricap_km: f64, uydu: u32, yercekimi: f64) -> Gezegen {
        Gezegen { ad: ad.to_string(), yaricap_km, uydu, yercekimi }
    }

    fn agirlik(&self, kilo: f64) -> f64 {
        kilo * self.yercekimi
    }

    fn uydu_ekle(&mut self) {
        self.uydu += 1;
    }
}

fn irtifa_yaz(m: &Metre) {
    println!("irtifa: {} metre", m.0);
}

fn main() {
    // olusturma ve alanlara erisim
    let n = Nokta { x: 3.0, y: 4.0 };
    println!("x={} y={}", n.x, n.y);

    // alan degistirmek icin TUM struct mut olmali - alan bazli mut yok
    let mut hareketli = Nokta::yeni(0.0, 0.0);
    hareketli.x = 5.0;
    hareketli.otele(1.0, 2.0);          // &mut self
    println!("({}, {})", hareketli.x, hareketli.y);

    // &self metotlari - nesne bizde kalir
    println!("uzunluk = {}", n.uzunluk());
    println!("merkeze uzaklik = {}", n.uzaklik(&Nokta::merkez()));
    println!("iki nokta arasi = {}", n.uzaklik(&hareketli));

    // self alan metot TUKETIR
    let gecici = Nokta::yeni(1.5, -2.5);
    println!("{}", gecici.metne_cevir());
    // println!("{}", gecici.x);        // E0382 - metne_cevir yuttu

    // ---- gercek veriyle ----
    let dunya = Gezegen::yeni("Dunya", 6371.0, 1, 1.00);
    let mars = Gezegen::yeni("Mars", 3390.0, 2, 0.38);
    let jupiter = Gezegen::yeni("Jupiter", 69911.0, 95, 2.53);

    for g in [&dunya, &mars, &jupiter] {
        println!("{:<8} yaricap={:>8} km  uydu={:<3} 70 kg -> {:.1} kg",
            g.ad, g.yaricap_km, g.uydu, g.agirlik(70.0));
    }

    // &mut self ile alan guncelleme
    let mut kesif = Gezegen::yeni("Neptun", 24622.0, 14, 1.14);
    kesif.uydu_ekle();
    println!("{} yeni uydu sayisi: {}", kesif.ad, kesif.uydu);

    // struct update syntax - yazilmayan alanlar digerinden ALINIR
    let ikiz = Gezegen { uydu: 5, ..dunya };
    println!("ikiz: {} uydu={} yercekimi={}", ikiz.ad, ikiz.uydu, ikiz.yercekimi);
    // println!("{}", dunya.ad);        // E0382 - ad bir String, .. ile TASINDI
    println!("{}", dunya.yaricap_km);   // sayi alanlari Copy, onlar hala okunabilir

    // tuple struct - ayni f64 ama AYRI tipler
    let yukseklik = Metre(8848.0);
    let yanlis_birim = Ayak(29032.0);
    irtifa_yaz(&yukseklik);
    // irtifa_yaz(&yanlis_birim);       // E0308 - Ayak, Metre degildir
    println!("ayak degeri: {}", yanlis_birim.0);

    // unit-like struct - 0 bayt
    let _b = Baslangic;
    println!("Baslangic = {} bayt", size_of::<Baslangic>());

    // bellekte struct - hizalama ve padding
    println!("(u8, u32, u8) = {} bayt (6 degil)", size_of::<(u8, u32, u8)>());
    println!("Nokta = {} bayt  Metre = {} bayt", size_of::<Nokta>(), size_of::<Metre>());

    // struct'lar stack'te, Vec<Nokta> yapinca icerik heap'te yan yana
    let yol = vec![Nokta::yeni(0.0, 0.0), Nokta::yeni(3.0, 4.0), Nokta::yeni(6.0, 8.0)];
    let mut toplam = 0.0;
    for i in 1..yol.len() {
        toplam += yol[i].uzaklik(&yol[i - 1]);
    }
    println!("yol uzunlugu = {}", toplam);
}
