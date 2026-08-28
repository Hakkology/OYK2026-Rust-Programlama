// Gun 7 / Ders 1 - Akilli Isaretciler: Box, Rc, RefCell
// rustc main.rs && ./main
//
// Tek bir dunya, dort tip:
//   Lead       -> ipucu zinciri   (Box)
//   MyBox<T>   -> kendi kutumuz   (Deref)
//   CaseFile   -> dosya           (Drop, Rc, RefCell)
//   Detective  -> dedektif        (Weak)

use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// ===============================================================
// 1) BOX - ozyinelemeli tip
// ===============================================================
// Box'siz: next: Option<Lead>  ->  E0072: recursive type has infinite size
// (boyut hesabi sonsuza gidiyor: Lead = String + Lead = ...)
struct Lead {
    text: String,
    next: Option<Box<Lead>>,
}

impl Lead {
    fn new(text: &str) -> Lead {
        Lead { text: text.to_string(), next: None }
    }

    // Bu ipucunu zincirin BASINA koyar.
    fn then(self, sonraki: Lead) -> Lead {
        Lead { text: self.text, next: Some(Box::new(sonraki)) }
    }

    fn chain(&self) -> String {
        match &self.next {
            Some(s) => format!("{} -> {}", self.text, s.chain()),
            None => self.text.clone(),
        }
    }
}

// ===============================================================
// 2) DEREF - Box'in sihri yok, sadece bir trait
// ===============================================================
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

// &str aliyor; &String ve &MyBox<String> de gecebiliyor (deref coercion).
fn duyur(metin: &str) -> String {
    format!("[ILAN] {}", metin)
}

// ===============================================================
// 3-5) CASEFILE - Drop, Rc ve RefCell ayni tip uzerinde
// ===============================================================
struct CaseFile {
    code: String,
    notes: RefCell<Vec<String>>,     // &self ile degisebilsin diye RefCell
}

impl CaseFile {
    fn new(code: &str) -> CaseFile {
        CaseFile { code: code.to_string(), notes: RefCell::new(Vec::new()) }
    }

    // DIKKAT: &self, &mut self DEGIL. Ic mutasyon budur.
    fn not_ekle(&self, not: &str) {
        self.notes.borrow_mut().push(not.to_string());
    }

    fn not_sayisi(&self) -> usize {
        self.notes.borrow().len()
    }
}

impl Drop for CaseFile {
    fn drop(&mut self) {
        println!("    [drop] {} dosyasi kapandi", self.code);
    }
}

// ===============================================================
// 6) WEAK - dosya dedektifi TUTAR, dedektif dosyaya SADECE BAKAR
// ===============================================================
struct Case {
    code: String,
    team: RefCell<Vec<Rc<Detective>>>,     // asagi dogru: Rc (sahiplik)
}

struct Detective {
    name: String,
    case: RefCell<Weak<Case>>,             // yukari dogru: Weak (sahiplik yok)
}

impl Drop for Case {
    fn drop(&mut self) {
        println!("    [drop] {} dosyasi kapandi", self.code);
    }
}

impl Drop for Detective {
    fn drop(&mut self) {
        println!("    [drop] {} evine gitti", self.name);
    }
}

// Ayni yapi, tek fark: geri baglanti da Rc -> DONGU
struct LeakyCase {
    code: String,
    team: RefCell<Vec<Rc<LeakyDetective>>>,
}

struct LeakyDetective {
    name: String,
    case: RefCell<Option<Rc<LeakyCase>>>,
}

impl Drop for LeakyCase {
    fn drop(&mut self) {
        println!("    [drop] {} dosyasi kapandi", self.code);
    }
}

impl Drop for LeakyDetective {
    fn drop(&mut self) {
        println!("    [drop] {} evine gitti", self.name);
    }
}

fn main() {
    println!("== 1) Box: ozyinelemeli zincir ==");
    let zincir = Lead::new("otoparktaki bilet")
        .then(Lead::new("plaka kaydi").then(Lead::new("gece bekcisinin ifadesi")));
    println!("  {}", zincir.chain());

    println!("== 2) Box: boyut ==");
    println!("  [u8; 4096]         {:>5} bayt", size_of::<[u8; 4096]>());
    println!("  Box<[u8; 4096]>    {:>5} bayt   <- icindeki ne olursa olsun bir pointer",
        size_of::<Box<[u8; 4096]>>());
    println!("  Option<Box<Lead>>  {:>5} bayt   <- Option bedava (Gun 4: niche)",
        size_of::<Option<Box<Lead>>>());

    println!("== 3) Deref: kutuyu acmak ==");
    let kutu = MyBox::new(String::from("dosya 47 acildi"));
    println!("  *kutu uzunlugu : {}", (*kutu).len());
    println!("  {}", duyur(&kutu));                     // &MyBox<String> -> &String -> &str
    println!("  {}", duyur(&String::from("dosya 48 acildi")));   // &String -> &str
    println!("  Gun 3'te 'parametrede &str al' demistik; sebebi bu zincir.");

    println!("== 4) Drop: kapsam bitince, TERS sirada ==");
    {
        let _a = CaseFile::new("47-A");
        let _b = CaseFile::new("47-B");
        println!("    iki dosya acik");
    }

    println!("== 5) Rc: paylasilan sahiplik ==");
    let dosya = Rc::new(CaseFile::new("KRG-12"));
    println!("  sayac: {}", Rc::strong_count(&dosya));
    let alvarez = Rc::clone(&dosya);                    // veri kopyalanmiyor, sayac artiyor
    println!("  sayac: {}   <- Alvarez de bakiyor", Rc::strong_count(&dosya));
    {
        let _gece_vardiyasi = Rc::clone(&dosya);
        println!("  sayac: {}   <- gece vardiyasi da acti", Rc::strong_count(&dosya));
    }
    println!("  sayac: {}   <- vardiya bitti", Rc::strong_count(&dosya));

    // hatırlatırım, mut metoda immmut veri gönderiyoruz. refcell yapıyor bunu.
    println!("== 6) RefCell: `mut` olmayan dosyaya not eklemek ==");
    dosya.not_ekle("tanik saat 23:40 diyor");
    alvarez.not_ekle("kamera kaydi 23:38'de kesiliyor");
    println!("  {} dosyasinda {} not var", dosya.code, dosya.not_sayisi()); // acaba rc counter 1 den fazla olsa da yapabilir miydi ?
    println!("  `dosya` mut degil - degisen sey RefCell'in ICI.");

    println!("== 6b) RefCell kurali CALISMA zamaninda ==");
    let hucre = RefCell::new(5);
    *hucre.borrow_mut() += 10;
    let ilk = hucre.borrow_mut();                       // tek yazici
    match hucre.try_borrow_mut() {
        Ok(_) => println!("  ikinci borrow_mut kabul edildi"),
        Err(_) => println!("  ikinci borrow_mut REDDEDILDI (already borrowed)"),
    }
    drop(ilk);
    println!("  ilki birakildi, deger: {}", hucre.borrow());
    println!("  borrow_mut yazsaydik PANIC ederdi. &mut olsaydi E0499 - derleme zamaninda.");

    // Rc sınırsız sayıda sahibin aynı veriyi paylaşmasını ($N$ sahip), 
    // RefCell ise bu sahiplerden aynı anda yalnızca birinin yazabilmesini (en fazla 1 aktif borrow_mut) denetler. 
    // İki sayaç birbirinden tamamen bağımsızdır.

    println!("== 6c) Cell: RefCell'in ucuz kardesi ==");
    let ziyaret = Cell::new(0u32);
    ziyaret.set(ziyaret.get() + 1);
    ziyaret.set(ziyaret.get() + 1);
    println!("  sayac {} | Cell deger kopyalar, RefCell referans verir", ziyaret.get());
    // Aynen öyle. Primitive (i32, bool, f64 vb.) veya küçük Copy tipler için Cell, 
    // Vec veya String gibi referans (&/&mut) ile çalışılması zorunlu tipler için RefCell tercih edilir.

    println!("== 6d) sayac sifira inince drop ==");
    drop(alvarez);
    println!("  Alvarez birakti, sayac: {}", Rc::strong_count(&dosya));
    drop(dosya);                                        // son sahip de gitti

    println!("== 7) Weak: dongu kurmadan geri baglanti ==");
    {
        let dava = Rc::new(Case {
            code: String::from("LMN-8"),
            team: RefCell::new(Vec::new()),
        });
        let dedektif = Rc::new(Detective {
            name: String::from("Alvarez"),
            case: RefCell::new(Rc::downgrade(&dava)),
        });
        dava.team.borrow_mut().push(Rc::clone(&dedektif));
        println!("  dava sayaci: strong {} / weak {}",
            Rc::strong_count(&dava), Rc::weak_count(&dava));
        // Weak sahiplenmez -> hedef dusmus olabilir -> upgrade() Option doner
        // borrow() gecici bir Ref uretir; once bir degiskene alalim.
        let baglanti = dedektif.case.borrow().upgrade();
        match baglanti {
            Some(d) => println!("  Alvarez'in dosyasi: {}", d.code),
            None => println!("  dosya kapanmis"),
        }
    }
    println!("  blok bitti, iki drop da calisti (yukarida).");

    println!("== 7b) ayni yapi Rc ile: SIZINTI ==");
    {
        let dava = Rc::new(LeakyCase {
            code: String::from("KRG-99"),
            team: RefCell::new(Vec::new()),
        });
        let dedektif = Rc::new(LeakyDetective {
            name: String::from("Kaya"),
            case: RefCell::new(None),
        });
        dava.team.borrow_mut().push(Rc::clone(&dedektif));      // asagi: Rc
        *dedektif.case.borrow_mut() = Some(Rc::clone(&dava));   // yukari: Rc  <- DONGU
        println!("  sayaclar: dava {} / dedektif {}",
            Rc::strong_count(&dava), Rc::strong_count(&dedektif));
    }
    println!("  blok bitti - HIC DROP SATIRI YOK. Ikisi birbirini tutuyor: bellek sizdi.");
    println!("  Fark tek kelime: Weak yerine Rc.");
    // Weak yukarıya sahiplik kurmadığı için sayaçlar kilitlenmez, 
    // ana nesne (parent) yok olunca alt nesne (child) zincirleme olarak temizlenir.
}
