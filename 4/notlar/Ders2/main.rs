// Gun 4 / Ders 2 - Derive, Hata Ayiklama ve Akici API
// rustc main.rs && ./main

use std::collections::HashSet;
use std::fmt;

// tur suresi: dakika + saniye. Tum alanlar tamsayi oldugu icin
// Eq, Ord, Hash hepsi derive edilebiliyor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct LapTime {
    minutes: u32,
    seconds: u32,
}

// f64 alanlari var: Copy olur, PartialEq olur - ama Eq ve Ord OLMAZ
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
// #[derive(Eq)] Point      // E0277: f64 Eq degil (NaN != NaN)

// String alani var: Clone olur ama Copy OLMAZ
#[derive(Debug, Clone, PartialEq)]
struct Pilot {
    name: String,
    lap: LapTime,
}
// #[derive(Copy)] Pilot     // E0204: String Copy degil

// Display ELLE yazilir - kullaniciya ne gosterilecegini derleyici bilemez
impl fmt::Display for LapTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}", self.minutes, self.seconds)
    }
}

// builder icin oyun karakteri
#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    attack: u32,
    can_fly: bool,
}

impl Character {
    fn new(name: &str) -> Character {
        Character { name: name.to_string(), health: 100, attack: 10, can_fly: false }
    }

    // her halka: mut self alir, bir alani degistirir, self'i geri dondurur
    fn health(mut self, x: u32) -> Self {
        self.health = x;
        self
    }

    fn attack(mut self, x: u32) -> Self {
        self.attack = x;
        self
    }

    fn can_fly(mut self) -> Self {
        self.can_fly = true;
        self
    }

    fn build(self) -> Character {
        self
    }
}

fn main() {
    let t1 = LapTime { minutes: 3, seconds: 45 };
    let t2 = LapTime { minutes: 3, seconds: 5 };

    // Debug gelistirici icin, Display kullanici icin
    println!("{:?}", t1);
    println!("{:#?}", t2);
    println!("{} ve {}", t1, t2);        // Display: 3:45 ve 3:05

    // PartialEq -> ==
    println!("esit mi: {}", t1 == t2);

    // Ord -> siralama LEKSIKOGRAFIK: once dakika, esitse saniye
    let mut turlar = vec![
        LapTime { minutes: 4, seconds: 2 },
        LapTime { minutes: 3, seconds: 45 },
        LapTime { minutes: 3, seconds: 5 },
        LapTime { minutes: 3, seconds: 45 },
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
    println!("varsayilan sure: {}", LapTime::default());

    // Copy: atama TASIMAZ, kopyalar
    let a = LapTime { minutes: 1, seconds: 30 };
    let b = a;
    println!("ikisi de yasiyor: {} {}", a, b);

    // String iceren tip Copy degil - clone gerekir
    let p1 = Pilot { name: String::from("Ada"), lap: t1 };
    let p2 = p1.clone();
    // println!("{:?}", p1);            // p2 = p1 yazsaydik burasi E0382 olurdu
    println!("{:?} / {:?}", p1.name, p2.name);

    // f64 tipinde Eq yok - ama PartialEq calisiyor
    let n1 = Point { x: 1.0, y: 2.0 };
    let n2 = Point { x: 1.0, y: 2.0 };
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
    let ejder = Character::new("Ejderha")
        .health(120)
        .attack(15)
        .can_fly()
        .build();
    println!("{:?}", ejder);

    let kopek = Character::new("Kopek").build();   // hepsi varsayilan
    println!("{:?}", kopek);
    println!("{} can={} / {} can={}", ejder.name, ejder.health, kopek.name, kopek.health);

    // her halka self'i TUKETIR - ara degisken iki zincire sokulamaz
    // let ara = Character::new("Ork").health(50);
    // let bir = ara.attack(20);
    // let iki = ara.can_fly();         // E0382 - ara tasindi
}
