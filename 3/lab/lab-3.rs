// Gun 3 / Lab - Kelime Frekans Sayaci
// rustc lab-3.rs && ./lab-3
//
// Vec + String + dilim + HashMap + gezinme + odunc tek programda.
// Her gorevde TODO'lari doldurun; ustundeki ORNEK nasil calistigini gosteriyor.

use std::collections::HashMap;

const METIN: &str = "Rust ogrenmek zor ama Rust yazmak keyifli
Rust guvenli ve Rust hizli
ogrenmek zaman alir ama deger";

fn main() {
    lab_1_frekans();
    lab_2_istatistik();
    lab_3_dilimler();
    lab_4_gezinme();
    lab_5_odunc();
}

// ---------------------------------------------------------------------------
// LAB 1 - Kelime frekansi
// ---------------------------------------------------------------------------
fn lab_1_frekans() {
    println!("-- lab 1 --");

    // ORNEK: kelimeleri ayirmak
    for kelime in METIN.split_whitespace() {
        print!("[{}]", kelime);
    }
    println!();
    println!();

    // TODO 1a: HashMap<&str, u32> ile her kelimenin frekansini sayin
    //          ipucu: *frekans.entry(kelime).or_insert(0) += 1;

    // TODO 1b: sonucu frekansa gore AZALAN siralayip yazdirin
    //          ipucu: Vec<(&str, u32)>'e toplayin, sonra sort_by
    //                 v.sort_by(|a, b| b.1.cmp(&a.1));
    //          not: sort_by'a verilen |a, b| ... bir closure.
    //               Simdilik bu kalibi oldugu gibi kullanin.

    // TODO 1c: sadece 1 kereden fazla gecen kelimeleri yazdirin

    // TODO 1d: buyuk/kucuk harf duyarsiz sayin
    //          "Rust" ve "rust" ayni kelime sayilsin
}

// ---------------------------------------------------------------------------
// LAB 2 - Metin istatistikleri
// ---------------------------------------------------------------------------
fn lab_2_istatistik() {
    println!("-- lab 2 --");

    println!("{} bayt", METIN.len());
    println!("{} char", METIN.chars().count());
    println!("{} satir", METIN.lines().count());

    // TODO 2a: toplam kelime sayisi
    // TODO 2b: farkli (tekrarsiz) kelime sayisi   -> HashSet
    // TODO 2c: en uzun kelime ve uzunlugu
    // TODO 2d: ortalama kelime uzunlugu (f64)
    // TODO 2e: her satirin kac kelime icerdigini yazdirin
    // TODO 2f: harf frekansi - hangi harf kac kez geciyor
    //          bosluk ve satir sonu sayilmasin
}

// ---------------------------------------------------------------------------
// LAB 3 - Dilimler
// ---------------------------------------------------------------------------
fn lab_3_dilimler() {
    println!("-- lab 3 --");

    let sayilar = vec![42, 7, 19, 88, 3, 55, 12, 91, 26, 64];
    println!("{:?}", sayilar);

    // ORNEK: ilk uc eleman
    println!("{:?}", &sayilar[0..3]);

    // TODO 3a: fn ortalama(s: &[i32]) -> f64 yazin
    //          hem Vec hem dizi hem dilim ile cagirin
    // TODO 3b: fn en_buyuk_ikili(s: &[i32]) -> (i32, i32) yazin
    //          en buyuk iki degeri dondursun
    // TODO 3c: diziyi ikiye bolup her yarinin ortalamasini yazdirin
    //          ipucu: split_at
    //          not: teke bolunemezse ne yapacaksiniz?
    // TODO 3d: fn tekrar_var_mi(s: &[i32]) -> bool
    //          ipucu: HashSet
    // TODO 3e: ardisik ikililerin farkini yazdirin
    //          ipucu: windows(2)
}

// ---------------------------------------------------------------------------
// LAB 4 - Gezinme: iter / iter_mut / into_iter ve enumerate
// ---------------------------------------------------------------------------
fn lab_4_gezinme() {
    println!("-- lab 4 --");

    // ORNEK: & ile okumak - liste bizde kalir
    let sayilar = vec![4, 8, 15, 16];
    for s in &sayilar {
        print!("{} ", s);
    }
    println!();
    println!("{:?} hala duruyor", sayilar);

    // TODO 4a: iter_mut ile her elemani 2 katina cikarin, sonra listeyi yazdirin
    //          ipucu: for x in &mut liste { *x ... }

    // TODO 4b: enumerate ile su bicimde yazdirin:  "1. sehir: Ankara"
    //          numaralar 1'den bassin
    //          let sehirler = vec!["Ankara", "Izmir", "Konya"];

    // TODO 4c: asagidaki blok DERLENMIYOR. Once neden oldugunu soyleyin,
    //          sonra TEK karakter ekleyerek duzeltin.
    //   let isimler = vec![String::from("ada"), String::from("ege")];
    //   for i in isimler {
    //       print!("{} ", i);
    //   }
    //   println!("{:?}", isimler);

    // TODO 4d: into_iter'in gercekten gerektigi durum:
    //          isimler listesini TUKETEREK her elemani buyuk harfe cevrilmis
    //          yeni bir Vec<String>'e toplayin
    //          soru: burada &isimler ile yapsaniz ne degisirdi?
}

// ---------------------------------------------------------------------------
// LAB 5 - Odunc alma
// Her gorevde yorumlu kod DERLENMIYOR. Yorumu acin, hatayi okuyun, sonra duzeltin.
// ---------------------------------------------------------------------------
fn lab_5_odunc() {
    println!("-- lab 5 --");

    // GOREV 5a - E0502: dongu icinde degistirme
    // let mut liste = vec![1, 2, 3];
    // for x in &liste {
    //     if x % 2 == 1 {
    //         liste.push(x * 10);
    //     }
    // }
    //
    // TODO: for'un actigi odunc ne zaman kapaniyor?
    //       Calisan halini yazin. Ipucu: eklenecekleri ayri bir Vec'te
    //       toplayin, dongu bittikten sonra ekleyin.
    let liste = vec![1, 2, 3];
    println!("{:?}", liste);

    // GOREV 5b - E0507: Vec'ten tasima
    // let kayitlar = vec![String::from("ali"), String::from("veli")];
    // let ilk = kayitlar[0];
    // println!("{} {:?}", ilk, kayitlar);
    //
    // TODO: uc farkli cozum bulun - odunc alin / kopyalayin / gercekten cikarin.
    //       Hangisinde kayitlar eksiksiz kaliyor, hangisinde kisaliyor?
    let kayitlar = vec![String::from("ali"), String::from("veli")];
    println!("{:?}", kayitlar);

    // GOREV 5c - imza secimi: mut parametre mi, &mut parametre mi?
    // Asagidaki fonksiyon calisiyor ama cagiranin verisi DEGISMIYOR.
    //
    // TODO: etiketle fonksiyonunun imzasini ve cagrisini oyle degistirin ki
    //       "rapor" degiskeninin kendisi degissin.
    let rapor = String::from("gunluk rapor");
    etiketle(rapor);
    // println!("{}", rapor);           // simdilik burasi da E0382 veriyor
}

fn etiketle(mut s: String) {
    s.push_str(" [okundu]");
    println!("{}", s);
}
