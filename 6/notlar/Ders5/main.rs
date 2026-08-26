// Gun 6 / Ders 5 - Associated Types ve "Generic mi, Associated mi?"
// rustc main.rs && ./main
//
// Ayni savas dunyasi: silahlar, cephane, uretim (crafting).

use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Arrow { count: u32 }
#[derive(Debug, Clone, Copy)]
struct Bullet { count: u32 }
#[derive(Debug, Clone, Copy)]
struct ManaCharge { amount: u32 }

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{} ok", self.count) }
}
impl fmt::Display for Bullet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{} mermi", self.count) }
}
impl fmt::Display for ManaCharge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{} mana yuku", self.amount) }
}

struct Bow;
struct Musket;
struct Staff;

// ---------------------------------------------------------------
// 1) ASSOCIATED TYPE: "her tip icin TEK dogru cevap"
// ---------------------------------------------------------------
// Yay ok atar. Tufek mermi atar. Asa mana yakar.
// Her silahin cephane tipi TEKTIR -> associated type.
trait Weapon {
    type Ammo;                              // trait'in ICINDE tanimli tip
    fn reload(&self) -> Self::Ammo;
}

impl Weapon for Bow {
    type Ammo = Arrow;                      // Bow icin cevap: Arrow
    fn reload(&self) -> Arrow { Arrow { count: 30 } }
}

impl Weapon for Musket {
    type Ammo = Bullet;                     // Musket icin cevap: Bullet
    fn reload(&self) -> Bullet { Bullet { count: 6 } }
}

impl Weapon for Staff {
    type Ammo = ManaCharge;
    fn reload(&self) -> ManaCharge { ManaCharge { amount: 100 } }
}

// Ayni tipe IKINCI kez implemente edilemez:
// impl Weapon for Bow { type Ammo = Bullet; ... }
//   E0119: conflicting implementations of trait `Weapon` for type `Bow`

// Associated type'i bound icinde kullanmak:
fn show_ammo<W>(w: &W)
where
    W: Weapon,
    W::Ammo: fmt::Display,                  // "cephane tipi yazdirilabilir olsun"
{
    println!("  doldurdu: {}", w.reload());
}

// ---------------------------------------------------------------
// 2) GENERIC PARAMETRE: "ayni tip icin BIRDEN COK cevap"
// ---------------------------------------------------------------
#[derive(Debug)]
struct Sword { sharpness: u32 }
#[derive(Debug)]
struct Shield { defense: u32 }

struct Iron { kg: u32 }

// Demirden HEM kilic HEM kalkan yapilabilir: cevap birden fazla -> generic.
trait Craft<T> {
    fn craft(&self) -> T;
}

impl Craft<Sword> for Iron {
    fn craft(&self) -> Sword { Sword { sharpness: self.kg * 3 } }
}

impl Craft<Shield> for Iron {               // AYNI tip, IKINCI impl - generic sayesinde
    fn craft(&self) -> Shield { Shield { defense: self.kg * 5 } }
}

// ---------------------------------------------------------------
// 3) STD'DEN ORNEKLER
// ---------------------------------------------------------------
// Add trait'i ikisini birden kullanir:
//   trait Add<Rhs = Self> { type Output; fn add(self, rhs: Rhs) -> Self::Output; }
//    ^ generic parametre (sag taraf degisebilir)   ^ associated type (sonuc tektir)
//
// Iterator ise sadece associated type kullanir:
//   trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
// Cunku bir iterator'un urettigi eleman tipi TEKTIR.

use std::ops::Add;

impl Add for Arrow {
    type Output = Arrow;                    // Arrow + Arrow = Arrow, tek cevap
    fn add(self, o: Arrow) -> Arrow { Arrow { count: self.count + o.count } }
}

impl Add<u32> for Arrow {                   // Arrow + u32 de tanimlanabilir
    type Output = Arrow;
    fn add(self, o: u32) -> Arrow { Arrow { count: self.count + o } }
}

fn main() {
    let bow = Bow;
    let musket = Musket;
    let staff = Staff;

    println!("-- associated type --");
    show_ammo(&bow);
    show_ammo(&musket);
    show_ammo(&staff);

    // Donen tipi derleyici biliyor: Bow -> Arrow
    let ammo: Arrow = bow.reload();
    println!("  tip belli: {:?}", ammo);

    println!("-- generic parametre: ayni girdiden iki urun --");
    let iron = Iron { kg: 4 };
    let sword: Sword = iron.craft();         // hangi impl? donus tipi soyluyor
    let shield: Shield = iron.craft();
    println!("  {:?} / {:?}", sword, shield);
    println!("  keskinlik {} | savunma {}", sword.sharpness, shield.defense);
    // let x = iron.craft();
    //   E0283: type annotations needed - IKI impl de uyuyor, derleyici secemez
    //   (E0282 "hic bilgi yok" demek; E0283 "birden fazla aday var" demek)

    println!("-- std: Add hem generic hem associated kullanir --");
    println!("  {}", Arrow { count: 30 } + Arrow { count: 12 });
    println!("  {}", Arrow { count: 30 } + 5u32);

    println!("-- ozet --");
    println!("  her tip icin cevap TEK ise      -> associated type (type Ammo)");
    println!("  ayni tip icin BIRDEN COK ise    -> generic parametre (Craft<T>)");
}
