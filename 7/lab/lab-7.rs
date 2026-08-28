// Gun 7 / Lab - Kayip Kargo Dosyasi
// rustc lab-7.rs && ./lab-7
//
// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan uyarilari normal.
#![allow(unused)]
//
// NASIL CALISILIR
//   Her bolumde uc tur satir var:
//     ORNEK      -> calisan kod, size verildi. Once bunu calistirin.
//     GOREV      -> siz yazacaksiniz. Altinda BEKLENEN CIKTI var, ona ulasin.
//     HATA DENEYI-> kasten derlenmeyecek kod. Yorumu acin, hata KODUNU not edin,
//                   sebebini bir cumleyle yazin, sonra geri kapatin.
//
// SENARYO
// Limandan bir kargo kayboldu. Ipuclari zincir halinde, ayni dosyaya birden
// cok dedektif bakiyor, tanik ifadeleri uzun metinler.

use std::cell::RefCell;
use std::mem::size_of;
use std::rc::{Rc, Weak};

fn main() {
    lab_1_akilli_isaretciler();
    lab_2_lifetime();
    lab_3_struct_omru();
    lab_4_closure();
    lab_5_kural_motoru();
}

// ===========================================================================
// LAB 1 - Akilli isaretciler                                        (Ders 1)
// ===========================================================================

// ORNEK: ipucu zinciri. Box olmasa bu tip derlenmezdi.
struct Clue {
    text: String,
    next: Option<Box<Clue>>,
}

impl Clue {
    fn new(text: &str) -> Clue {
        Clue { text: text.to_string(), next: None }
    }

    // GOREV 1a: zincire yeni ipucu bagla. self'i TUKETIP yeni Clue dondur.
    //   Ipucu: Clue { text: ..., next: Some(Box::new(self)) }
    //   fn then(self, onceki: Clue) -> Clue { ... }

    // GOREV 1b: zinciri yazdiran metot.
    //   fn chain(&self) -> String
    //   BEKLENEN: "otoparktaki bilet -> plaka kaydi -> gece bekcisi"
    //   Ipucu: match &self.next { Some(s) => ..., None => ... }
}

// HATA DENEYI 1c: yukaridaki `next: Option<Box<Clue>>` yerine
//   `next: Option<Clue>` yazip derleyin.  Hata kodu: ______
//   Derleyici neden boyutu hesaplayamiyor?

// ORNEK: paylasilan dosya. notes RefCell, cunku &self ile not eklenecek.
struct CaseFile {
    code: String,
    notes: RefCell<Vec<String>>,
}

impl CaseFile {
    fn new(code: &str) -> CaseFile {
        CaseFile { code: code.to_string(), notes: RefCell::new(Vec::new()) }
    }

    // GOREV 1d: iki metot ekleyin. DIKKAT: ikisi de &self aliyor, &mut self DEGIL.
    //   fn add_note(&self, note: &str)
    //   fn note_count(&self) -> usize
    //   Ipucu: self.notes.borrow_mut().push(...)  /  self.notes.borrow().len()
}

// ORNEK: birim -> dedektif SAHIPLIK (Rc), dedektif -> birim SAHIPLIK YOK (Weak)
struct Squad {
    name: String,
    members: RefCell<Vec<Rc<Agent>>>,
}

struct Agent {
    name: String,
    squad: RefCell<Weak<Squad>>,
}

// GOREV 1h: Squad ve Agent icin Drop yazin, birer satir yazdirsin.
//   impl Drop for Squad  { fn drop(&mut self) { println!("  [drop] {} kapandi", self.name); } }
//   impl Drop for Agent  { ... }

fn lab_1_akilli_isaretciler() {
    println!("== LAB 1: akilli isaretciler ==");

    // ORNEK (calisiyor)
    let clue = Clue::new("otoparktaki bilet");
    println!("  ilk ipucu   : {}", clue.text);
    println!("  Box<Clue>   : {} bayt (icindekinin boyutu ne olursa olsun)",
        size_of::<Box<Clue>>());

    // GOREV 1e: uc ipucu zincirleyip chain() ile yazdirin.
    //   BEKLENEN: otoparktaki bilet -> plaka kaydi -> gece bekcisi

    // GOREV 1f: su ucunu yazdirin ve ucuncusunun neden Box ile AYNI oldugunu yazin:
    //   size_of::<[u8; 1024]>() / size_of::<Box<[u8; 1024]>>() / size_of::<Option<Box<Clue>>>()
    //   BEKLENEN: 1024 / 8 / 8

    // GOREV 1g: let file = Rc::new(CaseFile::new("KRG-12")); olusturun.
    //   Sayaci DORT yerde yazdirin:
    //     (1) hemen sonra                                  -> 1
    //     (2) let alvarez = Rc::clone(&file);  sonrasinda   -> 2
    //     (3) bir ic blok icinde ucuncu bir klon alip       -> 3
    //     (4) blok bittikten sonra                          -> 2
    //   Sonra file ve alvarez ile birer not ekleyip note_count yazdirin.
    //   BEKLENEN: "KRG-12 dosyasinda 2 not var"

    // HATA DENEYI 1i: Rc icindeki veriyi degistirmeyi deneyin.
    // let r = Rc::new(String::from("x"));
    // r.push_str("y");
    //   Hata kodu: ______   Rc neden degistirilemez?

    // HATA DENEYI 1j: RefCell kuralini CALISMA zamaninda kirin.
    // let c = RefCell::new(0);
    // let a = c.borrow_mut();
    // let b = c.borrow_mut();
    // println!("{} {}", a, b);
    //   Derleniyor mu? Calistirinca ne oldu? Mesaj: ______
    //   Ayni ihlali &mut ile yapsaydiniz ne zaman yakalanirdi?
    //   Sonra ikinci borrow_mut'i try_borrow_mut ile degistirip panigi onleyin.

    // GOREV 1k: bir Squad ve bir Agent olusturup birbirine baglayin
    //   (squad.members'a Rc::clone(&agent), agent.squad'a Rc::downgrade(&squad)).
    //   Rc::strong_count ve Rc::weak_count yazdirin.
    //   BEKLENEN: strong 1 / weak 1, ve program sonunda IKI drop satiri.
    //
    // GOREV 1l: simdi Agent.squad'i Weak yerine RefCell<Option<Rc<Squad>>> yapin,
    //   ayni baglantiyi kurun. Drop satirlari ne oldu? Neden?
}

// ===========================================================================
// LAB 2 - Lifetime: neden var                                       (Ders 2)
// ===========================================================================

// ORNEK: sahiplik donduren surum - hicbir omur sorunu yok.
fn latest_statement() -> String {
    String::from("gece bekcisi 23:40 dedi")
}

// HATA DENEYI 2a: ayni fonksiyonu REFERANS donduren hale getirin.
// fn latest_statement_broken() -> &str {
//     let s = String::from("gece bekcisi 23:40 dedi");
//     &s
// }
//   Hata kodu: ______
//   'a eklemek neden COZUM DEGIL? Dogru cozum ne?

// GOREV 2b: iki ifadeden uzun olani donduren fonksiyonu yazin.
//   Once 'a OLMADAN yazip derleyin -> hata kodu: ______
//   Sonra 'a ekleyip derletin.
//   fn longer_one<'a>(a: &'a str, b: &'a str) -> &'a str

// GOREV 2c: donusun SADECE birinci parametreye bagli oldugu bir fonksiyon yazin.
//   fn preferred<'a>(primary: &'a str, fallback: &str) -> &'a str
//   fallback neden 'a almadi? Imza okuyana ne soyluyor?

// GOREV 2d: elision - 'a YAZMADAN derlenen bir fonksiyon yazin.
//   fn first_word(s: &str) -> &str          // ilk bosluga kadar olan kisim
//   BEKLENEN: first_word("plaka kismen okunabiliyor") -> "plaka"
//   Hangi elision kurali isledi? 2b neden ayni kuraldan yararlanamiyor?

fn lab_2_lifetime() {
    println!("== LAB 2: lifetime ==");
    println!("  {}", latest_statement());

    // GOREV 2e: 2b'deki fonksiyonu su sekilde cagirin:
    //   uzun yasayan bir String disarida, kisa yasayan bir String ic blokta.
    //   Sonucu once blok ICINDE yazdirin  -> calisir
    //   sonra blok DISINDA yazdirin       -> hata kodu: ______
    //   'a hangi omre esitlendi? Disaridaki String hala yasarken bile neden olmuyor?

    // HATA DENEYI 2f: degeri degil REFERANSI ic kapsamdan disari tasiyin.
    // let outer_ref;
    // {
    //     let inner = String::from("gecici tutanak");
    //     outer_ref = &inner;
    // }
    // println!("{}", outer_ref);
    //   Hata kodu: ______

    // GOREV 2g (NLL): bir Vec olusturun, `let first = &v[0];` alin, yazdirin,
    //   sonra v.push(...) yapin. Calisiyor.
    //   Simdi push'tan SONRA first'i tekrar yazdirin -> hata kodu: ______
    //   Odunc kapsamin sonuna kadar mi surdu, son kullanima kadar mi?
}

// ===========================================================================
// LAB 3 - Struct'ta lifetime ve 'static                             (Ders 3)
// ===========================================================================

// ORNEK: tanik ifadesinin tam metni BASKA yerde duruyor, biz sadece gosteriyoruz.
struct Statement<'a> {
    source: &'a str,
}

// GOREV 3a: impl<'a> Statement<'a> yazin:
//   fn new(source: &'a str) -> Statement<'a>
//   fn first_line(&self) -> &str                 // ilk satir
//   fn quote_with(&self, keyword: &str) -> Option<&str>   // keyword gecen ilk satir
//   Ipucu: self.source.lines()
//   first_line'da 'a yazmadiniz - hangi elision kurali isledi?

// HATA DENEYI 3b: Statement'tan <'a> ve &'a'yi SILIN (struct Statement { source: &str }).
//   Hata kodu: ______   Sonra geri koyun.

// GOREV 3c: sahiplenen surumu yazin:
//   struct OwnedStatement { source: String }  + ayni iki metot
//   Ikisinin size_of'unu yan yana yazdirin.
//   BEKLENEN: Statement 16 bayt, OwnedStatement 24 bayt
//   Kaynak metin 10 KB olsaydi hangisi buyurdu?

// HATA DENEYI 3d: yerel bir String'den Statement uretip DONDURMEYI deneyin.
// fn build() -> Statement<'static> {
//     let text = String::from("ifade");
//     Statement { source: &text }
// }
//   Hata kodu: ______   Bu Gun 2'deki hangi soruna denk geliyor?

// GOREV 3e: fn archive<T: 'static>(x: T) -> T yazin.
//   archive(String::from("dosya")) -> calisir
//   yerel bir String'in REFERANSIYLA cagirin -> hata kodu: ______
//   T: 'static "sonsuza kadar yasar" mi demek? Bir cumleyle yazin.

fn lab_3_struct_omru() {
    println!("== LAB 3: struct omru ==");
    let text = String::from("tanik: kamyon lacivertti\nsofor uzun boyluydu");
    let statement = Statement { source: &text };
    println!("  kaynak {} bayt, struct {} bayt",
        text.len(), size_of::<Statement>());

    // GOREV 3f: 3a'yi bitirince first_line ve quote_with("sofor") ciktilarini yazdirin.
    //   BEKLENEN: "tanik: kamyon lacivertti" ve Some("sofor uzun boyluydu")
}

// ===========================================================================
// LAB 4 - Closure temelleri                                         (Ders 4)
// ===========================================================================
#[derive(Debug, Clone)]
struct Tip {
    text: String,
    weight: u8,          // 0-10 guvenilirlik
    source: String,      // muhbir
}

// GOREV 4b: closure alan bir fonksiyon yazin. Neden generic olmak zorunda?
//   fn filter_tips<F>(tips: &[Tip], rule: F) -> Vec<String>
//   where F: Fn(&Tip) -> bool

// GOREV 4c: FnMut alan bir fonksiyon yazin (parametre `mut` olmali):
//   fn audit<F>(tips: &[Tip], mut record: F) where F: FnMut(&Tip)

// GOREV 4g: fn pointer alan bir fonksiyon yazin:
//   fn count_matching(tips: &[Tip], rule: fn(&Tip) -> bool) -> usize

fn lab_4_closure() {
    println!("== LAB 4: closure ==");
    let tips = vec![
        Tip { text: String::from("kamyon plakasi"), weight: 9, source: String::from("trafik") },
        Tip { text: String::from("isimsiz ihbar"), weight: 3, source: String::from("bilinmiyor") },
        Tip { text: String::from("liman kamerasi"), weight: 8, source: String::from("guvenlik") },
        Tip { text: String::from("kahvehane dedikodusu"), weight: 2, source: String::from("bilinmiyor") },
    ];

    // ORNEK (calisiyor): closure cevredeki threshold'u YAKALIYOR
    let threshold = 5;
    let strong = |t: &Tip| t.weight >= threshold;
    println!("  esik {} -> {} guclu ipucu", threshold, tips.iter().filter(|t| strong(t)).count());

    // HATA DENEYI 4a: ayni seyi fonksiyonla yapin.
    // fn strong_fn(t: &Tip) -> bool { t.weight >= threshold }
    //   Hata kodu: ______   Fonksiyonun cevresi neden yok?

    // GOREV 4d (FnMut): disarida bir sayac ve toplam tutup 4c'deki audit ile doldurun.
    //   BEKLENEN: "4 ipucu, toplam agirlik 22"

    // GOREV 4e (FnOnce): bir String'i `move` ile yakalayip TUKETEN closure yazin
    //   (icinde String'i geri dondurun). Iki kez cagirmayi deneyin.
    //   Hata kodu: ______

    // GOREV 4f: `move`lu bir closure'i IKI KEZ cagirin - calisiyor mu?
    //   "move" ile "bir kez cagrilir" ayni sey mi? Bir cumleyle yazin.

    // GOREV 4h: uc closure'in boyutunu yazdirin (std::mem::size_of_val):
    //   hicbir sey yakalamayan / bir u8 yakalayan / bir String yakalayan
    //   BEKLENEN: 0 / 1 / 24
    //   Sonucu "closure adsiz bir struct'tir" cumlesiyle aciklayin.

    // HATA DENEYI 4i: 4g'deki fonksiyona YAKALAYAN bir closure gecirin.
    //   count_matching(&tips, |t| t.weight >= threshold);
    //   Hata kodu: ______   Yakalamayan closure gecti mi?
}

// ===========================================================================
// LAB 5 - Closure'larla calismak                                    (Ders 5)
// ===========================================================================

// GOREV 5a: closure DONDUREN fonksiyon yazin.
//   fn weight_rule(threshold: u8) -> impl Fn(&Tip) -> bool
//   `move` neden zorunlu? Kaldirip deneyin.

// GOREV 5b: calisma zamaninda kural secen fonksiyon yazin.
//   fn pick_rule(mode: &str) -> Box<dyn Fn(&Tip) -> bool>
//     "strict" -> weight >= 8 | "loose" -> weight >= 3 | digeri -> hepsi true
//   HATA DENEYI: ayni fonksiyonu -> impl Fn ile yazin (closure'lar threshold YAKALASIN).
//     Hata kodu: ______   Gun 6'da hangi duvara benziyor?

// GOREV 5c: closure'i struct icinde saklayin.
//   struct Screen<F: Fn(&Tip) -> bool> { name: String, rule: F }
//   apply metodunda `(self.rule)(t)` yazin - parantezleri kaldirinca ne oluyor?

// GOREV 5d: farkli kurallari TEK listede tutun.
//   struct RuleBook { rules: Vec<(String, Box<dyn Fn(&Tip) -> bool>)> }
//   Su iki kurali ekleyin:
//     "agirlik >= 3"  -> |t| t.weight >= 3
//     "muhbir belli"  -> |t| t.source != "bilinmiyor"
//   HEPSINDEN gecenleri bulun (rules.iter().all(...)).
//   BEKLENEN: ["kamyon plakasi", "liman kamerasi"]

fn lab_5_kural_motoru() {
    println!("== LAB 5: kural motoru ==");
    let tips = vec![
        Tip { text: String::from("kamyon plakasi"), weight: 9, source: String::from("trafik") },
        Tip { text: String::from("isimsiz ihbar"), weight: 3, source: String::from("bilinmiyor") },
        Tip { text: String::from("liman kamerasi"), weight: 8, source: String::from("guvenlik") },
        Tip { text: String::from("kahvehane dedikodusu"), weight: 2, source: String::from("bilinmiyor") },
    ];

    // ORNEK (calisiyor): tembellik. Adaptor kurulunca HICBIR SEY yazilmiyor.
    let zincir = tips.iter().map(|t| {
        println!("    >> {} isleniyor", t.text);
        t.weight
    });
    println!("  zincir kuruldu - yukarida satir var mi?");
    let toplam: u32 = zincir.map(|w| w as u32).sum();
    println!("  sum() cagrildi -> toplam {}", toplam);

    // GOREV 5e: kombinatorlerle su dort sonucu uretin:
    //   - weight >= 5 olanlarin metinleri        (filter + map + collect)
    //   - toplam agirlik                          (map + sum)   BEKLENEN: 22
    //   - en guvenilir ipucu                      (max_by_key)  BEKLENEN: "kamyon plakasi"
    //   - agirliga gore azalan liste              (sort_by_key + std::cmp::Reverse)

    // GOREV 5f: fold ile toplam agirligi TEKRAR hesaplayin.
    //   fold(0, |acc, t| ...) BEKLENEN: 22 - sum ile ayni.
    //   fold ile reduce farki nedir?

    // GOREV 5g: find ile "plaka" gecen ipucunu bulun (Option doner), sonra
    //   map / and_then / filter / unwrap_or_else / ok_or besini de kullanin.
    //   unwrap_or ile unwrap_or_else farkini bir cumleyle yazin.
}
