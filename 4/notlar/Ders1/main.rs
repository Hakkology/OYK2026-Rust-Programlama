// Gun 4 / Ders 1 - Struct'lar ve impl Bloklari
// rustc main.rs && ./main

use std::mem::size_of;

// klasik struct - isimli alanlar
struct Point {
    x: f64,
    y: f64,
}

// tuple struct - alanlar isimsiz, .0 ile erisilir
struct Meters(f64);
struct Feet(f64);

// unit-like struct - hic alani yok, 0 bayt
struct Origin;

// gercek veri: gezegenler (yercekimi Dunya = 1.0)
struct Planet {
    name: String,
    radius_km: f64,
    moons: u32,
    gravity: f64,
}

impl Point {
    // associated function - self ALMAZ, Tip::fonksiyon() ile cagrilir
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }                  // field init shorthand: x: x yazmaya gerek yok
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // &self - sadece OKUR
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn distance(&self, digeri: &Point) -> f64 {
        let dx = self.x - digeri.x;
        let dy = self.y - digeri.y;
        (dx * dx + dy * dy).sqrt()
    }

    // &mut self - DEGISTIRIR
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    // self - TUKETIR, nesne bir daha kullanilamaz
    fn into_text(self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Planet {
    fn new(name: &str, radius_km: f64, moons: u32, gravity: f64) -> Planet {
        Planet { name: name.to_string(), radius_km, moons, gravity }
    }

    fn weight(&self, kilo: f64) -> f64 {
        kilo * self.gravity
    }

    fn add_moon(&mut self) {
        self.moons += 1;
    }
}

fn yut_gezegen(p: Planet) {
    println!("yutuldu: {}", p.name);
}

fn print_altitude(m: &Meters) {
    println!("irtifa: {} metre", m.0);
}

fn main() {
    // olusturma ve alanlara erisim
    let n = Point { x: 3.0, y: 4.0 };
    println!("x={} y={}", n.x, n.y);

    // alan degistirmek icin TUM struct mut olmali - alan bazli mut yok
    let mut hareketli = Point::new(0.0, 0.0);
    hareketli.x = 5.0;
    hareketli.translate(1.0, 2.0);          // &mut self
    println!("({}, {})", hareketli.x, hareketli.y);

    // &self metotlari - nesne bizde kalir
    println!("uzunluk = {}", n.length());
    println!("merkeze uzaklik = {}", n.distance(&Point::origin()));
    println!("iki nokta arasi = {}", n.distance(&hareketli));

    // self alan metot TUKETIR
    let gecici = Point::new(1.5, -2.5);
    println!("{}", gecici.into_text());
    // println!("{}", gecici.x);        // E0382 - metne_cevir yuttu

    // ---- gercek veriyle ----
    let dunya = Planet::new("Dunya", 6371.0, 1, 1.00);
    let mars = Planet::new("Mars", 3390.0, 2, 0.38);
    let jupiter = Planet::new("Jupiter", 69911.0, 95, 2.53);

    for g in [&dunya, &mars, &jupiter] {
        println!("{:<8} yaricap={:>8} km  uydu={:<3} 70 kg -> {:.1} kg",
            g.name, g.radius_km, g.moons, g.weight(70.0));
    }

    // &mut self ile alan guncelleme
    let mut kesif = Planet::new("Neptun", 24622.0, 14, 1.14);
    kesif.add_moon();
    println!("{} yeni uydu sayisi: {}", kesif.name, kesif.moons);

    // struct update syntax - yazilmayan alanlar digerinden ALINIR
    let ikiz = Planet { moons: 5, ..dunya };
    println!("ikiz: {} uydu={} yercekimi={}", ikiz.name, ikiz.moons, ikiz.gravity);

    // dunya artik KISMEN TASINMIS. Uc ayri durum:
    println!("{} {}", dunya.radius_km, dunya.moons);  // 1) Copy alanlar: calisir
    // println!("{}", dunya.name);                    // 2) E0382 borrow of moved value
    // yut_gezegen(dunya);                            // 3) E0382 use of partially moved value
    // .. butunu goturmez, sadece Copy olmayan alani (name) tasir.

    // tuple struct - ayni f64 ama AYRI tipler
    let yukseklik = Meters(8848.0);
    let yanlis_birim = Feet(29032.0);
    print_altitude(&yukseklik);
    // print_altitude(&yanlis_birim);   // E0308 - Feet, Meters degildir
    println!("ayak degeri: {}", yanlis_birim.0);

    // unit-like struct - 0 bayt
    let _b = Origin;
    println!("Baslangic = {} bayt", size_of::<Origin>());

    // bellekte struct - hizalama ve padding
    println!("(u8, u32, u8) = {} bayt (6 degil)", size_of::<(u8, u32, u8)>());
    println!("Point = {} bayt  Meters = {} bayt", size_of::<Point>(), size_of::<Meters>());

    // struct'lar stack'te, Vec<Point> yapinca icerik heap'te yan yana
    let yol = vec![Point::new(0.0, 0.0), Point::new(3.0, 4.0), Point::new(6.0, 8.0)];
    let mut toplam = 0.0;
    for i in 1..yol.len() {
        toplam += yol[i].distance(&yol[i - 1]);
    }
    println!("yol uzunlugu = {}", toplam);
}
