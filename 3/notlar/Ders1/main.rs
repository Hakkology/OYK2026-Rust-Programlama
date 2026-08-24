// Gun 3 / Ders 1 - Fonksiyonlarda Sahiplik
// rustc main.rs && ./main

fn main() {
    // parametreye gecen deger TASINIR, cagiran kaybeder
    let s = String::from("merhaba");
    yut(s);
    // println!("{}", s);               // E0382

    // Copy tipler tasinmaz, cagirilan kopya alir
    let n = 5;
    yut_sayi(n);
    println!("{}", n);

    // cozum denemesi 1 - geri dondur
    let s = String::from("merhaba");
    let s = al_ve_geri_ver(s);
    println!("{}", s);

    // zincir uzayinca dayanilmaz olur - her satir sadece iade icin
    let s = zincir(zincir(zincir(s)));
    println!("{}", s);

    // cozum denemesi 2 - tuple ile hem sonucu hem degeri geri ver, cirkin
    let s = String::from("merhaba dunya");
    let (s, uzunluk) = uzunluk_ve_geri_ver(s);
    println!("{} {}", s, uzunluk);

    // GERCEK COZUM - odunc al
    let s = String::from("merhaba dunya");
    println!("{}", uzunluk_odunc(&s));
    println!("{}", s);                  // s hala bizim

    // referans ucuzdur - bir adres kadar
    println!("&String = {} bayt", std::mem::size_of::<&String>());

    // & ile okuma - sahiplik gecmez
    let sayilar = vec![10, 20, 30];
    println!("{}", topla(&sayilar));
    println!("{:?}", sayilar);

    // * ile referansin gosterdigi degere in
    let x = 5;
    let r = &x;
    println!("{} {}", r, *r);
    println!("{}", *r + 1);             // aritmetikte * gerekli
    println!("{}", *r == 5);            // karsilastirmada otomatik da cozulur

    // &mut ile odunc alip degistir
    let mut m = String::from("merhaba");
    ekle(&mut m);
    println!("{}", m);

    let mut sayi = 10;
    ikiye_katla(&mut sayi);
    println!("{}", sayi);

    // cagri yerinde de &mut yazilir - okuyan v'nin degisecegini gorur
    let mut v = vec![3, 1, 2];
    sirala(&mut v);
    println!("{:?}", v);

    // mut parametre: sahipligi alir, kendi kopyasini degistirir
    let sahip = String::from("abc");
    yerel_degistir(sahip);
    // println!("{}", sahip);           // E0382 - sahiplik gitti

    // &mut parametre: sahiplik almaz, CAGIRANIN verisini degistirir
    let mut baska = String::from("abc");
    ekle(&mut baska);
    println!("{}", baska);

    // donus degeri de tasinir - yerel deger cagirana gecer
    let uretilen = uret();
    println!("{}", uretilen);

    // parametreden gelen referans geri dondurulebilir
    let liste = vec![7, 8, 9];
    println!("{}", ilk(&liste));
    println!("{:?}", liste);

    // &str parametre hem String hem sabit metin kabul eder
    let sahipli = String::from("merhaba");
    println!("{}", uzunluk_str(&sahipli));
    println!("{}", uzunluk_str("sabit metin"));

    // ayni kural listelerde: &[T] yazarsan Vec, dizi ve dilim ucu de gecer
    let vek = vec![3, 1, 4];
    let diz = [3, 1, 4];
    println!("{} {} {}", topla_dilim(&vek), topla_dilim(&diz), topla_dilim(&vek[1..3]));
    // fn topla_dilim(v: &Vec<i32>) yazsaydik dizi ve dilim gecmezdi

    // SAHIPLIGI ALAN FONKSIYON DEGERI DE DUSURUR
    // yut(s) donerken s'in verisi coktan birakilmisti - sahip fonksiyondu
    // odunc alan fonksiyon dusurmez, deger cagiranda kalir (yukaridaki uzunluk_odunc)

    // erken dusurmek: drop() sahipligi alir ve hicbir sey yapmadan biter
    let gecici = String::from("gecici veri");
    println!("{}", gecici);
    drop(gecici);
    // println!("{}", gecici);          // E0382 - sahipligi drop aldi
    println!("gecici dusuruldu, main devam ediyor");

    // drop METODUNU elle cagiramazsiniz - derleyici kapsam sonunda zaten cagiracak
    // let x = String::from("a");
    // x.drop();                        // E0040 explicit use of destructor method

    // imza bir sozlesmedir, icine bakmadan ne olacagini soyler
    //   fn f(s: String)      -> alir, geri vermez
    //   fn f(s: &String)     -> okur
    //   fn f(s: &mut String) -> okur ve degistirir
    //   fn f() -> String     -> uretir, size verir
}

fn yut(s: String) {
    println!("yut: {}", s);
}

fn yut_sayi(n: i32) {
    println!("yut_sayi: {}", n);
}

fn al_ve_geri_ver(s: String) -> String {
    println!("al_ve_geri_ver: {}", s);
    s
}

fn zincir(s: String) -> String {
    s
}

fn uzunluk_ve_geri_ver(s: String) -> (String, usize) {
    let u = s.len();
    (s, u)
}

fn uzunluk_odunc(s: &String) -> usize {
    s.len()
}

fn uzunluk_str(s: &str) -> usize {
    s.len()
}

fn topla_dilim(v: &[i32]) -> i32 {
    let mut t = 0;
    for n in v {
        t += n;
    }
    t
}

fn topla(v: &Vec<i32>) -> i32 {
    let mut t = 0;
    for n in v {
        t += n;
    }
    t
}

fn ilk(v: &Vec<i32>) -> &i32 {
    &v[0]
}

fn ekle(s: &mut String) {
    s.push_str(" dunya");
}

fn ikiye_katla(n: &mut i32) {
    *n *= 2;                            // hedefi degistirmek icin * gerekli
}

fn sirala(v: &mut Vec<i32>) {
    v.sort();
}

fn yerel_degistir(mut s: String) {
    s.push_str(" degisti");
    println!("yerel_degistir: {}", s);
}

fn uret() -> String {
    let s = String::from("uretildi");
    s
}
