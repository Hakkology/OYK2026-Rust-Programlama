// Gun 7 / Lab - Kayip Kargo Dosyasi
// rustc lab-7.rs && ./lab-7
//
// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan uyarilari normal.
#![allow(unused)]
//
// SENARYO
// Gece vardiyasi burosunda yeni bir dosya acildi: limandan bir kargo kayboldu.
// Ipuclari zincir halinde ilerliyor, ayni dosyaya birden cok dedektif bakiyor,
// tanik ifadeleri uzun metinler ve bunlari KOPYALAMADAN islemek istiyoruz.
//
// Bugunun bes dersi burada sirayla kullaniliyor:
//   LAB 1 -> Box, Rc, RefCell, Weak            (Ders 1)
//   LAB 2 -> lifetime: neden var                (Ders 2)
//   LAB 3 -> struct'ta lifetime, 'static        (Ders 3)
//   LAB 4 -> closure temelleri                  (Ders 4)
//   LAB 5 -> closure'larla calismak             (Ders 5)

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
// LAB 1 - Akilli isaretciler
// ===========================================================================
// ORNEK: her ipucu bir sonrakine goturuyor.
struct Clue {
    text: String,
    next: Option<Box<Clue>>,
}

impl Clue {
    fn new(text: &str) -> Clue {
        Clue { text: text.to_string(), next: None }
    }
}

// TODO 1a: `next: Option<Box<Clue>>` yerine `next: Option<Clue>` yazmayi DENEYIN.
//          Hata kodu ne? Derleyici neden boyutu hesaplayamiyor, bir cumleyle yazin.

// TODO 1b: impl Clue icine `fn then(self, next: Clue) -> Clue` ekleyin:
//          zincirin sonuna yeni ipucu baglasin (self'i tuketip yeni Clue dondursun).
//          Sonra `fn chain(&self) -> String` yazin: "a -> b -> c" uretsin.

// TODO 1c: size_of ile yazdirin ve farki aciklayin:
//          [u8; 1024] / Box<[u8; 1024]> / Option<Box<Clue>>
//          Ucuncusu neden Box ile ayni boyutta?

// ORNEK: paylasilan dosya. `notes` RefCell, cunku `&self` ile not eklenecek.
struct CaseFile {
    code: String,
    notes: RefCell<Vec<String>>,
}

impl CaseFile {
    fn new(code: &str) -> CaseFile {
        CaseFile { code: code.to_string(), notes: RefCell::new(Vec::new()) }
    }
}

// TODO 1d: CaseFile'a su metotlari ekleyin:
//            fn add_note(&self, note: &str)     -> DIKKAT: &self, &mut self DEGIL
//            fn note_count(&self) -> usize
//          Sonra Rc::new(CaseFile::new("KRG-12")) olusturup iki dedektife
//          Rc::clone ile dagitin, ikisi de not eklesin.

// TODO 1e: Rc::strong_count'u uc yerde yazdirin: klonlamadan once, sonra,
//          ve bir ic blok bitince. Sayacin dustugunu gosterin.

// TODO 1f: `let r = Rc::new(String::from("x")); r.push_str("y");` deneyin.
//          Hata kodu ne? Rc neden degistirilemez?

// TODO 1g: RefCell kuralini calisma zamaninda kirin:
//            let c = RefCell::new(0);
//            let a = c.borrow_mut();
//            let b = c.borrow_mut();
//          Ne oldu? Ayni ihlali &mut ile yapsaydiniz ne zaman yakalanirdi?
//          Sonra ikinci borrow_mut'i try_borrow_mut ile degistirip panigi onleyin.

// ORNEK: birim -> dedektif SAHIPLIK, dedektif -> birim SAHIPLIK YOK
struct Squad {
    name: String,
    members: RefCell<Vec<Rc<Agent>>>,
}

struct Agent {
    name: String,
    squad: RefCell<Weak<Squad>>,
}

// TODO 1h: Squad ve Agent icin Drop yazip birer satir yazdirin.
//          Bir Squad ve bir Agent olusturup birbirine baglayin.
//          Program bitince iki drop da gorunuyor mu?
//          Sonra Weak yerine Rc koyun: drop ciktilari ne oldu? Neden?

fn lab_1_akilli_isaretciler() {
    println!("-- lab 1: akilli isaretciler --");
    let clue = Clue::new("limanda gece 02:10'da bir kamyon");
    println!("  ilk ipucu: {}", clue.text);
    // TODO: 1b-1h bitince ciktilarinizi buraya ekleyin
}

// ===========================================================================
// LAB 2 - Lifetime: neden var
// ===========================================================================
// TODO 2a: su fonksiyonu yazmayi deneyin, hata kodunu okuyun:
//            fn son_ifade() -> &str {
//                let s = String::from("kamyonun plakasi 34 ile basliyordu");
//                &s
//            }
//          Neden 'a eklemek COZUM DEGIL? Dogru cozumu yazin.

// TODO 2b: `fn uzun_olan(a: &str, b: &str) -> &str` yazin - derlenmeyecek.
//          Hata kodunu not edin, sonra 'a ekleyerek derletin.

// TODO 2c: 2b'deki fonksiyonu su sekilde cagirin:
//            uzun bir String disarida, kisa bir String ic blokta.
//            Sonucu blok ICINDE yazdirin  -> calisir
//            Sonucu blok DISINDA yazdirin -> hata
//          Hangi hatayi aldiniz? 'a hangi omre esitlendi?

// TODO 2d: `fn tercihli<'a>(birincil: &'a str, _yedek: &str) -> &'a str` yazin.
//          _yedek neden 'a almadi? Imza okuyana ne soyluyor?

// TODO 2e: `fn ilk_kelime(s: &str) -> &str` yazin - 'a YAZMADAN derlenecek.
//          Elision'in hangi kurali devrede? 2b neden ayni kuraldan yararlanamiyor?

// TODO 2f (NLL): bir Vec'ten `let ilk = &v[0];` alin, yazdirin, sonra v.push(...) yapin.
//          Calisiyor. Simdi push'tan SONRA ilk'i tekrar yazdirin. Hata kodu ne?

fn lab_2_lifetime() {
    println!("-- lab 2: lifetime --");
    // TODO: 2a-2f
}

// ===========================================================================
// LAB 3 - Struct'ta lifetime ve 'static
// ===========================================================================
// ORNEK: tanik ifadesinin tam metni baska yerde duruyor.
struct Statement<'a> {
    source: &'a str,
}

// TODO 3a: impl<'a> Statement<'a> yazin:
//            fn new(source: &'a str) -> Statement<'a>
//            fn first_line(&self) -> &str
//            fn quote_with(&self, keyword: &str) -> Option<&str>
//          first_line'da 'a yazmadiniz - hangi elision kurali isledi?

// TODO 3b: Statement'tan lifetime'i SILIN (struct Statement { source: &str }).
//          Hata kodu ne? Sonra geri koyun.

// TODO 3c: size_of::<Statement>() ile kaynak metnin uzunlugunu yan yana yazdirin.
//          Struct neden metin buyudukce buyumuyor?

// TODO 3d: sahiplenen surumu yazin: struct OwnedStatement { source: String }
//          Ayni iki metodu ekleyin. Iki surumun boyutlarini karsilastirin.
//          Hangisini kutuphane API'sinde dondururdunuz? Neden?

// TODO 3e: su fonksiyonu yazmayi deneyin:
//            fn uret() -> Statement<'static> {
//                let metin = String::from("ifade");
//                Statement { source: &metin }
//            }
//          Hata kodu ne? Bu Gun 2'deki hangi soruna denk geliyor?

// TODO 3f: `fn arsivle<T: 'static>(x: T) -> T` yazin.
//          String::from("...") ile cagirin -> calisir.
//          Yerel bir String'in referansiyla cagirin -> hata.
//          T: 'static "sonsuza kadar yasar" mi demek? Bir cumleyle yazin.

fn lab_3_struct_omru() {
    println!("-- lab 3: struct omru --");
    let text = String::from("tanik: kamyon lacivertti\nsofor uzun boyluydu");
    let statement = Statement { source: &text };
    println!("  kaynak {} bayt", statement.source.len());
    // TODO: 3a-3f
}

// ===========================================================================
// LAB 4 - Closure temelleri
// ===========================================================================
#[derive(Debug, Clone)]
struct Tip {
    text: String,
    weight: u8,          // 0-10 guvenilirlik
    source: String,      // muhbir
}

// TODO 4a: bir `esik` degiskeni tanimlayip onu YAKALAYAN bir closure yazin:
//            let guclu = |t: &Tip| t.weight >= esik;
//          Ayni seyi `fn` ile yazmayi deneyin. Hata kodu ne? Neden?

// TODO 4b: `fn suz<F>(ipuclari: &[Tip], kural: F) -> Vec<String> where F: Fn(&Tip) -> bool`
//          yazin. Neden generic? Iki closure ayni tip midir?

// TODO 4c: FnMut ornegi: `fn denetle<F>(ipuclari: &[Tip], mut kaydet: F) where F: FnMut(&Tip)`
//          yazin. Disarida bir sayac ve toplam tutup closure icinde artirin.

// TODO 4d: FnOnce ornegi: bir String'i `move` ile yakalayan ve onu TUKETEN
//          bir closure yazin. Iki kez cagirmayi deneyin. Hata kodu ne?

// TODO 4e: `move` yakalanan degiskeni tasir. move'lu bir closure'i IKI KEZ
//          cagirin - calisiyor mu? "move" ile "bir kez cagrilir" ayni sey mi?

// TODO 4f: size_of_val ile uc closure'in boyutunu yazdirin:
//            hicbir sey yakalamayan / bir u8 yakalayan / bir String yakalayan
//          Sonuclari closure'in "adsiz struct" olmasiyla aciklayin.

// TODO 4g: `fn say(ipuclari: &[Tip], kural: fn(&Tip) -> bool) -> usize` yazin.
//          Gercek bir fonksiyon geciriliyor mu? Yakalamayan closure?
//          Yakalayan closure? Sonuncusunun hata kodunu not edin.

fn lab_4_closure() {
    println!("-- lab 4: closure --");
    let ipuclari = vec![
        Tip { text: String::from("kamyon plakasi"), weight: 9, source: String::from("trafik") },
        Tip { text: String::from("isimsiz ihbar"), weight: 3, source: String::from("bilinmiyor") },
        Tip { text: String::from("liman kamerasi"), weight: 8, source: String::from("guvenlik") },
        Tip { text: String::from("kahvehane dedikodusu"), weight: 2, source: String::from("bilinmiyor") },
    ];
    println!("  {} ipucu var", ipuclari.len());
    // TODO: 4a-4g
}

// ===========================================================================
// LAB 5 - Closure'larla calismak
// ===========================================================================
// TODO 5a: `fn agirlik_kurali(esik: u8) -> impl Fn(&Tip) -> bool` yazin.
//          `move` neden zorunlu?

// TODO 5b: `fn kural_sec(mod_adi: &str) -> Box<dyn Fn(&Tip) -> bool>` yazin:
//          "siki" -> weight >= 8, "gevsek" -> weight >= 3, digeri -> hepsi.
//          Ayni seyi -> impl Fn ile YAZMAYI deneyin (closure'lar esik yakalasin).
//          Hata kodu ne? Gun 6'da hangi duvara benziyor?

// TODO 5c: closure'i struct icinde saklayin:
//            struct Filtre<F: Fn(&Tip) -> bool> { ad: String, kural: F }
//          `(self.kural)(t)` yazilisina dikkat - parantezsiz neden olmuyor?

// TODO 5d: farkli kurallari TEK listede tutun:
//            struct KuralDefteri { kurallar: Vec<(String, Box<dyn Fn(&Tip) -> bool>)> }
//          En az iki kural ekleyip HEPSINDEN gecen ipuclarini bulun (`all`).

// TODO 5e: kombinatorlerle su dort sonucu uretin:
//            - weight >= 5 olanlarin metinleri (filter + map + collect)
//            - toplam agirlik (map + sum)
//            - en guvenilir ipucu (max_by_key)
//            - agirliga gore azalan sirali liste (sort_by_key + Reverse)

// TODO 5f: `find` ile "plaka" gecen ipucunu bulun (Option doner). Sonra:
//            map / and_then / filter / unwrap_or_else / ok_or
//          besini de kullanip ciktilarini yazdirin.
//          unwrap_or ile unwrap_or_else farki nedir, bir cumleyle yazin.

fn lab_5_kural_motoru() {
    println!("-- lab 5: kural motoru --");
    // TODO: 5a-5f
}
