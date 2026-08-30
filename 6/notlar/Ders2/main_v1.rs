// Gun 6 / Ders 2 (v1) - Trait Tanimi, Varsayilan Metotlar ve Bound'lar
// rustc main_v1.rs && ./main_v1
//
// Ayni ders, farkli dunya: sehir ulasim agi.
// Vapur, metro, tramvay, otobus tamamen farkli seyler yapar ama hepsinin
// ortak bir SOZLESMESI vardir: bir hatti vardir, kapasitesi vardir, ucreti vardir.

use std::fmt::Debug;
use std::mem::size_of;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------
// 1) TRAIT = SOZLESME
// ---------------------------------------------------------------
trait Vehicle {
    // zorunlu: implemente eden herkes yazmak ZORUNDA
    fn line(&self) -> &str;
    fn capacity(&self) -> u32;
    fn fare(&self) -> u32;              // kurus

    // VARSAYILAN metot: govdesi burada. Isteyen ezer, istemeyen bedava alir.
    fn announce(&self) -> String {
        format!("{} hatti kalkiyor", self.line())
    }

    // varsayilan metot ZORUNLU metotlari cagirabilir
    fn is_large(&self) -> bool {
        self.capacity() >= 200
    }

    fn status(&self) -> String {
        let boy = if self.is_large() { "buyuk" } else { "kucuk" };
        format!("{:<20} {:>5} kisi  {:>5} krs  [{}]", self.line(), self.capacity(), self.fare(), boy)
    }
}

struct Ferry {
    line: String,
    deck_count: u8,
}

struct Metro {
    line: String,
    car_count: u8,
}

struct Tram {
    line: String,
}

struct Funicular;                       // alani olmayan tip de olur

// Trait metotlari ile TIPIN KENDI metotlari (inherent) bir arada yasar.
// Bu metot trait'e ait degil, sadece Ferry'de var.
impl Ferry {
    fn deck_report(&self) -> String {
        format!("{} guverte acik", self.deck_count)
    }
}

impl Vehicle for Ferry {
    fn line(&self) -> &str { &self.line }
    fn capacity(&self) -> u32 { 450 }
    fn fare(&self) -> u32 { 1750 }
    // announce() EZILMEDI - varsayilani kullaniyor
}

impl Vehicle for Metro {
    fn line(&self) -> &str { &self.line }
    fn capacity(&self) -> u32 { 220 * self.car_count as u32 }
    fn fare(&self) -> u32 { 1500 }
    fn announce(&self) -> String {
        format!("{}: {} vagon, kapilar kapaniyor", self.line, self.car_count)
    }
}

impl Vehicle for Tram {
    fn line(&self) -> &str { &self.line }
    fn capacity(&self) -> u32 { 180 }
    fn fare(&self) -> u32 { 1500 }
    fn announce(&self) -> String {
        format!("{}: saga dikkat, tramvay geciyor", self.line)
    }
}

impl Vehicle for Funicular {
    fn line(&self) -> &str { "Funikuler" }
    fn capacity(&self) -> u32 { 90 }
    fn fare(&self) -> u32 { 1500 }
}

// ---------------------------------------------------------------
// 2) BOUND'UN UC YAZIMI - ucu de ayni sey
// ---------------------------------------------------------------
fn announce_a<T: Vehicle>(v: &T) -> String {
    v.announce()
}

fn announce_b<T>(v: &T) -> String
where
    T: Vehicle,
{
    v.announce()
}

fn announce_c(v: &impl Vehicle) -> String {
    v.announce()
}

// ---------------------------------------------------------------
// ZAR: std'de hazir rastgele sayi yok. Gercek projede `rand` crate'i kullanilir;
// biz crate indirmemek icin saatten tohum alip xorshift ile ilerletiyoruz.
// ---------------------------------------------------------------
struct Dice {
    seed: u64,
}

impl Dice {
    fn new() -> Dice {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        Dice { seed: nanos | 1 }
    }
    fn delay(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        (self.seed % 6) as u32          // 0..=5 dakika gecikme
    }
}

// ---------------------------------------------------------------
// 3) FARK NEREDE ORTAYA CIKIYOR: iki parametre
// ---------------------------------------------------------------
// T tek bir tip: iki arguman AYNI tip olmak zorunda (ayni turden iki arac)
fn race<T: Vehicle>(a: &T, b: &T) -> String {
    let mut dice = Dice::new();
    let da = dice.delay();
    let db = dice.delay();
    if da <= db {
        format!("{} once vardi ({} dk gecikme, digeri {} dk)", a.line(), da, db)
    } else {
        format!("{} once vardi ({} dk gecikme, digeri {} dk)", b.line(), db, da)
    }
}

// impl Trait: iki arguman FARKLI tip olabilir (aktarmali yolculuk)
fn transfer(a: &impl Vehicle, b: &impl Vehicle) -> String {
    format!("{} -> {} | toplam ucret {} krs", a.line(), b.line(), a.fare() + b.fare())
}

// ---------------------------------------------------------------
// 4) COKLU BOUND
// ---------------------------------------------------------------
#[derive(Debug)]
struct Shuttle {
    line: String,
}

impl Vehicle for Shuttle {
    fn line(&self) -> &str { &self.line }
    fn capacity(&self) -> u32 { 16 }
    fn fare(&self) -> u32 { 2500 }
}

// "Hem Vehicle hem Debug olacak"
fn debug_dispatch(v: &(impl Vehicle + Debug)) {
    println!("  {:?} -> {}", v, v.status());
}

// ---------------------------------------------------------------
// 5) DONUSTE impl Trait: somut tip gizlenir
// ---------------------------------------------------------------
fn first_departure() -> impl Vehicle {
    Tram { line: String::from("T1 Konak") }
}

// AMA tek bir somut tip olmak zorunda:
// fn pick(rush: bool) -> impl Vehicle {
//     if rush { Metro { line: String::from("M1"), car_count: 5 } }
//     else { Tram { line: String::from("T1") } }
// }
//   E0308: `if` and `else` have incompatible types
//   -> derleyicinin donus degerinin BOYUTUNU bilmesi gerekiyor

// DINAMIK dispatch: hangi metodun calisacagi vtable'dan bakilir
fn dynamic_status(v: &dyn Vehicle) -> String {
    v.status()
}

// STATIK dispatch: her somut tip icin ayri kod uretilir (Ders 1)
fn static_status<T: Vehicle>(v: &T) -> String {
    v.status()
}

// impl Vehicle ile YAPAMADIGIMIZ sey: iki farkli tipten birini dondurmek.
// Box<dyn Vehicle> hep ayni boyutta - bir pointer.
fn pick(rush: bool) -> Box<dyn Vehicle> {
    if rush {
        Box::new(Metro { line: String::from("M1 Fahrettin Altay"), car_count: 5 })
    } else {
        Box::new(Tram { line: String::from("T1 Konak") })
    }
}

fn main() {
    let ferry = Ferry { line: String::from("Konak-Karsiyaka"), deck_count: 2 };
    let metro = Metro { line: String::from("M1 Fahrettin Altay"), car_count: 5 };
    let tram = Tram { line: String::from("T1 Konak") };
    let funicular = Funicular;
    let shuttle = Shuttle { line: String::from("Havalimani servisi") };

    println!("-- varsayilan metot vs ezilmis metot --");
    println!("  {}", ferry.announce());       // varsayilan govde
    println!("  {}", funicular.announce());   // varsayilan govde
    println!("  {}", metro.announce());       // EZILMIS
    println!("  {}", tram.announce());        // EZILMIS

    println!("-- varsayilan metot zorunlu metodu cagiriyor --");
    println!("  {}", ferry.status());
    println!("  {}", metro.status());
    println!("  {}", funicular.status());     // kapasite 90 -> is_large() false

    println!("-- trait'e ait olmayan, tipin kendi metodu --");
    println!("  {}", ferry.deck_report());
    // metro.deck_report();  -> E0599: Metro'da boyle bir metot yok

    println!("-- uc bound yazimi, ayni sonuc --");
    println!("  {}", announce_a(&tram));
    println!("  {}", announce_b(&tram));
    println!("  {}", announce_c(&tram));

    println!("-- iki parametre: T ile impl Trait farki --");
    let tram2 = Tram { line: String::from("T2 Karsiyaka") };
    println!("  ayni tur : {}", race(&tram, &tram2));
    println!("  aktarmali: {}", transfer(&ferry, &metro));
    // race(&tram, &metro);
    //   E0308: mismatched types - T zaten Tram'a baglandi, ikincisi Metro

    println!("-- coklu bound: Vehicle + Debug --");
    debug_dispatch(&shuttle);
    // debug_dispatch(&tram);
    //   E0277: `Tram` doesn't implement `Debug` - derive eklemediniz

    println!("-- donuste impl Trait --");
    let ilk = first_departure();
    println!("  {}", ilk.status());

    println!("-- gunluk kapasite --");
    let toplam = ferry.capacity() + metro.capacity() + tram.capacity() + funicular.capacity();
    println!("  toplam kapasite: {} kisi", toplam);

    // DUVAR: dort araci TEK BIR sefer listesine (Vec) koyamiyoruz.
    // let sefer = vec![ferry, metro, tram, funicular];
    //   E0308: mismatched types - Vec tek tip tutar, bunlar dort ayri tip
    // Trait onlari DAVRANISTA birlestirdi, TIPTE birlestirmedi.

    println!("-- duvari yikmak: Box<dyn Vehicle> --");
    // Vec yine TEK tip tutuyor; o tip artik Box<dyn Vehicle>.
    let schedule: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Ferry { line: String::from("Konak-Karsiyaka"), deck_count: 2 }),
        Box::new(Metro { line: String::from("M1 Fahrettin Altay"), car_count: 5 }),
        Box::new(Tram { line: String::from("T1 Konak") }),
        Box::new(Funicular),
    ];
    for v in &schedule {
        println!("  {}", v.status());
    }
    let gunluk: u32 = schedule.iter().map(|v| v.capacity()).sum();
    println!("  sefer listesi kapasitesi: {} kisi", gunluk);

    println!("-- &dyn: sahiplik gerekmiyorsa --");
    let peron: Vec<&dyn Vehicle> = vec![&ferry, &metro];
    for v in &peron {
        println!("  {}", v.announce());
    }

    println!("-- ayni satir, iki farkli dispatch --");
    println!("  statik : {}", static_status(&ferry));
    println!("  dinamik: {}", dynamic_status(&ferry));

    println!("-- donuste dyn: if/else artik mumkun --");
    println!("  yogun saat : {}", pick(true).line());
    println!("  sakin saat : {}", pick(false).line());

    println!("-- fat pointer: dyn iki pointer tasir --");
    println!("  &Ferry            {:>3} bayt", size_of::<&Ferry>());
    println!("  &dyn Vehicle      {:>3} bayt", size_of::<&dyn Vehicle>());
    println!("  Box<Ferry>        {:>3} bayt", size_of::<Box<Ferry>>());
    println!("  Box<dyn Vehicle>  {:>3} bayt", size_of::<Box<dyn Vehicle>>());
    // Gun 3'te slice ve &str de fat pointer'di (ptr + uzunluk).
    // Burada da fat pointer, ama ikinci alan vtable pointeri.
}
