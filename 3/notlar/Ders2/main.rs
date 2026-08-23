// Gun 3 / Ders 2 - Borrowing Kurallari
// rustc main.rs && ./main

fn main() {
    // TEK KURAL - ayni anda ya sinirsiz okuyucu YA DA tek yazici
    let s = String::from("merhaba");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{} {} {}", r1, r2, r3);

    // tek mutable odunc, yaninda baska hicbir odunc olamaz
    let mut m = String::from("merhaba");
    let w = &mut m;
    w.push_str(" dunya");
    println!("{}", w);

    // odunc almak icin degiskenin kendisi mut olmali
    // let sabit = vec![1, 2, 3];
    // sabit.push(4);                   // E0596 cannot borrow as mutable
    let mut degisken = vec![1, 2, 3];
    degisken.push(4);
    println!("{:?}", degisken);

    // iki mutable odunc olmaz
    let mut v = vec![1, 2, 3];
    // let a = &mut v;
    // let b = &mut v;                  // E0499
    // a.push(4);
    {
        let a = &mut v;
        a.push(4);
    }
    let b = &mut v;
    b.push(5);
    println!("{:?}", v);

    // okuma ve yazma oduncu bir arada olmaz
    let mut k = vec![1, 2, 3];
    // let ilk = &k[0];
    // k.push(4);                       // E0502
    // println!("{}", ilk);
    let ilk = k[0];                     // kopyaladik, odunc kalmadi
    k.push(4);
    println!("{} {:?}", ilk, k);

    // NLL - odunc SON KULLANIMINDA biter, kapsam sonunda degil
    let mut n = vec![1, 2, 3];
    let oku = &n[0];
    println!("{}", oku);                // oduncun son kullanimi burada
    n.push(4);                          // artik serbest
    println!("{:?}", n);

    // ayni kod, sira degisince derlenmez
    // (a) derlenir:  let r = &n[0]; println!("{}", r); n.push(9);
    // (b) derlenmez: let r = &n[0]; n.push(9); println!("{}", r);

    // push neden okuyucuyu bozar - kapasite dolunca veri TASINIR
    let mut buyuyen = Vec::with_capacity(2);
    buyuyen.push(1);
    buyuyen.push(2);
    println!("cap={} adres={:p}", buyuyen.capacity(), buyuyen.as_ptr());
    buyuyen.push(3);                    // kapasite doldu, yeni yer alindi
    println!("cap={} adres={:p}", buyuyen.capacity(), buyuyen.as_ptr());
    buyuyen.reserve(1_000_000);         // buyuk istek - yerinde buyutulemez
    println!("cap={} adres={:p}", buyuyen.capacity(), buyuyen.as_ptr());
    // DIKKAT: allocator bazen yerinde buyutur ve adres AYNI kalir.
    // Ders bu: adres degisebilir, garantisi yoktur - o yuzden referans tutmak yasak.

    // dongu icinde degistirme
    let sayilar = vec![1, 2, 3];
    // for x in &sayilar { sayilar.push(*x); }      // E0502
    let mut eklenecek = Vec::new();
    for x in &sayilar {
        eklenecek.push(*x * 10);        // once topla
    }
    let mut sonuc = sayilar.clone();
    sonuc.extend(eklenecek);            // sonra uygula
    println!("{:?}", sonuc);

    // bir elemani odunc almak TUMUNU kilitler
    let mut d = [1, 2, 3, 4];
    let e = &mut d[0];
    *e = 10;
    // println!("{:?}", d);             // burada olmaz, e hala yasiyor
    println!("{}", e);
    println!("{:?}", d);                // e bitti, serbest

    // iki elemani ayni anda &mut almak - split_at_mut diziyi ikiye boler
    let (sol, sag) = d.split_at_mut(2);
    sol[0] = 100;
    sag[0] = 200;
    println!("{:?}", d);

    // &mut degeri kendi de tasinabilir, kopyalanamaz
    let mut z = String::from("z");
    let birinci = &mut z;
    let ikinci = birinci;               // &mut Copy degil, TASINDI
    ikinci.push('!');
    // println!("{}", birinci);         // E0382
    println!("{}", ikinci);

    // ama fonksiyona tekrar tekrar verilebilir - her cagri YENIDEN odunc alir
    let mut tekrar = String::from("a");
    ekle(&mut tekrar);
    ekle(&mut tekrar);
    println!("{}", tekrar);

    // referansin referansi olur, otomatik cozulur
    let sayi = 42;
    let r = &sayi;
    let rr = &r;
    println!("{} {} {}", sayi, r, rr);
    println!("{}", **rr + 1);

    // fonksiyona odunc verirken de ayni kurallar isler
    let mut liste = vec![5, 3, 8, 1];
    println!("{}", en_buyuk(&liste));
    sirala(&mut liste);
    println!("{:?}", liste);

    // ayni anda hem &mut hem & gonderilemez
    // yaz_ve_oku(&mut liste, &liste);  // E0502

    // sarkan referans imkansiz
    // let sark = sarkan();             // E0106 missing lifetime specifier
    println!("{}", sarkan_degil());

    // parametreden gelen referans dondurulebilir - veri cagiranda yasiyor
    println!("{}", ilk_eleman(&liste));

    // veri yarisi icin uc kosul gerekir
    //   1. iki veya daha fazla erisim ayni veriye
    //   2. en az biri yazma
    //   3. erisimler eszamanli ve senkronize degil
    // Rust 2. ve 3. kosulun ayni anda saglanmasini derleme zamaninda engeller
}

fn ekle(s: &mut String) {
    s.push('b');
}

// fn sarkan() -> &String {
//     let s = String::from("yerel");
//     &s                               // E0106
// }

fn sarkan_degil() -> String {
    String::from("yerel")
}

fn ilk_eleman(v: &Vec<i32>) -> &i32 {
    &v[0]
}

fn en_buyuk(v: &Vec<i32>) -> i32 {
    let mut enb = v[0];
    for n in v {
        if *n > enb {
            enb = *n;
        }
    }
    enb
}

fn sirala(v: &mut Vec<i32>) {
    let n = v.len();
    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if v[j] > v[j + 1] {
                let g = v[j];
                v[j] = v[j + 1];
                v[j + 1] = g;
            }
        }
    }
}

// fn yaz_ve_oku(a: &mut Vec<i32>, b: &Vec<i32>) {
//     a.push(b[0]);
// }
