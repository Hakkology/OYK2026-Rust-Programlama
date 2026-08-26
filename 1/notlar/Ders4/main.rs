// Gun 1 / Ders 4 - Degiskenler ve Veri Tipleri
// rustc main.rs && ./main
// rustc -O main.rs && ./main

fn main() {
    // varsayilan degismezlik, mut istisna
    let x = 10;
    // ilk deger okunmadan uzerine yaziliyor - derleyici hakli olarak uyarir,
    // ama biz kasten gosteriyoruz: mut olan degisken yeniden atanabilir
    #[allow(unused_assignments)]
    let mut y = 10;
    y = 20;
    println!("{} {}", x, y);
    // x = 20;                          // E0384

    // const derleme zamani sabit, static tek adres, tip zorunlu
    const MAKS: u32 = 100;
    static AD: &str = "rust101";
    println!("{} {}", MAKS, AD);
    // const SAYI = 5;                  // missing type

    // shadowing yeni degisken yaratir, tip degistirebilir; mut degistiremez
    let d = 5;
    let d = d + 10;
    let d = format!("metin: {}", d);
    println!("{}", d);

    // golgeleme kapsam bitince biter
    let s = 30;
    {
        let s = 40;
        println!("{}", s);
    }
    println!("{}", s);

    // tamsayi aileleri, varsayilan i32, usize neden var
    let i8_var: i8 = 12;
    let u8_var: u8 = 255;
    let u16_var: u16 = 40;
    let i32_var = 30;
    let i64_var: i64 = 9_000_000_000;
    let u128_var: u128 = 45;
    let idx: usize = 8;
    println!("{} {} {} {} {} {} {}", i8_var, u8_var, u16_var, i32_var, i64_var, u128_var, idx);
    // let k: u8 = 300;                 // literal out of range

    println!("{} {}", i32::MIN, i32::MAX);
    println!("{} {}", i8::MIN, i8::MAX);

    // taban gosterimleri
    let decimal = 256;
    let hexadecimal = 0x100;
    let octal = 0o400;
    let binary = 0b1_0000_0000;
    let byte = b'A';
    println!("{} {} {} {} {}", decimal, hexadecimal, octal, binary, byte);

    // f64 varsayilan, char 4 bayt
    let f1 = 2.1;
    let f2: f32 = 2.454;
    let b1 = true;
    let c1 = 'Ö';
    println!("{} {} {} {}", f1, f2, b1, c1);

    // IEEE-754, Rust hatasi degil
    println!("{}", 0.1 + 0.2);
    println!("{}", 0.1 + 0.2 == 0.3);

    println!("{} {} {} {}",
        std::mem::size_of::<u8>(), std::mem::size_of::<i32>(),
        std::mem::size_of::<char>(), std::mem::size_of::<f64>());

    // ayni kod dev'de panic, release'te sarar - iki profille de calistirin
    let t: u8 = 255;
    // let tasan = t + 1;
    println!("{:?}", t.checked_add(1));
    println!("{}", t.saturating_add(1));
    println!("{}", t.wrapping_add(1));
    println!("{:?}", t.overflowing_add(1));

    // as asla hata vermez, sessizce veri kaybeder
    println!("{}", 300i32 as u8);
    println!("{}", -1i32 as u8);
    println!("{}", 3.99f64 as i32);
    println!("{}", 1e20f64 as i32);

    // From kayipsiz, TryFrom riskli ve Result doner
    println!("{}", u32::from(200u8));
    println!("{:?}", u8::try_from(300i32));
    println!("{:?}", u8::try_from(200i32));

    // tuple farkli tipler, indeksle erisim, destructuring
    let mut t1: (i32, f64, u8) = (500, 6.4, 1);
    t1.0 = 80;
    println!("{}", t1.0);

    let t2: (i32, f64, char) = (4, 5.2, 'Ö');
    print_tuple(t2);
    let (a1, b2, c2) = t2;
    println!("{} {} {}", a1, b2, c2);

    let birim = ();
    println!("{}", std::mem::size_of_val(&birim));

    // dizi ayni tip, uzunluk tipin parcasi
    let a = [1, 2, 3, 4, 5];
    println!("{} {}", a[0], a.len());

    let a: [i64; 6] = [11, 22, 33, 44, 55, 66];
    println!("{}", a[0]);

    let gunler = ["Pzt", "Sal", "Car", "Per", "Cum", "Cmt", "Paz"];
    println!("{}", gunler[6]);

    let mut b: [i32; 5] = [3; 5];
    b[2] = 7;
    println!("{:?}", b);

    // sabit indeks derleyici yakalar, degisken indekste get() -> Option
    // let z = b[10];
    println!("{:?} {:?}", b.get(10), b.get(2));

    // cikarim ileriye bakar
    let mut liste = Vec::new();
    liste.push(3u8);
    println!("{:?}", liste);

    // turbofish
    // let bos = Vec::new();            // E0282
    let bos1: Vec<i32> = Vec::new();
    let bos2 = Vec::<i32>::new();
    println!("{:?} {:?}", bos1, bos2);
    println!("{}", "42".parse::<i32>().unwrap());

    // &str pencere, String sahip
    // ilk deger okunmadan degisiyor - uyari dogru, gosterdigimiz sey de bu
    #[allow(unused_assignments)]
    let mut camp: &str = "OYK";
    camp = "OYK CAMP";
    println!("{}", camp);

    let bos_s = String::new();
    println!("{:?}", bos_s);

    #[allow(unused_assignments)]
    let mut icerik = String::from("OYK");
    icerik = 234423.to_string();
    println!("{:?}", icerik);

    let mut v2 = "OYK KIS KAMPI".replace("OYK", "GUZEL");
    v2.push(' ');
    v2.push_str("OYK");
    println!("{}", v2);
    println!("{}", v2.as_str());

    // split tembel, collect etmeden is olmaz
    println!("{:?}", v2.trim().split(' '));
    println!("{:?}", v2.trim().split(' ').collect::<Vec<&str>>());
    println!("{}", format!("test {}", 1));

    // len() bayt sayar, chars().count() karakter
    for k in ["ada", "sss", "İstanbul", "🦀"] {
        println!("{:<10} {:<3} {}", k, k.len(), k.chars().count());
    }

    // buyuk harfe cevirme dile baglidir, std dil bilmez - Turkce 'i' dogru cevrilmez
    println!("{}", 'i'.to_uppercase());
}

fn print_tuple(t: (i32, f64, char)) {
    println!("{} {} {}", t.0, t.1, t.2);
}
