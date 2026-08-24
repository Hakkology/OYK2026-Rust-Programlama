// Gun 4 / Ders 3 - Enum'lar
// rustc main.rs && ./main

use std::mem::size_of;

// 1) SINIRLI SECENEKLER - veri tasimayan enum, C'deki gibi
#[derive(Debug, Clone, Copy, PartialEq)]
enum Isik {
    Kirmizi,
    Sari,
    Yesil,
}

// 5) VARYANTLAR VERI TASIR - Rust'in farki
#[derive(Debug)]
enum Sekil {
    Nokta,                              // veri yok
    Cember { r: f64 },                  // isimli alan
    Dikdortgen { en: f64, boy: f64 },   // iki isimli alan
    Ucgen(f64, f64, f64),               // isimsiz uclu
}

// 6) GECERSIZ DURUM TEMSIL EDILEMEZ - dort ihtimal, baskasi yok
#[derive(Debug, Clone, Copy, PartialEq)]
enum Baz {
    A,
    T,
    G,
    C,
}

// kenar not 2 - sayisal deger, sadece veri tasimayan enum'da
enum HttpDurum {
    Tamam = 200,
    Bulunamadi = 404,
}

impl Isik {
    // 3) enum'a da impl yazilir, self secimi struct'takiyle ayni
    fn saniye(&self) -> u32 {
        match self {
            Isik::Kirmizi => 45,
            Isik::Sari => 4,
            Isik::Yesil => 30,
        }
    }

    // 4) DURUM MAKINESI - gecisler tek yerde toplanir
    fn sonraki(&self) -> Isik {
        match self {
            Isik::Kirmizi => Isik::Yesil,
            Isik::Yesil => Isik::Sari,
            Isik::Sari => Isik::Kirmizi,
        }
    }
}

impl Sekil {
    fn alan(&self) -> f64 {
        // desen hem HANGISI oldugunu soyler hem ICINDEKINI verir
        match self {
            Sekil::Nokta => 0.0,
            Sekil::Cember { r } => 3.14159 * r * r,
            Sekil::Dikdortgen { en, boy } => en * boy,
            Sekil::Ucgen(a, b, c) => {
                let s = (a + b + c) / 2.0;          // Heron formulu
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

impl Baz {
    // DNA'da A-T, G-C eslesir
    fn tamamlayici(&self) -> Baz {
        match self {
            Baz::A => Baz::T,
            Baz::T => Baz::A,
            Baz::G => Baz::C,
            Baz::C => Baz::G,
        }
    }
}

// kural 2: sonuc bulunamayabiliyorsa donus tipi Option<T> olur.
// -> i32 yazsaydik "bulunamadi" durumunu ifade edecek yolumuz olmazdi.
fn ilk_negatif(sayilar: &[i32]) -> Option<i32> {
    for n in sayilar {
        if *n < 0 {
            return Some(*n);
        }
    }
    None
}

struct Kullanici {
    id: u64,                    // her kullanicinin ID'si olmak zorunda
    ad: String,                 // her kullanicinin adi var
    ikinci_isim: Option<String>,// bazi insanlarin ikinci ismi YOK
    can: i32,                   // eksik olamaz, ama 0 olabilir
}

// arama bulamayabilir -> Option<&Kullanici>
fn ara(liste: &[Kullanici], id: u64) -> Option<&Kullanici> {
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
    for isik in [Isik::Kirmizi, Isik::Sari, Isik::Yesil] {
        let davranis = match isik {
            Isik::Kirmizi => "dur",
            Isik::Sari => "hazirlan",
            Isik::Yesil => "gec",
        };
        println!("{:?}: {} ({} sn)", isik, davranis, isik.saniye());
    }

    // -----------------------------------------------------------
    // 4) durum makinesi
    // -----------------------------------------------------------
    let mut isik = Isik::Kirmizi;
    print!("dongu: ");
    for _ in 0..5 {
        print!("{:?} -> ", isik);
        isik = isik.sonraki();
    }
    println!("{:?}", isik);

    // -----------------------------------------------------------
    // 5) varyantlar veri tasir - hepsi ayni Vec'in icinde durabiliyor
    // -----------------------------------------------------------
    let sekiller = vec![
        Sekil::Nokta,
        Sekil::Cember { r: 2.0 },
        Sekil::Dikdortgen { en: 3.0, boy: 4.0 },
        Sekil::Ucgen(3.0, 4.0, 5.0),
    ];
    let mut toplam = 0.0;
    for s in &sekiller {
        println!("{:<34} alan = {:.2}", format!("{:?}", s), s.alan());
        toplam += s.alan();
    }
    println!("toplam alan = {:.2}", toplam);

    // -----------------------------------------------------------
    // 6) gecersiz durum temsil edilemez
    //    metinle olsaydi: let baz = "X";  -> derlenir, sessizce sacmalar
    //    let baz = Baz::X;                // E0599 no variant named `X`
    // -----------------------------------------------------------
    let dizi = [Baz::A, Baz::T, Baz::G, Baz::G, Baz::C, Baz::A];
    print!("dizi        : ");
    for b in &dizi {
        print!("{:?} ", b);
    }
    println!();
    print!("tamamlayici : ");
    for b in &dizi {
        print!("{:?} ", b.tamamlayici());
    }
    println!();

    let mut gc = 0;
    for b in &dizi {
        if *b == Baz::G || *b == Baz::C {
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
    println!("{:?} {:?}", ilk_negatif(&olcumler), ilk_negatif(&[1, 2, 3]));

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
        Kullanici { id: 1, ad: String::from("Ada"), ikinci_isim: Some(String::from("Lovelace")), can: 100 },
        Kullanici { id: 2, ad: String::from("Ege"), ikinci_isim: None, can: 0 },
    ];

    for k in &kayitlar {
        // id: u64        -> her zaman var
        // ikinci_isim    -> olmayabilir, o yuzden Option<String>
        // can: i32       -> eksik olamaz ama 0 OLABILIR (0 ile "yok" ayni sey degil)
        match &k.ikinci_isim {
            Some(i) => println!("#{} {} {} (can {})", k.id, k.ad, i, k.can),
            None => println!("#{} {} (ikinci isim yok, can {})", k.id, k.ad, k.can),
        }
    }

    // uzunluk her zaman vardir -> usize, Option degil
    println!("kayit sayisi = {}", kayitlar.len());

    // arama BULAMAYABILIR -> Option<&Kullanici>
    match ara(&kayitlar, 2) {
        Some(k) => println!("bulundu: {}", k.ad),
        None => println!("bulunamadi"),
    }
    match ara(&kayitlar, 99) {
        Some(k) => println!("bulundu: {}", k.ad),
        None => println!("99 numarali kayit bulunamadi"),
    }
    // C#/C++ tarafinda ayni fonksiyon Kullanici dondururdu ve null gelebilecegi
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
    println!("Isik = {} bayt   Sekil = {} bayt", size_of::<Isik>(), size_of::<Sekil>());

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
    println!("{} {}", HttpDurum::Tamam as i32, HttpDurum::Bulunamadi as i32);
}
