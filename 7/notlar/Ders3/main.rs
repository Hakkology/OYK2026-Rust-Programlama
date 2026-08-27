// Gun 7 / Ders 3 - Struct'larda Lifetime, 'static, coklu omur
// rustc main.rs && ./main
//
// Tanik ifadesinin tam metni bir yerde duruyor. Ayristirici onu
// KOPYALAMADAN dilimliyor - ama o zaman metinden uzun yasayamaz.

// ---------------------------------------------------------------
// 1) REFERANS TUTAN STRUCT
// ---------------------------------------------------------------
// struct Transcript { source: &str }
//   E0106: missing lifetime specifier
//   -> "bu referans ne kadar yasiyor?" Struct'in cevabi olmali.
struct Transcript<'a> {
    source: &'a str,
}

// impl blogu da omru tanimlamak ZORUNDA: omur tipin parcasi.
impl<'a> Transcript<'a> {
    fn new(source: &'a str) -> Transcript<'a> {
        Transcript { source }
    }

    // Elision kural 3: &self varsa cikisa self'in omru atanir.
    // Yani asagidaki imza aslinda -> &'a str demek, yazmaya gerek yok.
    fn first_line(&self) -> &str {
        match self.source.find('\n') {
            Some(i) => &self.source[..i],
            None => self.source,
        }
    }

    fn find_quote(&self, keyword: &str) -> Option<&str> {
        self.source
            .lines()
            .find(|satir| satir.contains(keyword))
    }

    // IKI ayri omur: donen dilim SELF'ten, yeni kaynak DISARIDAN.
    // 'b: bu odunc alma ne kadar surecek. 'a: metnin omru.
    fn replace_source<'b>(&'b mut self, new_source: &'a str) -> &'b str {
        let previous = self.source;
        self.source = new_source;
        previous
    }
}

// ---------------------------------------------------------------
// 2) SAHIPLENEN SURUM - ayni is, baska takas
// ---------------------------------------------------------------
struct OwnedTranscript {
    source: String,
}

impl OwnedTranscript {
    fn new(source: &str) -> OwnedTranscript {
        OwnedTranscript { source: source.to_string() }   // KOPYALIYOR
    }
    fn first_line(&self) -> &str {
        match self.source.find('\n') {
            Some(i) => &self.source[..i],
            None => &self.source,
        }
    }
}

// Fonksiyon icinde String uretip ondan referans tutan struct DONDUREMEZSINIZ:
// fn build_broken() -> Transcript<'static> {
//     let text = String::from("ifade metni");
//     Transcript { source: &text }
// }
//   E0515: cannot return value referencing local variable `text`
//   Gun 2'deki sarkan referansin ta kendisi.
// Cozum: sahiplenen surumu dondurun.
fn build_owned() -> OwnedTranscript {
    OwnedTranscript::new("ifade metni\nikinci satir")
}

// ---------------------------------------------------------------
// 3) 'static'IN IKI ANLAMI
// ---------------------------------------------------------------
// (a) &'static T : bu REFERANS program boyu gecerli
static AGENCY: &str = "Gece Vardiyasi Burosu";     // ikilinin icinde duruyor

// (b) T: 'static : bu TIP odunc referans ICERMIYOR
// String bunu saglar - icinde baskasina ait referans yok.
fn archive<T: 'static>(item: T) -> T {
    item
}

// ---------------------------------------------------------------
// 4) COKLU OMUR PARAMETRESI
// ---------------------------------------------------------------
// Iki AYRI omur: donen dilim yalnizca primary'ye bagli, secondary bagimsiz.
// secondary'ye 'b vermek zorunda degildik (elision zaten verirdi); acikca
// yaziyoruz ki ikisinin AYRI omurler oldugu gorulsun.
fn cross_check<'a, 'b>(primary: &'a str, secondary: &'b str) -> (&'a str, bool) {
    let confirmed = secondary.contains(primary);
    (primary, confirmed)
}

fn main() {
    let case_text = String::from(
        "tanik A: araba maviydi\ntanik B: plaka 34 ile basliyordu\ntanik C: saat 23:40",
    );

    println!("-- 1) referans tutan struct --");
    let mut transcript = Transcript::new(&case_text);
    println!("  ilk satir : {}", transcript.first_line());
    match transcript.find_quote("plaka") {
        Some(q) => println!("  plaka gecen: {}", q),
        None => println!("  plaka gecmiyor"),
    }
    println!("  kaynak {} bayt, struct {} bayt (sadece pointer + uzunluk)",
        case_text.len(), std::mem::size_of::<Transcript>());

    println!("-- 2) iki omurlu metot --");
    let revision = String::from("tanik A duzeltme: araba lacivertti");
    let previous = transcript.replace_source(&revision);
    let previous_first = previous.lines().next().unwrap_or(previous);
    println!("  eski kaynagin ilk satiri: {}", previous_first);
    println!("  yeni ilk satir          : {}", transcript.first_line());

    println!("-- 3) sahiplenen surum --");
    let owned = build_owned();
    println!("  {}", owned.first_line());
    println!("  OwnedTranscript {} bayt (String: ptr + len + cap)",
        std::mem::size_of::<OwnedTranscript>());

    println!("-- 4) 'static iki anlami --");
    println!("  &'static str : {}", AGENCY);
    let boxed_note = archive(String::from("dosya arsive kaldirildi"));   // T: 'static
    println!("  T: 'static   : {}", boxed_note);
    // let local = String::from("gecici");
    // archive(&local);
    //   E0597: `local` does not live long enough
    //   &local'in tipi &'x String; 'x program boyu degil -> T: 'static saglanmiyor.

    println!("-- 5) coklu omur --");
    let claim = String::from("araba maviydi");
    {
        // b bloktan cikinca dusuyor; donen dilim claim'e bagli oldugu icin sorun yok
        let other_statement = String::from("ikinci tanik da araba maviydi diyor");
        let (quote, confirmed) = cross_check(&claim, &other_statement);
        println!("  '{}' dogrulandi mi: {}", quote, confirmed);
    }
    let (quote, confirmed) = cross_check(&claim, "alakasiz ifade");
    println!("  '{}' dogrulandi mi: {}", quote, confirmed);
}
