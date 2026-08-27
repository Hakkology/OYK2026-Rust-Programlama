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
//     &note                                    // note fonksiyon bitince dusuyor
// }
//   E0106: missing lifetime specifier
//   -> "bu referans nereden geliyor?" Girdi yok, cevap yok.
//   Cozum: sahipligi dondurun (String), referans degil.
fn latest_note() -> String {
    String::from("gece bekcisi 23:40 dedi")
}

// ---------------------------------------------------------------
// 3) IKI GIRDI, HANGISI DONUYOR?
// ---------------------------------------------------------------
// fn longer_statement(a: &str, b: &str) -> &str      // E0106
// Derleyici donen referansin a'dan mi b'den mi geldigini bilmiyor.
// 'a yazinca "ikisinin de en az bu kadar yasadigini" soylemis oluyoruz;
// donen referans da o kadar yasar - yani KISA olani kadar.
fn longer_statement<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Donus TEK bir girdiye baglanabilir. O zaman digerinin omru onemsizdir:
// dikkat: _fallback'e 'a yazmadik, cunku donmuyor.
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
    //   E0597: `inner` does not live long enough
    //   inner blok bitince dustu; outer_ref onu gostermeye devam edemez.

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
    //   E0597: `short_lived` does not live long enough
    //   'a ikisinin KISA olanina esitlendi; blok bitince winner de gecersiz.

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
