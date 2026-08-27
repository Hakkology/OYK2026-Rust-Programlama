// Gun 6 / Lab - Galactic Citizenship Registry
// rustc lab-6.rs && ./lab-6
//
// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan uyarilari normal.
#![allow(unused)]
//
// SENARYO
// Galaktik Vatandaslik Burosu'nda calisiyorsunuz. Farkli turler kayit oluyor,
// hepsinin vergisi farkli hesaplaniyor, hepsi tek bir sicilde tutulacak.
//
// Bugunun bes dersi burada sirayla kullaniliyor:
//   LAB 1 -> newtype + standart trait'ler (Ders 3)
//   LAB 2 -> trait tanimi, varsayilan metot, bound (Ders 2)
//   LAB 3 -> generic yapi + kosullu impl (Ders 1)
//   LAB 4 -> supertrait, newtype, blanket impl (Ders 4)
//   LAB 5 -> associated type (Ders 5)

use std::fmt;
use std::ops::{Add, Mul, Sub};

fn main() {
    lab_1_credits();
    lab_2_citizen();
    lab_3_registry();
    lab_4_diplomat();
    lab_5_ship();
}

// ===========================================================================
// LAB 1 - Credits: galaktik para birimi
// Para f64 ile tutulmaz. En kucuk birim "santi-kredi", i64 tutuyoruz.
// ===========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Credits(i64);           // santi-kredi cinsinden

impl Credits {
    // ORNEK: iki farkli kurucu
    fn from_credits(c: f64) -> Credits {
        Credits((c * 100.0).round() as i64)
    }
    fn as_credits(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

// ORNEK: Display elle yazilir
impl fmt::Display for Credits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:02} Kr", self.0 / 100, (self.0 % 100).abs())
    }
}

// TODO 1a: `impl Add for Credits` yazin (type Output = Credits).
// TODO 1b: `impl Sub for Credits` yazin.
// TODO 1c: `impl Mul<i64> for Credits` yazin - "N kisilik ailenin vergisi".
// TODO 1d: `impl From<i64> for Credits` yazin (santi-kredi alacak).
//          Yazdiktan sonra `let c: Credits = 250i64.into();` calismali.
//          SORU: `Into` implementasyonunu yazdiniz mi? Neden calisiyor?
// TODO 1e: Credits neden `Ord` derive edebiliyor da bir `f64` newtype'i edemezdi?
//          Tek cumleyle yazin.

fn lab_1_credits() {
    println!("-- lab 1: Credits --");
    let vergi = Credits::from_credits(12.5);
    println!("  {} (ondalik: {})", vergi, vergi.as_credits());

    // TODO 1f: 1a-1d bittikten sonra sunlari calistirin:
    //   println!("{}", vergi + Credits(300));
    //   println!("{}", vergi * 3);
    //   let c: Credits = 999i64.into();
}

// ===========================================================================
// LAB 2 - Citizen trait'i
// ===========================================================================
// ORNEK: uc tur, tamamen farkli veriler
struct Human {
    name: String,
    home_world: String,
    income: Credits,
}

struct Vulcanoid {
    designation: String,
    logic_score: u32,
}

struct SiliconDrone {
    serial: u64,
    active: bool,
}

// TODO 2a: `trait Citizen` tanimlayin.
//          ZORUNLU:  fn species(&self) -> &str
//                    fn tax(&self) -> Credits
//          VARSAYILAN: fn passport(&self) -> String
//                        -> "[tur] kayitli, vergi X Kr" gibi bir metin uretsin
//                      fn is_taxable(&self) -> bool
//                        -> vergisi sifirdan buyukse true
//          Ipucu: varsayilan metot ZORUNLU metotlari cagirabilir.

// TODO 2b: Uc tip icin de `impl Citizen` yazin.
//          Human       -> vergi gelirin %20'si
//          Vulcanoid   -> mantik puani basina 5 santi-kredi
//          SiliconDrone-> aktifse 100 santi-kredi, degilse 0
//          Human icin passport()'u EZIN, adi da yazsin.

// TODO 2c: Bound'un uc yazimini da deneyin - ucu de ayni isi yapmali:
//          fn report_a<T: Citizen>(c: &T) -> String
//          fn report_b<T>(c: &T) -> String where T: Citizen
//          fn report_c(c: &impl Citizen) -> String

// TODO 2d: fn compare_tax<T: Citizen>(a: &T, b: &T) -> String yazin.
//          Sonra `compare_tax(&human, &drone)` cagirmayi DENEYIN.
//          Hangi hatayi aldiniz? Neden? Duzeltmek icin imzayi nasil degistirirsiniz?

// TODO 2e (DUVAR): uc vatandasi TEK bir Vec'e koymayi deneyin:
//          let sicil = vec![human, vulcanoid, drone];
//          Derlenmiyor. Hata kodu ne? Trait onlari neyde birlestirdi, neyde birlestirmedi?

// TODO 2f (DUVARI YIKIN): ayni ucunu `Vec<Box<dyn Citizen>>` icine koyun.
//          let sicil: Vec<Box<dyn Citizen>> = vec![Box::new(human), ...];
//          Hepsini dolasip passport() yazdirin, toplam vergiyi hesaplayin.
//          Sonra sunu da yazdirin ve farki aciklayin:
//            std::mem::size_of::<&Human>()        ->  ?
//            std::mem::size_of::<&dyn Citizen>()  ->  ?

// TODO 2g: fn spawn(vip: bool) -> Box<dyn Citizen> yazin: vip ise Human,
//          degilse SiliconDrone dondursun. Ayni seyi `-> impl Citizen` ile
//          yazmayi deneyin, aldiginiz hata kodunu not edin.

fn lab_2_citizen() {
    println!("-- lab 2: Citizen --");
    let human = Human {
        name: String::from("Ada"),
        home_world: String::from("Terra"),
        income: Credits::from_credits(2500.0),
    };
    let vulcanoid = Vulcanoid { designation: String::from("V-77"), logic_score: 940 };
    let drone = SiliconDrone { serial: 88_213, active: true };

    println!("  kayitlar hazir: {} / {} / {}", human.name, vulcanoid.designation, drone.serial);
    // TODO: 2a-2b bitince passport() ciktilarini yazdirin
}

// ===========================================================================
// LAB 3 - Registry<T>: generic sicil
// ===========================================================================
struct Registry<T> {
    world: String,
    entries: Vec<T>,
}

// TODO 3a: `impl<T> Registry<T>` yazin - BUTUN T'ler icin:
//          fn new(world: &str) -> Self
//          fn add(&mut self, item: T)
//          fn len(&self) -> usize

// TODO 3b: `impl<T: Citizen> Registry<T>` yazin - SADECE vatandaslar icin:
//          fn total_tax(&self) -> Credits
//          fn taxable_count(&self) -> usize

// TODO 3c: `impl<T: fmt::Display> Registry<T>` yazin - SADECE Display olanlar icin:
//          fn roster(&self) -> String

// TODO 3d: `let r: Registry<i32> = Registry::new("Test");` olusturup
//          `r.total_tax()` cagirmayi deneyin. Hata kodu ne?
//          Kosullu impl'in ne demek oldugunu bir cumleyle yazin.

fn lab_3_registry() {
    println!("-- lab 3: Registry --");
    // TODO: 3a bitince Registry<Human> olusturup birkac kayit ekleyin,
    //       total_tax() ve len() sonuclarini yazdirin.
}

// ===========================================================================
// LAB 4 - Diplomat: supertrait, newtype, blanket impl
// ===========================================================================
// TODO 4a: `trait Diplomat: Citizen + fmt::Display` tanimlayin.
//          fn clearance(&self) -> u8;
//          VARSAYILAN: fn credentials(&self) -> String
//            -> icinde self'i {} ile yazdirin. Neden yazabiliyorsunuz?
//          Human icin Display + Diplomat yazip deneyin.

// TODO 4b: `impl fmt::Display for Vec<&str>` yazmayi DENEYIN.
//          Hata kodu ne? Kuralin adi ne?

// TODO 4c: newtype ile 4b'yi asin:
//          struct Fleet(Vec<String>);
//          impl fmt::Display for Fleet   ->  "Filo[Nova + Orion]" gibi

// TODO 4d: blanket impl yazin:
//          trait Broadcast { fn broadcast(&self) -> String; }
//          impl<T: fmt::Display> Broadcast for T { ... }
//          Sonra 42.broadcast(), "sinyal".broadcast(), fleet.broadcast() deneyin.
//          std'de ayni desenin iki ornegini hatirliyor musunuz?

fn lab_4_diplomat() {
    println!("-- lab 4: Diplomat --");
    // TODO: 4a-4d bitince ciktilari buraya yazdirin
}

// ===========================================================================
// LAB 5 - Ship: associated type
// ===========================================================================
struct IonDrive;
struct WarpCore;
struct SolarSail;

#[derive(Debug)]
struct Xenon { grams: u32 }
#[derive(Debug)]
struct Antimatter { micrograms: u32 }
#[derive(Debug)]
struct Photons { lumens: u32 }

// TODO 5a: `trait Engine` tanimlayin:
//          type Fuel;
//          fn refuel(&self) -> Self::Fuel;
//          Uc motor icin de implemente edin (IonDrive->Xenon, WarpCore->Antimatter,
//          SolarSail->Photons).

// TODO 5b: `impl Engine for IonDrive` bloguna IKINCI bir tane daha ekleyip
//          `type Fuel = Antimatter;` yazmayi deneyin. Hata kodu ne?
//          Bu, associated type'in hangi ozelligini kanitliyor?

// TODO 5c: Su fonksiyonu yazin:
//          fn show_fuel<E>(e: &E) where E: Engine, E::Fuel: std::fmt::Debug
//          Uc motoru da verip calistirin.

// TODO 5d: Ayni girdiden IKI farkli urun cikan bir durum kurun:
//          trait Refine<T> { fn refine(&self) -> T; }
//          struct RawOre { kg: u32 }
//          impl Refine<Xenon> for RawOre     { ... }
//          impl Refine<Photons> for RawOre   { ... }
//          Sonra `let x: Xenon = ore.refine();` ve `let p: Photons = ore.refine();`
//          SORU: neden burada associated type kullanamazdik?

// TODO 5e: `let y = ore.refine();` yazip tip belirtmeden deneyin. Hata kodu ne?

fn lab_5_ship() {
    println!("-- lab 5: Engine --");
    // TODO: 5a bitince uc motoru da doldurup yakitlarini yazdirin
}
