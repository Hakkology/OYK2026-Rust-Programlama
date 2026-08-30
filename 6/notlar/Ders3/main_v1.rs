// Gun 6 / Ders 3 (v1) - Standart Trait'leri ELLE Yazmak
// rustc main_v1.rs && ./main_v1
//
// Ayni ders, farkli dunya: bilet ve tarife.
// Gun 4'te derive ettiklerimizi bugun elle yaziyoruz.

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::ops::{Add, Mul, Sub};

// ---------------------------------------------------------------
// NEWTYPE: her buyukluk AYRI tip
// ---------------------------------------------------------------
// Ucret KURUS cinsinden tam sayi. Para f64 ile tutulmaz: 0.1 + 0.2 != 0.3
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Fare(i64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Distance(u32);            // metre

// Bolge tam sayi: Eq ve Ord DERIVE EDILEBILIYOR
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Zone(u8);

// Indirim f64 tutuyor: NaN yuzunden Eq/Ord ALAMAZ (Gun 4'teki kural)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Discount(f64);

// ---------------------------------------------------------------
// 1) Display - ELLE yazilir, derive EDILEMEZ
// ---------------------------------------------------------------
impl fmt::Display for Fare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:02} TL", self.0 / 100, self.0 % 100)
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 >= 1000 {
            write!(f, "{:.1} km", self.0 as f64 / 1000.0)
        } else {
            write!(f, "{} m", self.0)
        }
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. bolge", self.0)
    }
}

// ---------------------------------------------------------------
// 2) From - kayipsiz donusum. From yazinca Into BEDAVA gelir.
// ---------------------------------------------------------------
impl From<Zone> for Fare {
    fn from(z: Zone) -> Fare {
        Fare(750 + z.0 as i64 * 250)         // taban 7,50 + bolge basina 2,50
    }
}

impl From<Zone> for Distance {
    fn from(z: Zone) -> Distance {
        Distance(z.0 as u32 * 3000)          // bolge basina 3 km
    }
}

// ---------------------------------------------------------------
// 3) TryFrom - donusum BASARISIZ olabiliyorsa
// ---------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct NegativeFare(i64);

// Hata tipi de bir NEWTYPE: reddedilen degeri yaninda tasiyor.
// Gun 5'te ogrendigimiz sozlesmeyi tamamliyoruz: Display + Error.
impl fmt::Display for NegativeFare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "negatif ucret olmaz: {}", self.0)
    }
}

impl Error for NegativeFare {}

impl TryFrom<i64> for Fare {
    type Error = NegativeFare;

    fn try_from(v: i64) -> Result<Fare, Self::Error> {
        if v < 0 {
            Err(NegativeFare(v))
        } else {
            Ok(Fare(v))
        }
    }
}

// Error yazdigimiz icin ? bu hatayi Box<dyn Error>'a cevirebiliyor (Gun 5).
fn load_fare(raw: i64) -> Result<Fare, Box<dyn Error>> {
    let f = Fare::try_from(raw)?;
    Ok(f)
}

// ---------------------------------------------------------------
// 4) OPERATOR ASIRI YUKLEME: + - * birer trait
// ---------------------------------------------------------------
impl Add for Fare {
    type Output = Fare;                       // associated type: donus tipini trait belirliyor
    fn add(self, o: Fare) -> Fare { Fare(self.0 + o.0) }
}

impl Sub for Fare {
    type Output = Fare;
    fn sub(self, o: Fare) -> Fare {
        Fare((self.0 - o.0).max(0))           // ucret sifirin altina inmez
    }
}

// Farkli tiple carpma: indirim. Sag taraf generic parametre.
impl Mul<Discount> for Fare {
    type Output = Fare;
    fn mul(self, d: Discount) -> Fare {
        Fare((self.0 as f64 * d.0) as i64)
    }
}

// Bolge + gecilen bolge sayisi
impl Add<u8> for Zone {
    type Output = Zone;
    fn add(self, k: u8) -> Zone { Zone(self.0 + k) }
}

fn print_fare<T: Into<Fare>>(x: T) {
    println!("  {}", x.into());
}

fn main() {
    let tek_binis = Fare(1750);
    let ogrenci = Fare(850);
    let aktarma = Fare(250);

    println!("-- Display (elle yazildi) --");
    println!("  {} / {}", tek_binis, ogrenci);
    // Display yazinca to_string() BEDAVA gelir (std'deki blanket impl sayesinde)
    println!("  to_string(): {:?}", ogrenci.to_string());
    println!("  {} / {}", Distance(850), Distance(12400));

    println!("-- operatorler --");
    println!("  aktarmali yolculuk : {}", tek_binis + aktarma);
    println!("  ogrenci farki      : {}", tek_binis - ogrenci);
    println!("  %40 indirim        : {}", tek_binis * Discount(0.6));
    println!("  bolge atladi       : {}", Zone(2) + 1);
    println!("  sifirin altina inmez: {}", ogrenci - tek_binis);

    println!("-- From / Into --");
    let bolge = Zone(3);
    let ucret: Fare = Fare::from(bolge);          // From ile
    let mesafe: Distance = bolge.into();          // Into - yazmadiniz, geldi
    println!("  {} -> {} + {}", bolge, ucret, mesafe);
    // into() ayri bir mekanizma degil: bolge.into() derlendiginde Fare::from(bolge)
    // cagrisina donusuyor. Bir seyi guncellemez, YENI deger uretir.
    let bolge = bolge + 1;
    let ucret: Fare = bolge.into();
    println!("  bolge atlayinca: {} -> {}", bolge, ucret);

    println!("-- impl Into<T> parametresi --");
    print_fare(Zone(1));                          // Zone verilebilir
    print_fare(Fare(9999));                       // Fare de verilebilir (impl<T> From<T> for T)

    println!("-- TryFrom: basarisiz olabilen donusum --");
    println!("  {:?}", Fare::try_from(2500));
    println!("  {:?}", Fare::try_from(-30));
    match Fare::try_from(-30) {
        Ok(f) => println!("  gecerli: {}", f),
        Err(NegativeFare(v)) => println!("  gecersiz: {} - negatif ucret olmaz", v),
    }
    println!("  Display : {}", NegativeFare(-30));
    println!("  ? ile   : {:?}", load_fare(2500).map(|f| f.to_string()));
    match load_fare(-30) {
        Ok(_) => {}
        Err(e) => println!("  ? ile   : hata -> {}", e),
    }

    println!("-- Ord: Zone siralanabiliyor, Discount siralanamiyor --");
    let mut bolgeler = vec![Zone(3), Zone(1), Zone(5), Zone(2)];
    bolgeler.sort();                              // Ord derive edildi
    print!("  ");
    for b in &bolgeler { print!("{} | ", b); }
    println!();

    let mut indirimler = vec![Discount(0.6), Discount(0.85), Discount(0.5)];
    // indirimler.sort();
    //   E0277: the trait bound `Discount: Ord` is not satisfied
    //   -> f64 iceriyor, NaN yuzunden tam siralama yok
    indirimler.sort_by(|a, b| a.partial_cmp(b).unwrap());   // PartialOrd yetiyor
    println!("  indirimler: {:?}", indirimler);

    println!("-- Default --");
    println!("  {:?} / {}", Zone::default(), Zone::default());

    println!("-- tip guvenligi --");
    // let yanlis = tek_binis + Distance(500);
    //   E0308: expected `Fare`, found `Distance`
    //   Ucret ile mesafe ayri tip oldugu icin KARISAMAZLAR. Bedeli sifir:
    println!("  Fare = {} bayt, i64 = {} bayt",
        std::mem::size_of::<Fare>(), std::mem::size_of::<i64>());
}
