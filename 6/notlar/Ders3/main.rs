// Gun 6 / Ders 3 - Standart Trait'leri ELLE Yazmak
// rustc main.rs && ./main
//
// Gun 4'te derive ettiklerimizi bugun elle yaziyoruz.
// Dunya: karakter istatistikleri - can, mana, seviye.

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::ops::{Add, Mul, Sub};

// ---------------------------------------------------------------
// NEWTYPE: her istatistik AYRI tip
// ---------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Hp(i32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Mana(i32);

// Seviye tam sayi: Eq ve Ord DERIVE EDILEBILIYOR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Level(u32);

// Kritik carpani f64 tutuyor: NaN yuzunden Eq/Ord ALAMAZ (Gun 4'teki kural).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct CritMultiplier(f64);

// ---------------------------------------------------------------
// 1) Display - ELLE yazilir, derive EDILEMEZ
// ---------------------------------------------------------------
impl fmt::Display for Hp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} can", self.0)
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mana", self.0)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sv.{}", self.0)
    }
}

// ---------------------------------------------------------------
// 2) From - kayipsiz donusum. From yazinca Into BEDAVA gelir.
//    Kural: her seviye 20 temel can verir.
// ---------------------------------------------------------------
impl From<Level> for Hp {
    fn from(l: Level) -> Hp {
        Hp(l.0 as i32 * 20)
    }
}

// Mana da seviyeye bagli: seviye basina 10
impl From<Level> for Mana {
    fn from(l: Level) -> Mana {
        Mana(l.0 as i32 * 10)
    }
}

// ---------------------------------------------------------------
// 3) TryFrom - donusum BASARISIZ olabiliyorsa
//    Negatif can diye bir sey yok.
// ---------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct NegativeHp(i32);

// Hata tipi de bir NEWTYPE: hangi degerin reddedildigini yaninda tasiyor.
// Gun 5'te ogrendigimiz sozlesmeyi tamamliyoruz: Display + Error.
impl fmt::Display for NegativeHp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "negatif can olmaz: {}", self.0)
    }
}

impl Error for NegativeHp {}

impl TryFrom<i32> for Hp {
    type Error = NegativeHp;

    fn try_from(v: i32) -> Result<Hp, Self::Error> {
        if v < 0 {
            Err(NegativeHp(v))
        } else {
            Ok(Hp(v))
        }
    }
}

// ---------------------------------------------------------------
// 4) OPERATOR ASIRI YUKLEME: + - * birer trait
// ---------------------------------------------------------------
impl Add for Hp {
    type Output = Hp;            // associated type: donus tipini trait belirliyor
    fn add(self, o: Hp) -> Hp { Hp(self.0 + o.0) }
}

impl Sub for Hp {
    type Output = Hp;
    fn sub(self, o: Hp) -> Hp {
        Hp((self.0 - o.0).max(0))    // can sifirin altina inmez
    }
}

// Farkli tiple carpma: kritik vurus. Sag taraf generic parametre.
impl Mul<CritMultiplier> for Hp {
    type Output = Hp;
    fn mul(self, c: CritMultiplier) -> Hp {
        Hp((self.0 as f64 * c.0) as i32)
    }
}

// Seviye + kazanilan seviye
impl Add<u32> for Level {
    type Output = Level;
    fn add(self, k: u32) -> Level { Level(self.0 + k) }
}

// ---------------------------------------------------------------
// 5) impl Into<T> parametresi: cagiran esnek olsun
// ---------------------------------------------------------------
fn print_hp<T: Into<Hp>>(x: T) {
    let hp: Hp = x.into();
    println!("  {}", hp);
}

// Error yazdigimiz icin ? bu hatayi Box<dyn Error>'a cevirebiliyor (Gun 5).
fn load_hp(raw: i32) -> Result<Hp, Box<dyn Error>> {
    let hp = Hp::try_from(raw)?;
    Ok(hp)
}

fn main() {
    let dragon_hp = Hp(500);
    let archer_hp = Hp(80);
    let hit = Hp(55);

    println!("-- Display (elle yazildi) --");
    println!("  {} / {}", dragon_hp, archer_hp);
    // Display yazinca to_string() BEDAVA gelir (std'deki blanket impl sayesinde)
    println!("  to_string(): {:?}", archer_hp.to_string());

    println!("-- operatorler --");
    println!("  ejderha vuruldu : {}", dragon_hp - hit);
    println!("  iksir icildi    : {}", archer_hp + Hp(40));
    println!("  kritik x2.5     : {}", hit * CritMultiplier(2.5));
    println!("  seviye atladi   : {}", Level(7) + 1);
    println!("  can sifirin alti: {}", Hp(10) - Hp(50));   // 0'da duruyor

    println!("-- From / Into --");
    let level = Level(7);
    let base_hp: Hp = Hp::from(level);      // From ile
    let base_mana: Mana = level.into();      // Into BEDAVA geldi
    println!("  {} -> {} + {}", level, base_hp, base_mana);

    // Seviye atlayinca can/mana KENDILIGINDEN degismez, yeniden turetilir.
    let level = level + 1;
    let base_hp: Hp = level.into();
    let base_mana: Mana = level.into();
    println!("  seviye atladi: {} -> {} + {}", level, base_hp, base_mana);

    println!("-- impl Into<T> parametresi --");
    print_hp(Level(3));                            // Level verdik
    print_hp(Hp(120));                             // Hp de verebiliriz

    println!("-- TryFrom: basarisiz olabilen donusum --");
    println!("  {:?}", Hp::try_from(250));
    println!("  {:?}", Hp::try_from(-30));
    match Hp::try_from(-30) {
        Ok(h) => println!("  gecerli: {}", h),
        Err(NegativeHp(v)) => println!("  gecersiz: {} - negatif can olmaz", v),
    }
    // Display yazdik: {} ile kullaniciya gosterilebiliyor
    println!("  Display : {}", NegativeHp(-30));
    // Error yazdik: ? artik bu hatayi Box<dyn Error>'a cevirebiliyor
    println!("  ? ile   : {:?}", load_hp(250).map(|h| h.to_string()));
    match load_hp(-30) {
        Ok(_) => {}
        Err(e) => println!("  ? ile   : hata -> {}", e),
    }

    println!("-- Ord: Level siralanabiliyor, CritMultiplier siralanamiyor --");
    let mut levels = vec![Level(12), Level(3), Level(27), Level(9)];
    levels.sort();                                   // Ord derive edildi
    print!("  ");
    for l in &levels { print!("{} | ", l); }
    println!();
    println!("  en dusuk: {}   en yuksek: {}", levels[0], levels[levels.len() - 1]);

    let mut crits = vec![CritMultiplier(2.0), CritMultiplier(1.25), CritMultiplier(3.0)];
    // crits.sort();
    //   E0277: the trait bound `CritMultiplier: Ord` is not satisfied
    //   -> f64 iceriyor, NaN yuzunden tam siralama yok
    crits.sort_by(|a, b| a.partial_cmp(b).unwrap());   // PartialOrd yetiyor
    println!("  crits: {:?}", crits);

    println!("-- Default --");
    println!("  {:?} / {}", Level::default(), Level::default());

    println!("-- tip guvenligi --");
    // let wrong = archer_hp + Mana(10);
    //   E0308: expected `Hp`, found `Mana`
    //   Can ile mana ayri tip oldugu icin KARISAMAZLAR. Bedeli sifir:
    println!("  Hp = {} bayt, i32 = {} bayt",
        std::mem::size_of::<Hp>(), std::mem::size_of::<i32>());
}
