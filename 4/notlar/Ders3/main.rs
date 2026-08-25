// Gun 4 / Ders 3 - Enum'lar
// rustc main.rs && ./main

use std::mem::size_of;

// 1) SINIRLI SECENEKLER - veri tasimayan enum, C'deki gibi
#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 5) VARYANTLAR VERI TASIR - Rust'in farki
#[derive(Debug)]
enum Shape {
    Dot,                              // veri yok
    Circle { r: f64 },                  // isimli alan
    Rectangle { width: f64, height: f64 },   // iki isimli alan
    Triangle(f64, f64, f64),               // isimsiz uclu
}

// 6) GECERSIZ DURUM TEMSIL EDILEMEZ - dort ihtimal, baskasi yok
#[derive(Debug, Clone, Copy, PartialEq)]
enum Base {
    A,
    T,
    G,
    C,
}

// kenar not 2 - sayisal deger, sadece veri tasimayan enum'da
enum HttpStatus {
    Success = 200,
    NotFound = 404,
}

impl TrafficLight {
    // 3) enum'a da impl yazilir, self secimi struct'takiyle ayni
    fn seconds(&self) -> u32 {
        match self {
            TrafficLight::Red => 45,
            TrafficLight::Yellow => 4,
            TrafficLight::Green => 30,
        }
    }

    // 4) DURUM MAKINESI - gecisler tek yerde toplanir
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

impl Shape {
    fn area(&self) -> f64 {
        // desen hem HANGISI oldugunu soyler hem ICINDEKINI verir
        match self {
            Shape::Dot => 0.0,
            Shape::Circle { r } => 3.14159 * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;          // Heron formulu
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

impl Base {
    // DNA'da A-T, G-C eslesir
    fn complement(&self) -> Base {
        match self {
            Base::A => Base::T,
            Base::T => Base::A,
            Base::G => Base::C,
            Base::C => Base::G,
        }
    }
}

// kural 2: sonuc bulunamayabiliyorsa donus tipi Option<T> olur.
// -> i32 yazsaydik "bulunamadi" durumunu ifade edecek yolumuz olmazdi.
fn first_negative(sayilar: &[i32]) -> Option<i32> {
    for n in sayilar {
        if *n < 0 {
            return Some(*n);
        }
    }
    None
}

struct User {
    id: u64,                    // her kullanicinin ID'si olmak zorunda
    name: String,                 // her kullanicinin adi var
    middle_name: Option<String>,// bazi insanlarin ikinci ismi YOK
    health: i32,                   // eksik olamaz, ama 0 olabilir
}

// arama bulamayabilir -> Option<&User>
fn find(liste: &[User], id: u64) -> Option<&User> {
    for k in liste {
        if k.id == id {
            return Some(k);
        }
    }
    None
}

fn main() {
    // -----------------------------------------------------------
    // 1-2) sinirli secenekler + match ile okumak
    // -----------------------------------------------------------
    for isik in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green] {
        let davranis = match isik {
            TrafficLight::Red => "dur",
            TrafficLight::Yellow => "hazirlan",
            TrafficLight::Green => "gec",
        };
        println!("{:?}: {} ({} sn)", isik, davranis, isik.seconds());
    }

    // -----------------------------------------------------------
    // 4) durum makinesi
    // -----------------------------------------------------------
    let mut isik = TrafficLight::Red;
    print!("dongu: ");
    for _ in 0..5 {
        print!("{:?} -> ", isik);
        isik = isik.next();
    }
    println!("{:?}", isik);

    // -----------------------------------------------------------
    // 5) varyantlar veri tasir - hepsi ayni Vec'in icinde durabiliyor
    // -----------------------------------------------------------
    let sekiller = vec![
        Shape::Dot,
        Shape::Circle { r: 2.0 },
        Shape::Rectangle { width: 3.0, height: 4.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    let mut toplam = 0.0;
    for s in &sekiller {
        println!("{:<34} alan = {:.2}", format!("{:?}", s), s.area());
        toplam += s.area();
    }
    println!("toplam alan = {:.2}", toplam);

    // -----------------------------------------------------------
    // 6) gecersiz durum temsil edilemez
    //    metinle olsaydi: let baz = "X";  -> derlenir, sessizce sacmalar
    //    let baz = Base::X;               // E0599 no variant named `X`
    // -----------------------------------------------------------
    let dizi = [Base::A, Base::T, Base::G, Base::G, Base::C, Base::A];
    print!("dizi        : ");
    for b in &dizi {
        print!("{:?} ", b);
    }
    println!();
    print!("tamamlayici : ");
    for b in &dizi {
        print!("{:?} ", b.complement());
    }
    println!();

    let mut gc = 0;
    for b in &dizi {
        if *b == Base::G || *b == Base::C {
            gc += 1;
        }
    }
    println!("GC orani = {:.0}%", 100.0 * gc as f64 / dizi.len() as f64);

    // -----------------------------------------------------------
    // 7) Option - null'un yerine gecen enum
    //    pub enum Option<T> { None, Some(T) }
    //
    //    Kontrolsuz null referanslari yerine Rust, bir degerin VARLIGINI veya
    //    YOKLUGUNU tip seviyesinde zorunlu bir SOZLESME haline getirir.
    //
    //    UC KURAL:
    //    1. Tipi T olan deger KESINLIKLE vardir       -> null yazamazsiniz
    //    2. Bulunmama ihtimali varsa tip Option<T>'dir -> imzada yazar
    //    3. Acmadan icindeki T'ye ERISEMEZSINIZ        -> unutmak mumkun degil
    // -----------------------------------------------------------

    // kural 1: i32 dediyseniz elinizde bir sayi VAR
    let kesin: i32 = 42;
    println!("kesin = {}", kesin);
    // let kesin2: i32 = None;          // E0308 - null diye bir sey yok

    // kural 2: bulunamama ihtimali imzaya yazilir
    let olcumler = [3, -7, 12, -1];
    println!("{:?} {:?}", first_negative(&olcumler), first_negative(&[1, 2, 3]));

    // kural 3: kutuyu acmadan icindekini kullanamazsiniz
    let d: Option<i32> = Some(5);
    // let e: i32 = d;                  // E0308 - Option<i32>, i32 degildir
    // let f = d + 1;                   // E0369 - Option'a toplama yapilmaz

    // acmanin yollari
    match d {
        Some(n) => println!("match     -> {}", n),
        None => println!("match     -> deger yok"),
    }
    if let Some(n) = d {
        println!("if let    -> {}", n);
    }
    println!("unwrap    -> {}", d.unwrap());            // bossa PANIKLER
    println!("expect    -> {}", d.expect("olcum bekleniyordu"));
    println!("unwrap_or -> {}", None.unwrap_or(0));     // bossa varsayilan

    // HANGI DURUMDA HANGI TIP - her alan icin "bu olmayabilir mi?" diye sorun
    let kayitlar = vec![
        User { id: 1, name: String::from("Ada"), middle_name: Some(String::from("Lovelace")), health: 100 },
        User { id: 2, name: String::from("Ege"), middle_name: None, health: 0 },
    ];

    for k in &kayitlar {
        // id: u64        -> her zaman var
        // ikinci_isim    -> olmayabilir, o yuzden Option<String>
        // can: i32       -> eksik olamaz ama 0 OLABILIR (0 ile "yok" ayni sey degil)
        match &k.middle_name {
            Some(i) => println!("#{} {} {} (can {})", k.id, k.name, i, k.health),
            None => println!("#{} {} (ikinci isim yok, can {})", k.id, k.name, k.health),
        }
    }

    // uzunluk her zaman vardir -> usize, Option degil
    println!("kayit sayisi = {}", kayitlar.len());

    // arama BULAMAYABILIR -> Option<&User>
    match find(&kayitlar, 2) {
        Some(k) => println!("bulundu: {}", k.name),
        None => println!("bulunamadi"),
    }
    match find(&kayitlar, 99) {
        Some(k) => println!("bulundu: {}", k.name),
        None => println!("99 numarali kayit bulunamadi"),
    }
    // C#/C++ tarafinda ayni fonksiyon User dondururdu ve null gelebilecegi
    // imzadan ANLASILMAZDI. Burada imza soyluyor.

    // 0 ILE "YOK" AYNI SEY DEGIL - en cok karistirilan yer
    // sensor okundu ve 0 gosterdi   -> Some(0)
    // sensor hic okunamadi          -> None
    let okumalar: [Option<i32>; 3] = [Some(21), Some(0), None];
    for (i, o) in okumalar.iter().enumerate() {
        match o {
            Some(0) => println!("sensor {}: okundu, deger 0 (buz gibi ama calisiyor)", i),
            Some(d) => println!("sensor {}: okundu, deger {}", i, d),
            None => println!("sensor {}: HIC OKUNAMADI (arizali olabilir)", i),
        }
    }
    // ayni veriyi tek i32 ile tutsaydik 0 ile "okunamadi" ayni gorunurdu
    // ve "buz gibi" ile "arizali" ayrimini yapamazdik

    // ayni is, iki durum yan yana
    for o in [Some(5), None] {
        match o {
            Some(n) => println!("deger var: {}", n),
            None => println!("deger yok"),
        }
    }

    // -----------------------------------------------------------
    // kenar not 1 - bellekte enum: etiket + en buyuk varyant + hizalama
    // -----------------------------------------------------------
    println!("TrafficLight = {} bayt   Shape = {} bayt", size_of::<TrafficLight>(), size_of::<Shape>());

    // niche optimization: Box asla null olamaz, None o bos desene yerlesir
    println!("Box<i32>         = {} bayt", size_of::<Box<i32>>());
    println!("Option<Box<i32>> = {} bayt  <- None bedava", size_of::<Option<Box<i32>>>());
    println!("i32              = {} bayt", size_of::<i32>());
    println!("Option<i32>      = {} bayt  <- etiket icin yer gerekti", size_of::<Option<i32>>());

    // ayni garanti referanslarda da var: safe Rust'ta &T asla null olamaz
    println!("&i32                 = {} bayt", size_of::<&i32>());
    println!("Option<&i32>         = {} bayt  <- None bedava", size_of::<Option<&i32>>());
    // ham isaretci null OLABILIR, o yuzden bos desen kalmiyor:
    println!("*const i32           = {} bayt", size_of::<*const i32>());
    println!("Option<*const i32>   = {} bayt  <- iki katina cikti", size_of::<Option<*const i32>>());

    // kenar not 2 - sayisal deger
    println!("{} {}", HttpStatus::Success as i32, HttpStatus::NotFound as i32);
}
