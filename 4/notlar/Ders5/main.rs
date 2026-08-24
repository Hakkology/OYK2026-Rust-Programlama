// Gun 4 / Ders 5 - Donguler ve iter Uclusu
// rustc main.rs && ./main

use std::collections::HashMap;

#[derive(Debug)]
struct Gezegen {
    ad: String,
    uydu: u32,
}

enum Sekil {
    Cember { r: f64 },
    Kare { kenar: f64 },
}

fn main() {
    // for her zaman bir iterator uzerinde doner
    for i in 0..3 {
        print!("{} ", i);
    }
    println!();

    // --- uc bicim, tek fark sahiplik ---
    let v = vec![10, 20, 30];

    // &v  -> iter()  -> &T, koleksiyon bizde kalir
    for x in &v {
        print!("{} ", x);
    }
    println!();
    println!("{:?} hala bizim", v);

    // acik yazimi ayni sey
    for x in v.iter() {
        print!("{} ", x);
    }
    println!();

    // &mut v -> iter_mut() -> &mut T, degistirmek icin * SART
    let mut m = vec![1, 2, 3];
    for x in &mut m {
        *x *= 2;
    }
    println!("{:?}", m);

    // v -> into_iter() -> T, koleksiyon TUKENIR
    let t = vec![String::from("a"), String::from("b")];
    for s in t {
        print!("{} ", s);               // s: String, sahipligi bizde
    }
    println!();
    // println!("{:?}", t);             // E0382 - t tasindi

    // en sik hata bu; duzeltmesi tek karakter: &t
    let t2 = vec![String::from("a"), String::from("b")];
    for s in &t2 {
        print!("{} ", s);
    }
    println!();
    println!("{:?} hala duruyor", t2);

    // --- &T ile calismak ---
    let sayilar = vec![5, 12, 8, 20];
    let mut toplam = 0;
    let mut buyuk = 0;
    for x in &sayilar {
        toplam += x;                    // otomatik cozuluyor
        if *x > 10 {                    // karsilastirmada * ile daha okunakli
            buyuk += 1;
        }
    }
    println!("toplam={} 10'dan buyuk={}", toplam, buyuk);

    // --- enumerate ---
    let sehirler = ["Ankara", "Izmir", "Konya"];
    for (i, sehir) in sehirler.iter().enumerate() {
        println!("{}. {}", i + 1, sehir);
    }

    // elle indeks yonetmenin gereksiz hali
    for i in 0..sehirler.len() {
        print!("{} ", sehirler[i]);
    }
    println!();

    // enumerate + iter_mut
    let mut e = vec![100, 100, 100];
    for (i, x) in e.iter_mut().enumerate() {
        *x += i as i32;
    }
    println!("{:?}", e);

    // --- metinde enumerate vs char_indices ---
    let kelime = "gül";
    for (i, k) in kelime.chars().enumerate() {
        print!("({},{}) ", i, k);       // kacinci HARF
    }
    println!();
    for (i, k) in kelime.char_indices() {
        print!("({},{}) ", i, k);       // kacinci BAYT
    }
    println!();

    // --- ters cevirme ---
    for x in sayilar.iter().rev() {
        print!("{} ", x);
    }
    println!();
    for (i, x) in sayilar.iter().enumerate().rev() {
        print!("{}:{} ", i, x);
    }
    println!();

    // --- kendi tiplerimizde gezinme: struct ve enum ---
    let mut gezegenler = vec![
        Gezegen { ad: String::from("Dunya"), uydu: 1 },
        Gezegen { ad: String::from("Mars"), uydu: 2 },
        Gezegen { ad: String::from("Neptun"), uydu: 14 },
    ];

    // okumak
    for g in &gezegenler {
        print!("{}({}) ", g.ad, g.uydu);
    }
    println!();

    // degistirmek - alanlara dogrudan yaziyoruz, * gerekmiyor
    for g in &mut gezegenler {
        g.uydu += 1;
    }
    println!("{:?}", gezegenler);

    // tuketmek - String alanini disari TASIMAK icin tek yol
    let mut adlar = Vec::new();
    for g in gezegenler {
        adlar.push(g.ad);               // sahipligi devraldik, kopya yok
    }
    // println!("{:?}", gezegenler);    // E0382 - liste tukendi
    println!("{:?}", adlar);

    // enum listesinde dongu + match yan yana
    let sekiller = vec![Sekil::Cember { r: 1.0 }, Sekil::Kare { kenar: 2.0 }];
    for s in &sekiller {
        let alan = match s {
            Sekil::Cember { r } => 3.14159 * r * r,
            Sekil::Kare { kenar } => kenar * kenar,
        };
        print!("{:.2} ", alan);
    }
    println!();

    // --- diger koleksiyonlar, ayni kural ---
    let mut harita: HashMap<&str, i32> = HashMap::new();
    harita.insert("elma", 3);
    harita.insert("armut", 5);

    let mut cift: Vec<(&&str, &i32)> = harita.iter().collect();
    cift.sort();                        // HashMap sirasiz, cikti icin sirala
    println!("{:?}", cift);

    for deger in harita.values_mut() {
        *deger += 1;
    }
    println!("{:?}", harita.get("elma"));
}
