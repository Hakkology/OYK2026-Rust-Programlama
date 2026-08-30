// Gun 6 / Ders 4 (v1) - Supertrait, Orphan Rule, Newtype, Blanket impl
// rustc main_v1.rs && ./main_v1
//
// Ayni ulasim dunyasi: araclar, ekspres hatlar, guzergahlar.

use std::fmt;
use std::fmt::Display;

trait Vehicle {
    fn line(&self) -> &str;
    fn capacity(&self) -> u32;
}

// ---------------------------------------------------------------
// 1) SUPERTRAIT: "once su olacaksin"
// ---------------------------------------------------------------
// Express olmak icin once Vehicle VE Display olmak zorundasin.
trait Express: Vehicle + Display {
    fn skipped_stops(&self) -> u8;

    // Supertrait'in faydasi: varsayilan govdede self'i {} ile yazabiliyoruz,
    // cunku Display oldugu GARANTI. Vehicle metotlarini da cagirabiliyoruz.
    fn banner(&self) -> String {
        format!("[EKSPRES] {} - {} durak atliyor, {} kisilik",
            self, self.skipped_stops(), self.capacity())
    }
}

struct Metro {
    line: String,
    car_count: u8,
}

impl Vehicle for Metro {
    fn line(&self) -> &str { &self.line }
    fn capacity(&self) -> u32 { 220 * self.car_count as u32 }
}

impl Display for Metro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} vagon)", self.line, self.car_count)
    }
}

impl Express for Metro {
    fn skipped_stops(&self) -> u8 {
        if self.car_count >= 5 { 4 } else { 2 }
    }
}

// Display OLMAYAN bir tipe Express uygulanamaz:
#[allow(dead_code)]                 // sadece asagidaki yorumlu ornek icin duruyor
struct Minibus { line: String }

// impl Vehicle for Minibus { ... }
// impl Express for Minibus { fn skipped_stops(&self) -> u8 { 1 } }
//   E0277: `Minibus` doesn't implement `std::fmt::Display`
//   -> supertrait sozlesmesi saglanmadi

// ---------------------------------------------------------------
// 2) ORPHAN RULE
// ---------------------------------------------------------------
// Kural: `impl Trait for Type` yazabilmek icin TRAIT ya da TYPE sizin olmali.
//   impl Display for Metro       OK  - Metro benim
//   impl Announce for u32        OK  - Announce benim
//   impl Display for Vec<i32>    HAYIR - ikisi de baskasinin  (E0117)

trait Announce {
    fn announce(&self) -> String;
}

// Kendi trait'imizi BASKASININ tipine uygulayabiliriz:
impl Announce for u32 {
    fn announce(&self) -> String {
        format!("{} numarali sefer", self)
    }
}

impl Announce for &str {
    fn announce(&self) -> String {
        format!("'{}' anonsu ({} harf)", self, self.len())
    }
}

// Ama bu YASAK:
// impl Display for Vec<i32> { ... }
//   E0117: only traits defined in the current crate can be implemented
//          for types defined outside of the crate

// ---------------------------------------------------------------
// 3) NEWTYPE: orphan rule'un etrafindan dolasmak
// ---------------------------------------------------------------
// Vec<String>'e Display yazamayiz. Kendi tipimize sararsak tip BIZIM olur.
struct Route(Vec<String>);

impl Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guzergah[")?;
        for (i, durak) in self.0.iter().enumerate() {
            if i > 0 { write!(f, " > ")?; }
            write!(f, "{}", durak)?;
        }
        write!(f, "]")
    }
}

// ---------------------------------------------------------------
// 4) BLANKET IMPL: "su kosulu saglayan HER tip su trait'i alsin"
// ---------------------------------------------------------------
trait Loudspeaker {
    fn over_speaker(&self) -> String;
}

// Display olan HER tip Loudspeaker kazanir - tek satirla milyonlarca tipe
impl<T: Display> Loudspeaker for T {
    fn over_speaker(&self) -> String {
        format!(">> {} <<", self)
    }
}

// std bunu cok kullanir:
//   impl<T: Display> ToString for T                -> Display yazinca to_string() bedava
//   impl<T, U: From<T>> Into<U> for T              -> From yazinca into() bedava

// ---------------------------------------------------------------
// 5) SEALED TRAIT: disaridan implemente edilemeyen trait
// ---------------------------------------------------------------
mod sealed {
    pub trait Seal {}
}

pub trait TicketKind: sealed::Seal {
    fn multiplier(&self) -> f64;
}

struct Student;
impl sealed::Seal for Student {}
impl TicketKind for Student {
    fn multiplier(&self) -> f64 { 0.5 }
}
// Disaridaki bir crate `impl TicketKind for X` yazamaz:
// once sealed::Seal'i implemente etmesi gerekir, ama ona erisemez.

// ---------------------------------------------------------------
// 6) OBJECT SAFETY: her trait dyn olamaz
// ---------------------------------------------------------------
// Bu trait NESNE GUVENLI DEGIL: self almayan, Self donduren metot var.
trait Spawnable {
    fn spawn() -> Self;
}

impl Spawnable for Metro {
    fn spawn() -> Metro { Metro { line: String::from("M1"), car_count: 5 } }
}

// Cozum: sorunlu metodu vtable'in DISINDA birak.
trait Deployable {
    fn label(&self) -> String;                    // vtable'a girer

    fn deploy() -> Self                           // vtable'a GIRMEZ
    where
        Self: Sized;
}

impl Deployable for Metro {
    fn label(&self) -> String { format!("{} sefere cikti", self.line) }
    fn deploy() -> Metro { Metro { line: String::from("M2"), car_count: 4 } }
}

// Express nesne guvenli: tum metotlar &self aliyor, Self dondurmuyor.
fn board_all(hatlar: &[Box<dyn Express>]) {
    for h in hatlar {
        println!("  {}", h.banner());
    }
}

fn main() {
    let metro = Metro { line: String::from("M1 Fahrettin Altay"), car_count: 5 };
    let kisa = Metro { line: String::from("M2 Bornova"), car_count: 3 };

    println!("-- supertrait --");
    println!("  {}", metro.banner());
    println!("  {}", kisa.banner());          // 3 vagon -> 2 durak atliyor

    println!("-- kendi trait'imiz, BASKASININ tipinde --");
    println!("  {}", 34u32.announce());
    println!("  {}", "son sefer 23:40".announce());

    println!("-- newtype ile orphan rule asildi --");
    let route = Route(vec![
        String::from("Konak"),
        String::from("Cankaya"),
        String::from("Basmane"),
    ]);
    println!("  {}", route);

    println!("-- blanket impl: Display olan her tip --");
    println!("  {}", 42.over_speaker());
    println!("  {}", "peron degisikligi".over_speaker());
    println!("  {}", metro.over_speaker());
    println!("  {}", route.over_speaker());

    println!("-- object safety --");
    // Deployable dyn olabiliyor, cunku deploy() where Self: Sized ile isaretli
    let hazir: Box<dyn Deployable> = Box::new(<Metro as Deployable>::deploy());
    println!("  {}", hazir.label());
    // hazir.deploy();  -> vtable'da yok, dyn uzerinden cagrilamaz
    //   E0599: no method named `deploy` found for struct `Box<dyn Deployable>`
    println!("  somut tip uzerinden: {}", <Metro as Spawnable>::spawn().line());
    // let s: Box<dyn Spawnable> = Box::new(<Metro as Spawnable>::spawn());
    //   E0038: the trait `Spawnable` is not dyn compatible
    //   ...because associated function `spawn` has no `self` parameter

    let ekspresler: Vec<Box<dyn Express>> = vec![
        Box::new(Metro { line: String::from("M1 Fahrettin Altay"), car_count: 5 }),
        Box::new(Metro { line: String::from("M2 Bornova"), car_count: 3 }),
    ];
    board_all(&ekspresler);

    println!("-- sealed trait --");
    println!("  ogrenci bilet carpani: {}", Student.multiplier());
}
