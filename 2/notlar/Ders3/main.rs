// Gun 2 / Ders 3 - Fonksiyonlarda Sahiplik
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

    // cozum 1 - geri dondur, ama zincir uzayinca dayanilmaz
    let s = String::from("merhaba");
    let s = al_ve_geri_ver(s);
    println!("{}", s);

    // cozum 2 - tuple ile hem sonucu hem degeri geri ver, cirkin
    let s = String::from("merhaba dunya");
    let (s, uzunluk) = uzunluk_ve_geri_ver(s);
    println!("{} {}", s, uzunluk);

    // cozum 3 - ODUNC AL. Referanslar tam olarak bunun icin var.
    let s = String::from("merhaba dunya");
    println!("{}", uzunluk_odunc(&s));
    println!("{}", s);                  // s hala bizim

    // & referans alir, sahiplik gecmez
    let sayilar = vec![10, 20, 30];
    println!("{}", topla(&sayilar));
    println!("{:?}", sayilar);

    // * ile referansin gosterdigi degere in
    let x = 5;
    let r = &x;
    println!("{} {}", r, *r);
    println!("{}", *r + 1);

    // karsilastirmada otomatik cozulur, atama ve aritmetikte elle gerekir
    println!("{}", *r == 5);

    // &mut ile odunc alip degistir
    let mut m = String::from("merhaba");
    ekle(&mut m);
    println!("{}", m);

    let mut sayi = 10;
    ikiye_katla(&mut sayi);
    println!("{}", sayi);

    // fonksiyon parametresi mut olabilir - bu sahipligi degistirmez,
    // sadece fonksiyonun aldigi deger degistirilebilir olur
    let sahip = String::from("abc");
    yerel_degistir(sahip);

    // donus degeri de tasinir - yerel deger cagirana gecer
    let uretilen = uret();
    println!("{}", uretilen);

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

fn uzunluk_ve_geri_ver(s: String) -> (String, usize) {
    let u = s.len();
    (s, u)
}

fn uzunluk_odunc(s: &String) -> usize {
    s.len()
}

fn topla(v: &Vec<i32>) -> i32 {
    let mut t = 0;
    for n in v {
        t += n;
    }
    t
}

fn ekle(s: &mut String) {
    s.push_str(" dunya");
}

fn ikiye_katla(n: &mut i32) {
    *n *= 2;                            // hedefi degistirmek icin * gerekli
}

fn yerel_degistir(mut s: String) {
    s.push_str(" degisti");
    println!("yerel_degistir: {}", s);
}

fn uret() -> String {
    let s = String::from("uretildi");
    s
}
