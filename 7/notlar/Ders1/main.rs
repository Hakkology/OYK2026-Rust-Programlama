// Gun 7 / Ders 1 - Akilli Isaretciler: Box, Rc, RefCell
// rustc main.rs && ./main
//
// Dunya: gece vardiyasi dedektiflik burosu.
// Dosyalar, ipucu zincirleri, ayni dosyaya bakan birden cok dedektif.

use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// ---------------------------------------------------------------
// 1) BOX - SEBEP 1: OZYINELEMELI TIP
// ---------------------------------------------------------------
// Her ipucu bir sonrakine goturuyor. Box'siz:
//   next: Option<Lead>  ->  E0072: recursive type has infinite size
//   (boyut hesabi sonsuza gidiyor: Lead = String + Lead = ...)
struct Lead {
    note: String,
    next: Option<Box<Lead>>,
}

impl Lead {
    fn chain(&self) -> String {
        match &self.next {
            Some(sonraki) => format!("{} -> {}", self.note, sonraki.chain()),
            None => self.note.clone(),
        }
    }
}

// ---------------------------------------------------------------
// 3) DEREF - kendi akilli isaretcimiz
// ---------------------------------------------------------------
// Box'in sihri yok; Deref implemente eden bir tip.
struct CaseBox<T>(T);

impl<T> CaseBox<T> {
    fn new(x: T) -> CaseBox<T> {
        CaseBox(x)
    }
}

impl<T> Deref for CaseBox<T> {
    type Target = T;                       // Gun 6: associated type
    fn deref(&self) -> &T {
        &self.0
    }
}

// &str aliyor. &String, &CaseBox<String> da gecebiliyor: deref coercion.
fn announce(text: &str) -> String {
    format!("[ILAN] {}", text)
}

// ---------------------------------------------------------------
// 4) DROP - kapsam bitince temizlik (Gun 2'deki RAII)
// ---------------------------------------------------------------
struct Surveillance {
    target: String,
}

impl Drop for Surveillance {
    fn drop(&mut self) {
        println!("    [drop] {} takibi sonlandirildi", self.target);
    }
}

// ---------------------------------------------------------------
// 5-7) RC / REFCELL - paylasilan dosya
// ---------------------------------------------------------------
struct CaseFile {
    code: String,
    notes: RefCell<Vec<String>>,           // icerik `mut` olmadan degisebiliyor
}

impl CaseFile {
    fn new(code: &str) -> CaseFile {
        CaseFile { code: code.to_string(), notes: RefCell::new(Vec::new()) }
    }

    // DIKKAT: &self aliyor, &mut self degil. Ic mutasyon budur.
    fn add_note(&self, note: &str) {
        self.notes.borrow_mut().push(note.to_string());
    }

    fn note_count(&self) -> usize {
        self.notes.borrow().len()
    }
}

// ---------------------------------------------------------------
// 8) WEAK - dongusel referans
// ---------------------------------------------------------------
struct Department {
    name: String,
    detectives: RefCell<Vec<Rc<Detective>>>,     // asagi dogru: SAHIPLIK
}

struct Detective {
    name: String,
    department: RefCell<Weak<Department>>,       // yukari dogru: SAHIPLIK YOK
}

impl Drop for Department {
    fn drop(&mut self) {
        println!("    [drop] {} birimi kapandi", self.name);
    }
}

impl Drop for Detective {
    fn drop(&mut self) {
        println!("    [drop] {} evine gitti", self.name);
    }
}

// AYNI YAPI, geri baglanti Weak yerine Rc: DONGU olusuyor.
struct LeakyDept {
    name: String,
    members: RefCell<Vec<Rc<LeakyAgent>>>,
}

struct LeakyAgent {
    name: String,
    dept: RefCell<Option<Rc<LeakyDept>>>,     // Weak DEGIL -> sahiplik
}

impl Drop for LeakyDept {
    fn drop(&mut self) {
        println!("    [drop] {} birimi kapandi", self.name);
    }
}

impl Drop for LeakyAgent {
    fn drop(&mut self) {
        println!("    [drop] {} evine gitti", self.name);
    }
}

fn main() {
    println!("-- 1) Box: ozyinelemeli ipucu zinciri --");
    let chain = Lead {
        note: String::from("otoparktaki bilet"),
        next: Some(Box::new(Lead {
            note: String::from("plaka kaydi"),
            next: Some(Box::new(Lead { note: String::from("gece bekcisinin ifadesi"), next: None })),
        })),
    };
    println!("  {}", chain.chain());

    println!("-- 2) Box: boyutlar --");
    // Box her zaman tek bir pointer: icindeki ne olursa olsun 8 bayt.
    println!("  [u8; 4096]        {:>5} bayt", size_of::<[u8; 4096]>());
    println!("  Box<[u8; 4096]>   {:>5} bayt", size_of::<Box<[u8; 4096]>>());
    // Box asla null olamaz; o bosluk None icin kullaniliyor (Gun 4: niche).
    println!("  Option<Box<Lead>> {:>5} bayt", size_of::<Option<Box<Lead>>>());

    println!("-- 3) Deref ve deref coercion --");
    let boxed = CaseBox::new(String::from("dosya 47 acildi"));
    println!("  *boxed uzunlugu: {}", (*boxed).len());     // acik dereference
    println!("  {}", announce(&boxed));                    // &CaseBox<String> -> &String -> &str
    let owned = String::from("dosya 48 acildi");
    println!("  {}", announce(&owned));                    // &String -> &str
    // Gun 3'te "parametrede &str al, &String alma" demistik; sebebi bu zincir.

    println!("-- 4) Drop: kapsam bitince --");
    {
        let _t1 = Surveillance { target: String::from("Kordon Kafe") };
        let _t2 = Surveillance { target: String::from("Liman deposu") };
        println!("    iki takip suruyor");
    }   // ters sirada dusuyorlar: once _t2, sonra _t1
    let early = Surveillance { target: String::from("Tren gari") };
    drop(early);                                            // erken dusurmek serbest
    println!("    tren gari takibi manuel kapatildi");

    println!("-- 5) Rc: paylasilan sahiplik --");
    let file = Rc::new(CaseFile::new("47-B"));
    println!("  sayac: {}", Rc::strong_count(&file));
    let for_alvarez = Rc::clone(&file);                     // veri kopyalanmiyor, sayac artiyor
    println!("  sayac: {}", Rc::strong_count(&file));
    {
        let _for_night_shift = Rc::clone(&file);
        println!("  sayac (blok icinde): {}", Rc::strong_count(&file));
    }
    println!("  sayac (blok bitti): {}", Rc::strong_count(&file));

    println!("-- 6) RefCell: ic mutasyon --");
    // `file` mut degil, ama icindeki notlar degisiyor.
    file.add_note("tanik saat 23:40 diyor");
    for_alvarez.add_note("kamera kaydi 23:38'de kesiliyor");
    println!("  {} dosyasinda {} not var", file.code, file.note_count());

    println!("-- 6b) RefCell kurali CALISMA zamaninda --");
    let cell = RefCell::new(5);
    *cell.borrow_mut() += 10;
    println!("  deger: {}", cell.borrow());
    let first = cell.borrow_mut();                          // tek yazici
    match cell.try_borrow_mut() {                           // ikincisi reddediliyor
        Ok(_) => println!("  ikinci borrow_mut kabul edildi"),
        Err(_) => println!("  ikinci borrow_mut REDDEDILDI - already borrowed"),
    }
    drop(first);
    println!("  ilki birakildi, artik serbest: {}", cell.borrow());
    // try_borrow_mut yerine borrow_mut yazsaydik program PANIC ederdi.
    // &mut olsaydi ayni hatayi DERLEME zamaninda alirdik (E0499).

    println!("-- 6c) Cell: RefCell'in ucuz kardesi --");
    // Cell odunc saymaz, degeri kopyalar: panic riski yok, ama Copy tipler icin.
    let ziyaret = Cell::new(0u32);
    ziyaret.set(ziyaret.get() + 1);
    ziyaret.set(ziyaret.get() + 1);
    println!("  Cell sayac: {} (borrow yok, panic riski yok)", ziyaret.get());
    println!("  Cell: get/set ile deger kopyalar | RefCell: borrow ile REFERANS verir");

    println!("-- 7) Weak: dongu kurmadan geri baglanti --");
    let department = Rc::new(Department {
        name: String::from("Cinayet Masasi"),
        detectives: RefCell::new(Vec::new()),
    });
    let alvarez = Rc::new(Detective {
        name: String::from("Alvarez"),
        department: RefCell::new(Rc::downgrade(&department)),
    });
    department.detectives.borrow_mut().push(Rc::clone(&alvarez));

    println!("  birim sayaci  : strong {} / weak {}",
        Rc::strong_count(&department), Rc::weak_count(&department));
    // Weak sahiplenmiyor, o yuzden hedefi dusmus olabilir: upgrade() Option doner.
    let back = alvarez.department.borrow().upgrade();
    match back {
        Some(d) => println!("  Alvarez'in birimi: {}", d.name),
        None => println!("  birim kapanmis"),
    }
    println!("  program bitiyor, drop ciktilari:");
    drop(alvarez);
    drop(department);

    println!("-- 7b) ayni yapi Rc ile: SIZINTI --");
    {
        let dept = Rc::new(LeakyDept {
            name: String::from("Kacakcilik Masasi"),
            members: RefCell::new(Vec::new()),
        });
        let agent = Rc::new(LeakyAgent {
            name: String::from("Kaya"),
            dept: RefCell::new(None),
        });
        dept.members.borrow_mut().push(Rc::clone(&agent));      // asagi: Rc
        *agent.dept.borrow_mut() = Some(Rc::clone(&dept));      // yukari: Rc  <- DONGU
        println!("    sayaclar: birim {} / dedektif {}",
            Rc::strong_count(&dept), Rc::strong_count(&agent));
    }
    println!("    blok bitti - YUKARIDA HIC DROP SATIRI YOK.");
    println!("    Ikisi birbirini tutuyor, sayaclar sifira inmedi: bellek sizdi.");
    println!("    Weak surumunde (7) iki drop da calismisti. Fark tek satir.");
}
