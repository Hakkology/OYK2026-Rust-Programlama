// Gun 6 / Ders 5 (v1) - Associated Types ve "Generic mi, Associated mi?"
// rustc main_v1.rs && ./main_v1
//
// Ayni ulasim dunyasi: araclar, biletler, tarife donusumleri.

use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct BoatPass { crossings: u8 }

#[derive(Debug)]
struct SmartCard { balance_kurus: u32 }

#[derive(Debug)]
struct PaperTicket { valid_minutes: u16 }

impl fmt::Display for BoatPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{} gecislik vapur karti", self.crossings) }
}
impl fmt::Display for SmartCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "kentkart, {} krs bakiye", self.balance_kurus) }
}
impl fmt::Display for PaperTicket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{} dk gecerli kagit bilet", self.valid_minutes) }
}

struct Ferry;
struct Metro;
struct Tram;

// ---------------------------------------------------------------
// 1) ASSOCIATED TYPE: "her tip icin TEK dogru cevap"
// ---------------------------------------------------------------
// Vapur vapur karti keser. Metro kentkart okutur. Tramvay kagit bilet verir.
// Her aracin bilet tipi TEKTIR -> associated type.
trait Vehicle {
    type Ticket;                            // trait'in ICINDE tanimli tip
    fn issue(&self) -> Self::Ticket;
}

impl Vehicle for Ferry {
    type Ticket = BoatPass;                 // Ferry icin cevap: BoatPass
    fn issue(&self) -> BoatPass { BoatPass { crossings: 10 } }
}

impl Vehicle for Metro {
    type Ticket = SmartCard;                // Metro icin cevap: SmartCard
    fn issue(&self) -> SmartCard { SmartCard { balance_kurus: 5000 } }
}

impl Vehicle for Tram {
    type Ticket = PaperTicket;
    fn issue(&self) -> PaperTicket { PaperTicket { valid_minutes: 90 } }
}

// Ayni tipe IKINCI kez implemente edilemez:
// impl Vehicle for Ferry { type Ticket = PaperTicket; ... }
//   E0119: conflicting implementations of trait `Vehicle` for type `Ferry`

// Ayni bileti kullanan ikinci bir arac - dyn ornegi icin
struct Funicular;

impl Vehicle for Funicular {
    type Ticket = SmartCard;
    fn issue(&self) -> SmartCard { SmartCard { balance_kurus: 1500 } }
}

// Associated type'i bound icinde kullanmak:
fn show_ticket<V>(v: &V)
where
    V: Vehicle,
    V::Ticket: fmt::Display,                // "bilet tipi yazdirilabilir olsun"
{
    println!("  kesildi: {}", v.issue());
}

// ---------------------------------------------------------------
// 2) GENERIC PARAMETRE: "ayni tip icin BIRDEN COK cevap"
// ---------------------------------------------------------------
#[derive(Debug)]
struct Fare(u32);

#[derive(Debug)]
struct Distance(u32);

#[derive(Debug, Clone, Copy)]
struct Zone(u8);

// Bir bolgeden HEM ucret HEM mesafe cikarilabilir: cevap birden fazla -> generic.
trait Estimate<T> {
    fn estimate(&self) -> T;
}

impl Estimate<Fare> for Zone {
    fn estimate(&self) -> Fare { Fare(750 + self.0 as u32 * 250) }
}

impl Estimate<Distance> for Zone {          // AYNI tip, IKINCI impl - generic sayesinde
    fn estimate(&self) -> Distance { Distance(self.0 as u32 * 3000) }
}

// ---------------------------------------------------------------
// 3) STD'DEN ORNEKLER
// ---------------------------------------------------------------
// Add trait'i ikisini birden kullanir:
//   trait Add<Rhs = Self> { type Output; }
//   Rhs GENERIC   -> sag taraf birden cok olabilir
//   Output ASSOCIATED -> belirli bir sol/sag cifti icin sonuc TEKTIR
impl Add for Fare {
    type Output = Fare;
    fn add(self, o: Fare) -> Fare { Fare(self.0 + o.0) }
}

impl Add<u32> for Fare {                    // Fare + u32 de tanimlanabilir
    type Output = Fare;
    fn add(self, k: u32) -> Fare { Fare(self.0 + k) }
}

impl fmt::Display for Fare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:02} TL", self.0 / 100, self.0 % 100)
    }
}

fn main() {
    let ferry = Ferry;
    let metro = Metro;
    let tram = Tram;

    println!("-- associated type --");
    show_ticket(&ferry);
    show_ticket(&metro);
    show_ticket(&tram);

    // Donen tipi derleyici biliyor: Ferry -> BoatPass
    let bilet: BoatPass = ferry.issue();
    println!("  tip belli: {:?}", bilet);

    println!("-- generic parametre: ayni girdiden iki sonuc --");
    let bolge = Zone(3);
    let ucret: Fare = bolge.estimate();      // hangi impl? donus tipi soyluyor
    let mesafe: Distance = bolge.estimate();
    println!("  {} bolgeden: {} ucret, {} metre", bolge.0, ucret, mesafe.0);
    // let x = bolge.estimate();
    //   E0283: type annotations needed - IKI impl de uyuyor, derleyici secemez
    //   (E0282 "hic bilgi yok" demek; E0283 "birden fazla aday var" demek)

    println!("-- std: Add hem generic hem associated kullanir --");
    println!("  {}", Fare(1750) + Fare(250));
    println!("  {}", Fare(1750) + 500u32);

    println!("-- associated type ve dyn --");
    // let v: Box<dyn Vehicle> = Box::new(Ferry);
    //   E0191: the value of the associated type `Ticket` must be specified
    //   dyn derken somut tipi unutuyoruz; Ticket'in ne oldugu yazilmali.
    let kentkartlilar: Vec<Box<dyn Vehicle<Ticket = SmartCard>>> =
        vec![Box::new(Metro), Box::new(Funicular)];
    for v in &kentkartlilar {
        println!("  {}", v.issue());
    }
    // Ferry bu listeye giremez: onun Ticket'i BoatPass.

    println!("-- ozet --");
    println!("  her tip icin cevap TEK ise      -> associated type (type Ticket)");
    println!("  ayni tip icin BIRDEN COK ise    -> generic parametre (Estimate<T>)");
}
