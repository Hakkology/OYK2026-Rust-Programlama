// Gun 7 / Ders 2 - Lifetime: neden var, nasil okunur
// rustc main.rs && ./main
//
// Ayni buro. Bugun tanik ifadelerinin uzerinde calisiyoruz:
// uzun metinleri KOPYALAMADAN, dilimleyerek.

// ---------------------------------------------------------------
// 1) SARKAN REFERANS
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

// fn latest_note() -> Box<str> {
//     Box::from("gece bekcisi 23:40 dedi") // Drop edilince bellek temizlenir
// }

// fn latest_note_leaked() -> &'static str {
//     let note = String::from("gece bekcisi 23:40 dedi");
//     Box::leak(note.into_boxed_str()) // Heap'teki alan asla serbest bırakılmaz!
// }

// ---------------------------------------------------------------
// 2) IKI GIRDI, HANGISI DONUYOR?
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
// 3) ELISION - uc kural, uc ornek
// ---------------------------------------------------------------
// Derleyici su uc kurali SIRAYLA uygular; cozulurse siz 'a yazmazsiniz.

// KURAL 1: referans olan HER parametre kendi omrunu alir.
//   yazdiginiz           : fn same_length(a: &str, b: &str) -> bool
//   derleyicinin gordugu : fn same_length<'a, 'b>(a: &'a str, b: &'b str) -> bool
// Referans DONMUYOR, is burada biter.
fn same_length(a: &str, b: &str) -> bool {
    a.len() == b.len()
}

fn same_length_explicit<'a, 'b>(a: &'a str, b: &'b str) -> bool {
    a.len() == b.len()
}

// KURAL 2: TEK girdi omru varsa, cikisa o atanir.
//   fn first_word(s: &str) -> &str  ==  fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// KURAL 3: parametrelerden biri &self ise, cikisa SELF'in omru atanir.
struct Casebook {
    title: String,
    entries: Vec<String>,
}

impl Casebook {
    //   fn title(&self) -> &str  ==  fn title<'a>(&'a self) -> &'a str
    fn title(&self) -> &str {
        &self.title
    }

    fn title_explicit<'a>(&'a self) -> &'a str {
        &self.title
    }

    // Iki referans girdi var (self ve keyword) ama kural 3 isliyor:
    // donen referans SELF'e bagli, keyword'e degil.
    fn find(&self, keyword: &str) -> Option<&String> {
        self.entries.iter().find(|e| e.contains(keyword))
    }
}

// KURALLAR YETMEZSE: iki girdi + referans donusu
//   fn longer_statement(a: &str, b: &str) -> &str  ->  E0106
// Kural 1 iki ayri omur verdi, kural 2 islemedi (tek girdi degil),
// kural 3 islemedi (&self yok). Geriye elle yazmak kaliyor.

// ---------------------------------------------------------------
// 4) SAHIPLIK DEVRI OMRU BITIRIR
// ---------------------------------------------------------------
fn file_away(s: String) {
    println!("  arsive kaldirildi: {}", s);
}   // s burada dusuyor - omru fonksiyonun sonunda bitti


fn main() {
    println!("-- 1) somut omur --");
    let statement = String::from("kirmizi bir araba hizla gecti");
    let slice = &statement[..7];              // omru statement'a bagli
    // statement.push_str("!!!");
    println!("  dilim: '{}'", slice);
    println!("  kaynak yerinde: '{}'", statement);

    // Copy tipleri: Move olmaz, i'nin omru bitmez
    let i = 5;
    let j = i; // i32 Copy trait'ini uygular, deger kopyalanir
    println!("  i: {i}, j: {j}");

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
    println!("  {}", latest_note()); // dangling referans zaten drop edilen bir yere işaret ediyor.

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

    println!("-- 4) elision: uc kural --");
    let report = String::from("plaka kismen okunabiliyor");
    let other = String::from("kamyon lacivertti mi");

    // Kural 1: referans donmuyor
    println!("  kural 1 | ayni uzunluk mu : {} = {}",
        same_length(&report, &other), same_length_explicit(&report, &other));

    // Kural 2: tek girdi -> cikisa o omur
    println!("  kural 2 | ilk kelime      : {} = {}",
        first_word(&report), first_word_explicit(&report));

    // Kural 3: &self -> cikisa self'in omru
    let defter = Casebook {
        title: String::from("KRG-12 kayit defteri"),
        entries: vec![
            String::from("23:38 kamera kesildi"),
            String::from("23:40 tanik beyani"),
        ],
    };
    println!("  kural 3 | baslik          : {} = {}",
        defter.title(), defter.title_explicit());
    {
        // keyword KISA yasiyor; donen referans self'e bagli oldugu icin sorun yok
        let keyword = String::from("tanik");
        println!("  kural 3 | arama           : {:?}", defter.find(&keyword));
    }
    println!("  kural 3 | defter yasiyor  : {} kayit", defter.entries.len());

    println!("-- 5) NLL: omur son KULLANIMDA biter --");
    let mut leads = vec![String::from("otopark"), String::from("plaka")];
    let peek = &leads[0];                      // okuma odunci
    println!("  ilk ipucu: {}", peek);         // peek'in son kullanimi burasi
    leads.push(String::from("bekci"));         // artik yazma odunci alinabiliyor
    println!("  {} ipucu var", leads.len());
    // peek'i asagida kullansaydik E0502 alirdik: omurler cakisirdi.
}