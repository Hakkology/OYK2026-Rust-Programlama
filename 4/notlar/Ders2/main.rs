// Gun 4 / Ders 2 - Derive, Hata Ayiklama ve Akici API
// rustc main.rs && ./main

use std::collections::HashSet;
use std::fmt;

// tur suresi: dakika + saniye. Tum alanlar tamsayi oldugu icin
// Eq, Ord, Hash hepsi derive edilebiliyor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Sure {
    dakika: u32,
    saniye: u32,
}

// f64 alanlari var: Copy olur, PartialEq olur - ama Eq ve Ord OLMAZ
#[derive(Debug, Clone, Copy, PartialEq)]
struct Nokta {
    x: f64,
    y: f64,
}
// #[derive(Eq)] Nokta       // E0277: f64 Eq degil (NaN != NaN)

// String alani var: Clone olur ama Copy OLMAZ
#[derive(Debug, Clone, PartialEq)]
struct Pilot {
    ad: String,
    tur: Sure,
}
// #[derive(Copy)] Pilot     // E0204: String Copy degil

// Display ELLE yazilir - kullaniciya ne gosterilecegini derleyici bilemez
impl fmt::Display for Sure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}", self.dakika, self.saniye)
    }
}

// builder icin oyun karakteri
#[derive(Debug)]
struct Karakter {
    ad: String,
    can: u32,
    saldiri: u32,
    ucabilir: bool,
}

impl Karakter {
    fn yeni(ad: &str) -> Karakter {
        Karakter { ad: ad.to_string(), can: 100, saldiri: 10, ucabilir: false }
    }

    // her halka: mut self alir, bir alani degistirir, self'i geri dondurur
    fn can(mut self, x: u32) -> Self {
        self.can = x;
        self
    }

    fn saldiri(mut self, x: u32) -> Self {
        self.saldiri = x;
        self
    }

    fn ucabilir(mut self) -> Self {
        self.ucabilir = true;
        self
    }

    fn olustur(self) -> Karakter {
        self
    }
}

fn main() {
    let t1 = Sure { dakika: 3, saniye: 45 };
    let t2 = Sure { dakika: 3, saniye: 5 };

    // Debug gelistirici icin, Display kullanici icin
    println!("{:?}", t1);
    println!("{:#?}", t2);
    println!("{} ve {}", t1, t2);        // Display: 3:45 ve 3:05

    // PartialEq -> ==
    println!("esit mi: {}", t1 == t2);

    // Ord -> siralama LEKSIKOGRAFIK: once dakika, esitse saniye
    let mut turlar = vec![
        Sure { dakika: 4, saniye: 2 },
        Sure { dakika: 3, saniye: 45 },
        Sure { dakika: 3, saniye: 5 },
        Sure { dakika: 3, saniye: 45 },
    ];
    turlar.sort();
    print!("sirali turlar: ");
    for t in &turlar {
        print!("{} ", t);
    }
    println!();

    // Ord geldiyse min/max de bedava
    println!("en iyi tur: {}", turlar[0]);

    // Hash + Eq -> HashSet anahtari olabilir; tekrar eden turu bulalim
    let mut gorulen = HashSet::new();
    for t in &turlar {
        if !gorulen.insert(*t) {         // Copy oldugu icin *t ile kopyaladik
            println!("ayni tur iki kez atildi: {}", t);
        }
    }

    // Default -> alanlarin sifir degeri
    println!("varsayilan sure: {}", Sure::default());

    // Copy: atama TASIMAZ, kopyalar
    let a = Sure { dakika: 1, saniye: 30 };
    let b = a;
    println!("ikisi de yasiyor: {} {}", a, b);

    // String iceren tip Copy degil - clone gerekir
    let p1 = Pilot { ad: String::from("Ada"), tur: t1 };
    let p2 = p1.clone();
    // println!("{:?}", p1);            // p2 = p1 yazsaydik burasi E0382 olurdu
    println!("{:?} / {:?}", p1.ad, p2.ad);

    // f64 tipinde Eq yok - ama PartialEq calisiyor
    let n1 = Nokta { x: 1.0, y: 2.0 };
    let n2 = Nokta { x: 1.0, y: 2.0 };
    println!("{:?} == {:?} -> {}", n1, n2, n1 == n2);
    println!("0.1 + 0.2 == 0.3 -> {}", 0.1 + 0.2 == 0.3);   // iste bu yuzden Eq yok

    // f64 listesi sort() ile siralanamaz, partial_cmp kalibi gerekir
    let mut olcumler = vec![2.5, 1.25, 3.75];
    olcumler.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", olcumler);

    // dbg! degeri yazdirir ve GERI DONDURUR - zincire sokulabilir
    let x = 21;
    let toplam = dbg!(x + 1) * 2;
    println!("toplam = {}", toplam);

    // dbg! ve eprintln! stderr'e gider, println! stdout'a
    eprintln!("bu satir stderr'e gitti");

    // assert aileleri - beklenti bozulursa program orada durur
    assert_eq!(t2.to_string(), "3:05");
    assert!(turlar[0] <= turlar[1]);
    println!("assert'ler gecti");

    // BUILDER - alan sirasi onemsiz, yazmadiginiz alan varsayilan kalir
    let ejder = Karakter::yeni("Ejderha")
        .can(120)
        .saldiri(15)
        .ucabilir()
        .olustur();
    println!("{:?}", ejder);

    let kopek = Karakter::yeni("Kopek").olustur();   // hepsi varsayilan
    println!("{:?}", kopek);
    println!("{} can={} / {} can={}", ejder.ad, ejder.can, kopek.ad, kopek.can);

    // her halka self'i TUKETIR - ara degisken iki zincire sokulamaz
    // let ara = Karakter::yeni("Ork").can(50);
    // let bir = ara.saldiri(20);
    // let iki = ara.ucabilir();        // E0382 - ara tasindi
}
