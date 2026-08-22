// Gun 1 / Ders 5 - Fonksiyonlar ve Kontrol Akisi
// rustc main.rs && ./main

fn main() {
    // blok bir ifadedir, son satirda ; yoksa deger doner
    let sonuc = {
        let a = 10;
        let b = 20;
        a + b
    };
    println!("{}", sonuc);
    // let bos = { let a = 10; a + 1; };    // deger () olur

    // son ifade donus degeri, return erken cikis icin
    println!("{}", kare(7));
    println!("{}", mutlak(-5));
    println!("{}", topla(3, 4));
    selamla("Rust");
    // fn h(a, b) -> i32 { a + b }          // parametre tipi zorunlu
    // fn h2(x: i32) -> i32 { x * x; }      // E0308, ; degeri yutar

    // if bir ifade, ucluk operatoru yok, kollar ayni tip, kosul bool
    let sayi = 7;
    let durum = if sayi % 2 == 0 { "cift" } else { "tek" };
    println!("{}", durum);
    // let x = if sayi > 0 { 1 } else { "yok" };   // E0308
    // if sayi { }                                 // E0308

    let sinif = if sayi < 0 {
        "negatif"
    } else if sayi == 0 {
        "sifir"
    } else if sayi < 10 {
        "tek haneli"
    } else {
        "cok haneli"
    };
    println!("{}", sinif);

    // cok dalda match daha okunakli
    let grup = match sayi {
        0 => "sifir",
        1..=5 => "kucuk",
        6..=9 => "orta",
        _ => "buyuk",
    };
    println!("{}", grup);

    let sehir = match 6 {
        6 => "Ankara",
        34 => "Istanbul",
        _ => "diger",
    };
    println!("{}", sehir);

    // .. haric, ..= dahil; C tarzi for yok, her for bir iterator
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    for i in (0..=10).step_by(2) {
        print!("{} ", i);
    }
    println!();

    let sehirler = ["Ankara", "Izmir", "Konya"];
    for s in sehirler {
        print!("{} ", s);
    }
    println!();

    // indeks lazimsa enumerate
    for (i, s) in sehirler.iter().enumerate() {
        println!("{} {}", i, s);
    }

    let mut n = 3;
    while n > 0 {
        print!("{} ", n);
        n -= 1;
    }
    println!();

    for i in 1..10 {
        if i == 3 {
            continue;
        }
        if i == 6 {
            break;
        }
        print!("{} ", i);
    }
    println!();

    // sadece loop deger dondurebilir, while/for donduremez
    let mut sayac = 0;
    let ilk = loop {
        sayac += 1;
        if sayac * sayac > 50 {
            break sayac;
        }
    };
    println!("{}", ilk);

    // etiket ic ice donguden cikarir
    let mut bulundu = (0, 0);
    'dis: for i in 1..=5 {
        for j in 1..=5 {
            if i * j == 12 {
                bulundu = (i, j);
                break 'dis;
            }
        }
    }
    println!("{:?}", bulundu);
}

fn kare(x: i32) -> i32 {
    x * x
}

fn mutlak(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}

fn topla(a: i32, b: i32) -> i32 {
    a + b
}

fn selamla(ad: &str) {
    println!("Merhaba {}", ad);
}
