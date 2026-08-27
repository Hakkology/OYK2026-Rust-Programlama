// Gun 6 / Lab - CEVAP ANAHTARI (lab-6.rs icindeki tum TODO cozumleri)
// rustc lab-6-cevap.rs && ./lab-6-cevap
#![allow(unused)]
use std::fmt;
use std::ops::{Add, Mul, Sub};

fn main() { lab_1(); lab_2(); lab_3(); lab_4(); lab_5(); }

// ---------------- LAB 1 ----------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Credits(i64);

impl Credits {
    fn from_credits(c: f64) -> Credits { Credits((c * 100.0).round() as i64) }
    fn as_credits(&self) -> f64 { self.0 as f64 / 100.0 }
}
impl fmt::Display for Credits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:02} Kr", self.0 / 100, (self.0 % 100).abs())
    }
}
impl Add for Credits { type Output = Credits; fn add(self, o: Credits) -> Credits { Credits(self.0 + o.0) } }
impl Sub for Credits { type Output = Credits; fn sub(self, o: Credits) -> Credits { Credits(self.0 - o.0) } }
impl Mul<i64> for Credits { type Output = Credits; fn mul(self, n: i64) -> Credits { Credits(self.0 * n) } }
impl From<i64> for Credits { fn from(v: i64) -> Credits { Credits(v) } }

fn lab_1() {
    println!("-- lab 1 --");
    let vergi = Credits::from_credits(12.5);
    println!("  {} + 3,00 = {}", vergi, vergi + Credits(300));
    println!("  {} x 3     = {}", vergi, vergi * 3);
    let c: Credits = 999i64.into();          // 1d: Into bedava geldi
    println!("  into()     = {}", c);
    let mut v = vec![Credits(300), Credits(50), Credits(1200)];
    v.sort();                                 // 1e: Ord derive edilebildi (i64)
    println!("  sirali     = {:?}", v.iter().map(|c| c.to_string()).collect::<Vec<_>>());
}

// ---------------- LAB 2 ----------------
struct Human { name: String, home_world: String, income: Credits }
struct Vulcanoid { designation: String, logic_score: u32 }
struct SiliconDrone { serial: u64, active: bool }

trait Citizen {
    fn species(&self) -> &str;
    fn tax(&self) -> Credits;
    fn passport(&self) -> String {
        format!("{} kayitli, vergi {}", self.species(), self.tax())
    }
    fn is_taxable(&self) -> bool { self.tax() > Credits(0) }
}

impl Citizen for Human {
    fn species(&self) -> &str { "Human" }
    fn tax(&self) -> Credits { Credits((self.income.0 as f64 * 0.20) as i64) }
    fn passport(&self) -> String {
        format!("{} ({}, {}) vergi {}", self.name, self.species(), self.home_world, self.tax())
    }
}
impl Citizen for Vulcanoid {
    fn species(&self) -> &str { "Vulcanoid" }
    fn tax(&self) -> Credits { Credits(self.logic_score as i64 * 5) }
}
impl Citizen for SiliconDrone {
    fn species(&self) -> &str { "SiliconDrone" }
    fn tax(&self) -> Credits { if self.active { Credits(100) } else { Credits(0) } }
}

fn report_a<T: Citizen>(c: &T) -> String { c.passport() }
fn report_b<T>(c: &T) -> String where T: Citizen { c.passport() }
fn report_c(c: &impl Citizen) -> String { c.passport() }
fn compare_tax<T: Citizen>(a: &T, b: &T) -> String {
    if a.tax() >= b.tax() { format!("{} daha cok odedi", a.species()) }
    else { format!("{} daha cok odedi", b.species()) }
}
fn compare_any(a: &impl Citizen, b: &impl Citizen) -> String {   // 2d duzeltmesi
    if a.tax() >= b.tax() { format!("{} daha cok odedi", a.species()) }
    else { format!("{} daha cok odedi", b.species()) }
}

fn lab_2() {
    println!("-- lab 2 --");
    let h = Human { name: "Ada".into(), home_world: "Terra".into(), income: Credits::from_credits(2500.0) };
    let v = Vulcanoid { designation: "V-77".into(), logic_score: 940 };
    let d = SiliconDrone { serial: 88_213, active: true };
    println!("  {}", report_a(&h));
    println!("  {}", report_b(&v));
    println!("  {}", report_c(&d));
    println!("  vergilendirilebilir mi: {} {} {}", h.is_taxable(), v.is_taxable(), d.is_taxable());
    let h2 = Human { name: "Ege".into(), home_world: "Mars".into(), income: Credits::from_credits(900.0) };
    println!("  {}", compare_tax(&h, &h2));      // ayni tip
    // compare_tax(&h, &d);                      // 2d: E0308
    println!("  {}", compare_any(&h, &d));       // farkli tip: impl Trait
    // let registry = vec![h, v, d];                // 2e: E0308 - DUVAR

    // 2f: duvari yikmak. Vec yine TEK tip tutuyor; o tip artik Box<dyn Citizen>.
    let registry: Vec<Box<dyn Citizen>> = vec![Box::new(h), Box::new(v), Box::new(d)];
    for c in &registry {
        println!("  {}", c.passport());
    }
    let total: f64 = registry.iter().map(|c| c.tax().as_credits()).sum();
    println!("  toplam vergi {}", Credits::from_credits(total));
    // dyn bir FAT POINTER: veri pointeri + vtable pointeri
    println!("  &Human {} bayt / &dyn Citizen {} bayt",
        std::mem::size_of::<&Human>(), std::mem::size_of::<&dyn Citizen>());

    // 2g
    println!("  {}", spawn(true).passport());
    println!("  {}", spawn(false).passport());
}

// ---------------- LAB 3 ----------------
struct Registry<T> { world: String, entries: Vec<T> }

impl<T> Registry<T> {
    fn new(world: &str) -> Self { Registry { world: world.to_string(), entries: Vec::new() } }
    fn add(&mut self, item: T) { self.entries.push(item); }
    fn len(&self) -> usize { self.entries.len() }
}
impl<T: Citizen> Registry<T> {
    fn total_tax(&self) -> Credits {
        let mut t = Credits(0);
        for e in &self.entries { t = t + e.tax(); }
        t
    }
    fn taxable_count(&self) -> usize {
        let mut n = 0;
        for e in &self.entries { if e.is_taxable() { n += 1; } }
        n
    }
}
impl<T: fmt::Display> Registry<T> {
    fn roster(&self) -> String {
        let mut s = format!("{}: ", self.world);
        for (i, e) in self.entries.iter().enumerate() {
            if i > 0 { s.push_str(", "); }
            s.push_str(&e.to_string());
        }
        s
    }
}

// 2g: iki farkli tipten birini dondurmek -> Box<dyn>.
// `-> impl Citizen` ile yazsaydik E0308 alirdik: impl Trait TEK somut tipe baglanir.
fn spawn(vip: bool) -> Box<dyn Citizen> {
    if vip {
        Box::new(Human { name: "Ada".into(), home_world: "Terra".into(), income: Credits::from_credits(2500.0) })
    } else {
        Box::new(SiliconDrone { serial: 88_213, active: true })
    }
}

fn lab_3() {
    println!("-- lab 3 --");
    let mut r: Registry<Human> = Registry::new("Terra");
    r.add(Human { name: "Ada".into(), home_world: "Terra".into(), income: Credits::from_credits(2500.0) });
    r.add(Human { name: "Ege".into(), home_world: "Terra".into(), income: Credits::from_credits(900.0) });
    println!("  {} kayit, toplam vergi {}", r.len(), r.total_tax());
    println!("  vergilendirilebilir: {}", r.taxable_count());
    let empty: Registry<i32> = Registry::new("Test");
    println!("  Registry<i32> len = {}", empty.len());
    // empty.total_tax();                       // 3d: E0599
}

// ---------------- LAB 4 ----------------
impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.name, self.home_world)
    }
}
trait Diplomat: Citizen + fmt::Display {
    fn clearance(&self) -> u8;
    fn credentials(&self) -> String {
        format!("[SEVIYE {}] {} - vergi {}", self.clearance(), self, self.tax())
    }
}
impl Diplomat for Human { fn clearance(&self) -> u8 { 3 } }

// impl fmt::Display for Vec<&str> { ... }      // 4b: E0117

struct Fleet(Vec<String>);                // 4c: newtype
impl fmt::Display for Fleet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filo[")?;
        for (i, s) in self.0.iter().enumerate() {
            if i > 0 { write!(f, " + ")?; }
            write!(f, "{}", s)?;
        }
        write!(f, "]")
    }
}

trait Broadcast { fn broadcast(&self) -> String; }
impl<T: fmt::Display> Broadcast for T {         // 4d: blanket impl
    fn broadcast(&self) -> String { format!(">> {} <<", self) }
}

fn lab_4() {
    println!("-- lab 4 --");
    let h = Human { name: "Ada".into(), home_world: "Terra".into(), income: Credits::from_credits(2500.0) };
    println!("  {}", h.credentials());
    let fleet = Fleet(vec![String::from("Nova"), String::from("Orion")]);
    println!("  {}", fleet);
    println!("  {}", 42.broadcast());
    println!("  {}", "sinyal".broadcast());
    println!("  {}", fleet.broadcast());
}

// ---------------- LAB 5 ----------------
struct IonDrive; struct WarpCore; struct SolarSail;
#[derive(Debug)] struct Xenon { grams: u32 }
#[derive(Debug)] struct Antimatter { micrograms: u32 }
#[derive(Debug)] struct Photons { lumens: u32 }

trait Engine { type Fuel; fn refuel(&self) -> Self::Fuel; }
impl Engine for IonDrive  { type Fuel = Xenon;      fn refuel(&self) -> Xenon { Xenon { grams: 500 } } }
impl Engine for WarpCore  { type Fuel = Antimatter; fn refuel(&self) -> Antimatter { Antimatter { micrograms: 3 } } }
impl Engine for SolarSail { type Fuel = Photons;    fn refuel(&self) -> Photons { Photons { lumens: 90_000 } } }
// impl Engine for IonDrive { type Fuel = Antimatter; ... }   // 5b: E0119

fn show_fuel<E>(e: &E) where E: Engine, E::Fuel: fmt::Debug {
    println!("  yakit: {:?}", e.refuel());
}

struct RawOre { kg: u32 }
trait Refine<T> { fn refine(&self) -> T; }
impl Refine<Xenon> for RawOre   { fn refine(&self) -> Xenon { Xenon { grams: self.kg * 10 } } }
impl Refine<Photons> for RawOre { fn refine(&self) -> Photons { Photons { lumens: self.kg * 1000 } } }

fn lab_5() {
    println!("-- lab 5 --");
    show_fuel(&IonDrive); show_fuel(&WarpCore); show_fuel(&SolarSail);
    let ore = RawOre { kg: 7 };
    let x: Xenon = ore.refine();
    let p: Photons = ore.refine();
    println!("  {:?} / {:?}", x, p);
    // let y = ore.refine();                    // 5e: E0282
}
