// Gun 2 / Lab - Borrow Checker Dojo
// rustc lab-2.rs && ./lab-2
//
// Her gorevde yorumlu kod DERLENMIYOR. Yorumu acin, hatayi okuyun,
// sonra ALTINDAKI fonksiyonu duzeltin. Amac hata mesajini tanimak.

// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan degisken/import uyarilari normal.
#![allow(unused)]

fn main() {
    dojo_1();
    dojo_2();
    dojo_3();
    dojo_4();
    dojo_5();
    dojo_6();
    dojo_7();
    dojo_8();
}

// ---------------------------------------------------------------------------
// DOJO 1 - E0382
// let s1 = String::from("merhaba");
// let s2 = s1;
// println!("{} {}", s1, s2);
//
// TODO: ikisi de yazdirilsin. Iki farkli cozum bulun, hangisi daha ucuz?
// ---------------------------------------------------------------------------
fn dojo_1() {
    println!("-- dojo 1 --");
    let s1 = String::from("merhaba");
    let s2 = s1;
    println!("{}", s2);
}

// ---------------------------------------------------------------------------
// DOJO 2 - E0382
// let ad = String::from("Ayse");
// selamla(ad);
// selamla(ad);
//
// TODO: selamla iki kez cagrilabilsin. Imzayi degistirmek serbest.
// ---------------------------------------------------------------------------
fn dojo_2() {
    println!("-- dojo 2 --");
    let ad = String::from("Ayse");
    selamla(ad);
}

fn selamla(a: String) {
    println!("merhaba {}", a);
}

// ---------------------------------------------------------------------------
// DOJO 3 - E0596
// let v = vec![1, 2, 3];
// v.push(4);
//
// TODO: push calissin.
// ---------------------------------------------------------------------------
fn dojo_3() {
    println!("-- dojo 3 --");
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

// ---------------------------------------------------------------------------
// DOJO 4 - E0499
// let mut v = vec![1, 2, 3];
// let a = &mut v;
// let b = &mut v;
// a.push(4);
// b.push(5);
//
// TODO: ikisi de calissin. Ipucu: oduncleri zamanda ayirin.
// ---------------------------------------------------------------------------
fn dojo_4() {
    println!("-- dojo 4 --");
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);
}

// ---------------------------------------------------------------------------
// DOJO 5 - E0502
// let mut v = vec![1, 2, 3];
// let ilk = &v[0];
// v.push(4);
// println!("{}", ilk);
//
// TODO: iki farkli cozum bulun.
//   1) oduncun son kullanimini one alin (NLL)
//   2) odunc yerine degeri kopyalayin
// Ikisi ayni sey mi? Hangisi her durumda calisir?
// ---------------------------------------------------------------------------
fn dojo_5() {
    println!("-- dojo 5 --");
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);
}

// ---------------------------------------------------------------------------
// DOJO 6 - E0106
// fn ilk_kelime() -> &String {
//     let s = String::from("merhaba dunya");
//     &s
// }
//
// TODO: fonksiyon calissin. Referans dondurmek yerine ne yapmali?
// ---------------------------------------------------------------------------
fn dojo_6() {
    println!("-- dojo 6 --");
    println!("(bos)");
}

// ---------------------------------------------------------------------------
// DOJO 7 - E0382 (kismi move)
// let kayit = (String::from("Ayse"), String::from("Ankara"));
// let ad = kayit.0;
// println!("{:?}", kayit);
//
// TODO: hem ad hem kayit kullanilabilsin.
// Bonus: kayit.1 hala erisilebilir mi? Deneyin.
// ---------------------------------------------------------------------------
fn dojo_7() {
    println!("-- dojo 7 --");
    let kayit = (String::from("Ayse"), String::from("Ankara"));
    println!("{} {}", kayit.0, kayit.1);
}

// ---------------------------------------------------------------------------
// DOJO 8 - fonksiyon imzasi secimi
// Asagidaki uc fonksiyonun imzasi EKSIK. Her biri icin dogru olani secin:
//   String  /  &String  /  &mut String
//
// TODO 8a: metni sadece okuyup uzunlugunu donduren
// TODO 8b: metnin sonuna ekleme yapan
// TODO 8c: metni tuketip buyuk harfe cevrilmis YENI bir String donduren
// ---------------------------------------------------------------------------
fn dojo_8() {
    println!("-- dojo 8 --");
    let mut s = String::from("merhaba");
    println!("{}", s.len());
    s.push_str(" dunya");
    println!("{}", s);
    println!("{}", s.to_uppercase());
}
