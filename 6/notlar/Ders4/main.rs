// Gun 6 / Ders 4 - Supertrait, Orphan Rule, Newtype, Blanket impl
// rustc main.rs && ./main
//
// Ayni savas dunyasi: birimler, patronlar, birlikler.

use std::fmt;
use std::fmt::Display;

trait Unit {
    fn name(&self) -> &str;
    fn hp(&self) -> i32;
}

// ---------------------------------------------------------------
// 1) SUPERTRAIT: "once su olacaksin"
// ---------------------------------------------------------------
// Boss olmak icin once Unit VE Display olmak zorundasin.
trait Boss: Unit + Display {
    fn phase(&self) -> u8;

    // Supertrait'in faydasi: varsayilan gövdede self'i {} ile yazabiliyoruz,
    // cunku Display oldugu GARANTI. Unit metotlarini da cagirabiliyoruz.
    fn intro(&self) -> String {
        format!("[FAZ {}] {} sahneye cikti ({} can)", self.phase(), self, self.hp())
    }
}

struct Dragon {
    hp: i32,
    rage: i32,
}

impl Unit for Dragon {
    fn name(&self) -> &str { "Dragon" }
    fn hp(&self) -> i32 { self.hp }
}

impl Display for Dragon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (ofke {})", self.name(), self.rage)
    }
}

impl Boss for Dragon {
    fn phase(&self) -> u8 {
        if self.hp < 200 { 2 } else { 1 }
    }
}

// Display OLMAYAN bir tipe Boss uygulanamaz:
#[allow(dead_code)]                 // sadece asagidaki yorumlu ornek icin duruyor
struct Slime { hp: i32 }

// impl Unit for Slime { ... }
// impl Boss for Slime { fn phase(&self) -> u8 { 1 } }
//   E0277: `Slime` doesn't implement `std::fmt::Display`
//   -> supertrait sozlesmesi saglanmadi

// ---------------------------------------------------------------
// 2) ORPHAN RULE
// ---------------------------------------------------------------
// Kural: `impl Trait for Type` yazabilmek icin TRAIT ya da TYPE sizin olmali.
//
//   impl Display for Dragon      OK  - Dragon benim
//   impl Describe for u32        OK  - Describe benim
//   impl Display for Vec<i32>    HAYIR - ikisi de baskasinin  (E0117)

trait Describe {
    fn describe(&self) -> String;
}

// Kendi trait'imizi BASKASININ tipine uygulayabiliriz:
impl Describe for u32 {
    fn describe(&self) -> String {
        format!("{} hasar", self)
    }
}

impl Describe for &str {
    fn describe(&self) -> String {
        format!("'{}' buyusu ({} harf)", self, self.chars().count())
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
struct Party(Vec<String>);

impl Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Birlik[")?;
        for (i, u) in self.0.iter().enumerate() {
            if i > 0 { write!(f, " + ")?; }
            write!(f, "{}", u)?;
        }
        write!(f, "]")
    }
}

// ---------------------------------------------------------------
// 4) BLANKET IMPL: "su kosulu saglayan HER tip su trait'i alsin"
// ---------------------------------------------------------------
trait Taunt {
    fn taunt(&self) -> String;
}

// Display olan HER tip Taunt kazanir - tek satirla milyonlarca tipe
impl<T: Display> Taunt for T {
    fn taunt(&self) -> String {
        format!("{} seni korkutmuyor mu?", self)
    }
}

// std'de ayni desenin iki unlu ornegi:
//   impl<T: Display> ToString for T                -> Display yazinca to_string() bedava
//   impl<T, U: From<T>> Into<U> for T              -> From yazinca into() bedava

// ---------------------------------------------------------------
// 5) SEALED TRAIT: disaridan implemente edilemeyen trait
// ---------------------------------------------------------------
mod sealed {
    pub trait Seal {}          // bu modulun disina cikmiyor
}

// Disaridaki bir crate sealed::Seal'e erisemez, dolayisiyla DamageType'i
// implemente edemez. std API'sini kilitlemek icin bunu kullanir.
pub trait DamageType: sealed::Seal {
    fn multiplier(&self) -> f64;
}

struct Fire;
impl sealed::Seal for Fire {}
impl DamageType for Fire {
    fn multiplier(&self) -> f64 { 1.5 }
}

// ---------------------------------------------------------------
// 6) OBJECT SAFETY: her trait dyn olamaz
// ---------------------------------------------------------------
// Bu trait NESNE GUVENLI DEGIL: self almayan, Self donduren metot var.
trait Summon {
    fn summon() -> Self;
}

impl Summon for Dragon {
    fn summon() -> Dragon { Dragon { hp: 500, rage: 15 } }
}

// Summon dyn OLAMAZ - denemesi main icinde, yorumda duruyor.

// Cozum: sorunlu metodu vtable'in DISINDA birak.
trait Summonable {
    fn title(&self) -> String;                    // vtable'a girer

    fn summon() -> Self                           // vtable'a GIRMEZ
    where
        Self: Sized;
}

impl Summonable for Dragon {
    fn title(&self) -> String { format!("{} cagrildi", self.name()) }
    fn summon() -> Dragon { Dragon { hp: 400, rage: 20 } }
}

// Boss nesne guvenli: tum metotlar &self aliyor, Self dondurmuyor.
fn parade(bosses: &[Box<dyn Boss>]) {
    for b in bosses {
        println!("  {}", b.intro());
    }
}

fn main() {
    let dragon = Dragon { hp: 500, rage: 15 };
    let wounded = Dragon { hp: 120, rage: 40 };

    println!("-- supertrait --");
    println!("  {}", dragon.intro());
    println!("  {}", wounded.intro());       // can dustu -> faz 2

    println!("-- kendi trait'imiz, BASKASININ tipinde --");
    println!("  {}", 55u32.describe());
    println!("  {}", "alev topu".describe());

    println!("-- newtype ile orphan rule asildi --");
    let party = Party(vec![
        String::from("Archer"),
        String::from("Knight"),
        String::from("Healer"),
    ]);
    println!("  {}", party);

    println!("-- blanket impl: Display olan her tip --");
    println!("  {}", 42.taunt());
    println!("  {}", "goblin".taunt());
    println!("  {}", dragon.taunt());
    println!("  {}", party.taunt());

    println!("-- object safety --");
    // Summonable dyn olabiliyor, cunku summon() where Self: Sized ile isaretli
    // iki trait de `summon` tanimliyor -> Dragon::summon() belirsiz kalir (E0034),
    // tam nitelikli cagri sart:
    let summoned: Box<dyn Summonable> = Box::new(<Dragon as Summonable>::summon());
    println!("  {}", summoned.title());
    // let s: Box<dyn Summon> = Box::new(<Dragon as Summon>::summon());
    //   E0038: the trait `Summon` is not dyn compatible
    //   ...because associated function `summon` has no `self` parameter
    //   Sebep: vtable'da metot basina TEK adres var; self almayan bir metotta
    //   hangi nesnenin tablosuna bakilacagi belli degil.
    // summoned.summon();
    //   E0599: no method named `summon` found for struct `Box<dyn Summonable>`
    //   -> where Self: Sized ile isaretledigimiz metot vtable'a girmedi
    println!("  somut tip uzerinden: {}", <Dragon as Summon>::summon().name());

    let bosses: Vec<Box<dyn Boss>> = vec![
        Box::new(Dragon { hp: 500, rage: 15 }),
        Box::new(Dragon { hp: 120, rage: 40 }),
    ];
    parade(&bosses);

    println!("-- sealed trait --");
    println!("  ates hasari carpani: {}", Fire.multiplier());
    // Disaridaki bir crate `impl DamageType for X` yazamaz:
    // once sealed::Seal'i implemente etmesi gerekir, ama ona erisemez.
}
