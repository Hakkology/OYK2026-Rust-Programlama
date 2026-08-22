// Gun 2 / Ders 1 - Stack ve Heap
// rustc main.rs && ./main

use std::mem::{size_of, size_of_val};

struct Iz(String);

impl Drop for Iz {
    fn drop(&mut self) {
        println!("drop: {}", self.0);
    }
}

fn selam(ad: &str) {
    println!("selam fonksiyonuna &str olarak geldi: {}", ad);
}

fn main() {
    // skaler tipler stack'te, boyut derleme zamaninda belli
    let a = 5i32;
    let b = 3.5f64;
    let c = 'x';
    let d = true;
    println!("i32={} f64={} char={} bool={}",
        size_of_val(&a), size_of_val(&b), size_of_val(&c), size_of_val(&d));

    // tamsayi ailesinin tamami - kac bit, kac bayt
    println!("i8={} i16={} i32={} i64={} i128={}",
        size_of::<i8>(), size_of::<i16>(), size_of::<i32>(), size_of::<i64>(), size_of::<i128>());
    println!("u8 araligi: {}..{}   i8 araligi: {}..{}",
        u8::MIN, u8::MAX, i8::MIN, i8::MAX);

    // usize makine kelimesi - adres ve uzunluklar hep bu tiple
    println!("usize={} bayt  (pointer da {} bayt)",
        size_of::<usize>(), size_of::<&i32>());

    // tip yazmayinca varsayilan i32 / f64 secilir
    let varsayilan = 7;
    let ondalik = 2.5;
    println!("varsayilan tamsayi={} bayt  ondalik={} bayt",
        size_of_val(&varsayilan), size_of_val(&ondalik));

    // tipler otomatik karismaz, as ile cevrilir
    let kucuk: u8 = 200;
    let genis: i64 = kucuk as i64;
    println!("u8 {} -> i64 {}", kucuk, genis);

    // tasma debug'da PANIKLER; bilerek sarmak icin wrapping_add
    println!("255u8.wrapping_add(1) = {}", 255u8.wrapping_add(1));

    // dizi de stack'te, uzunluk tipin parcasi oldugu icin boyut belli
    let dizi = [0u8; 100];
    println!("[u8; 100] = {} bayt", size_of_val(&dizi));

    // String stack'te 3 kelime tutar (ptr + len + cap), veri heap'te
    let s = String::from("merhaba");
    println!("len={} cap={} stack={} bayt", s.len(), s.capacity(), size_of_val(&s));

    // char 4 BAYT - bir harf degil, bir Unicode kod noktasi
    let turkce = 'ş';
    let ascii = 'A';
    println!("char 'ş'={} bayt  char 'A'={} bayt",
        size_of_val(&turkce), size_of_val(&ascii));

    // String UTF-8 BAYT dizisi - len() harf sayisi DEGIL
    let t = String::from("şğü");
    println!("\"şğü\" len()={} chars().count()={}", t.len(), t.chars().count());
    println!("ilk harf dilimi: {:?}", &t[0..2]);   // "ş" - iki bayt
    for (i, k) in t.chars().enumerate() {
        println!("  {}. karakter: {} ({} bayt)", i, k, k.len_utf8());
    }
    // t[0] burada DERLENMEZ: String indekslenemez (bir bayt bir harf degil)

    // ayni metin uc ayri yerde - literal, String, dilim
    let lit: &str = "merhaba";           // ikilinin icinde, salt okunur
    let own: String = lit.to_string();   // heap'te, sahibi var
    let dlm: &str = &own[0..3];          // baskasinin verisine bakan pencere
    println!("&str fat pointer={} bayt  String={} bayt  isaret ettigi veri={} bayt",
        size_of_val(&lit), size_of_val(&own), size_of_val(lit));
    println!("dilim: {}", dlm);
    selam(lit);    // &str gecer
    selam(&own);   // &String da &str'ye duser (deref coercion)

    // Vec de ayni ucluyu tutar - 24 bayt, icerik ne olursa olsun
    let v: Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("Vec len={} cap={} stack={} bayt", v.len(), v.capacity(), size_of_val(&v));

    // dizi vs Vec - biri tamamen stack'te, digeri stack + heap
    let d5: [i32; 5] = [1, 2, 3, 4, 5];
    let v5: Vec<i32> = vec![1, 2, 3, 4, 5];
    let dilim1: &[i32] = &d5[..];
    let dilim2: &[i32] = &v5[..];
    println!("[i32; 5]={} bayt  Vec<i32>={} bayt  &[i32]={} bayt",
        size_of_val(&d5), size_of_val(&v5), size_of_val(&dilim1));
    println!("iki dilim de ayni tip: {:?} {:?}", dilim1, dilim2);

    // Box ile bir degeri acikca heap'e tasi
    let kutu = Box::new(42i32);
    println!("Box degeri={} stack'teki boyut={} bayt", kutu, size_of_val(&kutu));

    // heap'te buyume - kapasite dolunca YENI yer alinir, veri tasinir
    let mut g = String::new();
    let mut son = g.capacity();
    println!("len={} cap={}", g.len(), g.capacity());
    for _ in 0..40 {
        g.push('x');
        if g.capacity() != son {
            println!("len={} cap={} adres={:p}", g.len(), g.capacity(), g.as_ptr());
            son = g.capacity();
        }
    }

    // tasinma adresten gorulur - E0502'nin sebebi bu
    // DIKKAT: allocator bazen yerinde buyutur ve adres AYNI kalir.
    // Ders bu: adres degisebilir, garantisi yoktur - o yuzden referans tutmak yasak.
    let mut adres = String::from("ab");
    println!("baslangic       : {:p}  cap={}", adres.as_ptr(), adres.capacity());
    adres.push_str("cdefghijklmnopqrstuvwxyz0123456789");
    println!("push_str sonrasi: {:p}  cap={}", adres.as_ptr(), adres.capacity());
    adres.reserve(1_000_000);
    println!("1 MB rezerv     : {:p}  cap={}", adres.as_ptr(), adres.capacity());

    // kapsam bitince deger duser, drop CAGRILMAZ - derleyici koyar
    {
        let _i1 = Iz(String::from("ic kapsam"));
        println!("ic kapsam icinde");
    }
    println!("ic kapsam bitti");

    // drop sirasi TERS - son tanimlanan once duser
    let _x = Iz(String::from("birinci"));
    let _y = Iz(String::from("ikinci"));
    let _z = Iz(String::from("ucuncu"));

    println!("main bitiyor");
}
