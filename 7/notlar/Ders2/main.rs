// Gun 7 / Ders 2 - Lifetime: neden var, nasil okunur
// rustc main.rs && ./main
//
// Ayni buro. Bugun tanik ifadelerinin uzerinde calisiyoruz:
// uzun metinleri KOPYALAMADAN, dilimleyerek.

// ---------------------------------------------------------------
// 1) SOMUT OMUR (concrete lifetime)
// ---------------------------------------------------------------
// Her degerin bir omru vardir: dogdugu satirda baslar,
// dustugu ya da tasindigi satirda biter.

// ---------------------------------------------------------------
// 2) SARKAN REFERANS
// ---------------------------------------------------------------
// fn latest_note_broken() -> &str {
//     let note = String::from("gece bekcisi 23:40 dedi");
//     &note
// }
//   E0106: referans girdisi yok -> donen neye baglanacak belli degil.
//   Cozum 'a eklemek degil, sahipligi dondurmek.
fn latest_note() -> String {
    String::from("gece bekcisi 23:40 dedi")
}

// ---------------------------------------------------------------
// 3) IKI GIRDI, HANGISI DONUYOR?
// ---------------------------------------------------------------
// fn longer_statement(a: &str, b: &str) -> &str   ->  E0106
// 'a demek: donen referans, iki girdinin KISA olani kadar yasar.
fn longer_statement<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Donus tek girdiye baglanabilir; _fallback donmedigi icin 'a almadi.
fn preferred<'a>(primary: &'a str, _fallback: &str) -> &'a str {
    primary
}

// ---------------------------------------------------------------
// 4) ELISION - neden cogu zaman 'a yazmiyoruz
// ---------------------------------------------------------------
// Kural 2: tek girdi omru varsa cikisa o atanir. Yazmaya gerek yok:
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// Yukaridakinin elision'siz hali - AYNI fonksiyon:
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// ---------------------------------------------------------------
// 6) SOMUT OMUR: sahiplik devri omru BITIRIR
// ---------------------------------------------------------------
fn file_away(s: String) {
    println!("  arsive kaldirildi: {}", s);
}   // s burada dusuyor - omru fonksiyonun sonunda bitti

fn main() {
    println!("-- 1) somut omur --");
    let statement = String::from("kirmizi bir araba hizla gecti");
    let slice = &statement[..7];              // omru statement'a bagli
    println!("  dilim: '{}'", slice);
    println!("  kaynak yerinde: '{}'", statement);
    // Ic kapsam: DEGERI disari tasimak serbest
    let outer;
    {
        let inner = String::from("otoparkta bir golge vardi");
        outer = inner.len();                   // uzunluk kopyalandi
    }
    println!("  ic kapsamdan tasinan uzunluk: {}", outer);

    // REFERANSI tasimak serbest degil - asagiyi yorumdan cikarin:
    // let outer_ref;
    // {
    //     let inner = String::from("otoparkta bir golge vardi");
    //     outer_ref = &inner;
    // }
    // println!("{}", outer_ref);
    //   E0597: inner blok bitince dustu, outer_ref onu gosteremez.

    println!("-- 1b) omru bitiren uc yol --");
    // (a) kapsam bitti
    {
        let temp = String::from("gecici tutanak");
        println!("  kapsam icinde: {}", temp);
    }   // temp dustu

    // (b) baska bir binding'e TASINDI
    let original = String::from("ilk tutanak");
    let moved = original;
    println!("  tasindi: {}", moved);
    // println!("{}", original);
    //   E0382: borrow of moved value - original'in omru tasima satirinda bitti

    // (c) fonksiyona DEGERLE gecildi
    let report = String::from("gunluk rapor");
    file_away(report);
    // println!("{}", report);
    //   E0382: omru cagri satirinda bitti, fonksiyon icinde dustu

    println!("-- 2) sarkan referans yerine sahiplik --");
    println!("  {}", latest_note());

    println!("-- 3) iki girdi, tek donus --");
    let a = String::from("tanik A: araba maviydi");
    let b = String::from("tanik B: kirmizi");
    println!("  uzun olan : {}", longer_statement(&a, &b));
    println!("  tercihli  : {}", preferred(&a, &b));

    // Omur KISITI burada goruluyor:
    let long_lived = String::from("tanik A: araba maviydi");
    let winner;
    {
        let short_lived = String::from("tanik B: kirmizi");
        winner = longer_statement(&long_lived, &short_lived);
        println!("  blok icinde kullanmak serbest: {}", winner);
    }
    // println!("{}", winner);
    //   E0597: 'a KISA olana esitlendi; blok bitince winner de gecersiz.

    println!("-- 4) elision --");
    let report = String::from("plaka kismen okunabiliyor");
    println!("  ilk kelime          : {}", first_word(&report));
    println!("  elision'siz ayni sey: {}", first_word_explicit(&report));

    println!("-- 5) NLL: omur son KULLANIMDA biter --");
    let mut leads = vec![String::from("otopark"), String::from("plaka")];
    let peek = &leads[0];                      // okuma odunci
    println!("  ilk ipucu: {}", peek);         // peek'in son kullanimi burasi
    leads.push(String::from("bekci"));         // artik yazma odunci alinabiliyor
    println!("  {} ipucu var", leads.len());
    // peek@i asagida kullansaydik E0502 alirdik: omurler cakisirdi.
}
