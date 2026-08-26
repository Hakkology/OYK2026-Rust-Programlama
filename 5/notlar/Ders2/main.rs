// Gun 5 / Ders 2 - Hata Yayma: ? ve From
// rustc main.rs && ./main

use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
enum TelemetryError {
    EmptyLine,
    MissingField(&'static str),
    NotANumber(String),
    OutOfRange { field: &'static str, value: f64 },
    AtLine { line_no: usize, source: Box<TelemetryError> },
}

// Display: hatanin KULLANICIYA gosterilen hali. Elle yazilir.
impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TelemetryError::EmptyLine => write!(f, "bos satir"),
            TelemetryError::MissingField(a) => write!(f, "'{}' alani yok", a),
            TelemetryError::NotANumber(h) => write!(f, "sayiya cevrilemedi ({})", h),
            TelemetryError::OutOfRange { field, value } => {
                write!(f, "{} araligin disinda: {}", field, value)
            }
            TelemetryError::AtLine { line_no, source } => {
                write!(f, "{}. satir: {}", line_no, source)
            }
        }
    }
}

// std::error::Error'i uygulayinca hata tipimiz Box<dyn Error> kutusuna girebilir
impl Error for TelemetryError {}

// ?'in arkasindaki mekanizma: bir kez yaz, her yerde calissin
impl From<ParseFloatError> for TelemetryError {
    fn from(e: ParseFloatError) -> Self {
        TelemetryError::NotANumber(e.to_string())
    }
}

// ---- ? OLMADAN: uc kat derinlik, is kayboluyor ----
fn parse_long(line: &str) -> Result<f64, TelemetryError> {
    let esit = match line.find('=') {
        Some(i) => i,
        None => return Err(TelemetryError::MissingField("sicaklik")),
    };
    if &line[..esit] != "sicaklik" {
        return Err(TelemetryError::MissingField("sicaklik"));
    }
    let sayi = match line[esit + 1..].parse::<f64>() {
        Ok(n) => n,
        Err(e) => return Err(TelemetryError::from(e)),
    };
    Ok(sayi)
}

// ---- ? ILE: ayni is, iki satir ----
fn parse_short(line: &str) -> Result<f64, TelemetryError> {
    let esit = line.find('=').ok_or(TelemetryError::MissingField("sicaklik"))?;
    if &line[..esit] != "sicaklik" {
        return Err(TelemetryError::MissingField("sicaklik"));
    }
    let sayi: f64 = line[esit + 1..].parse()?;   // ParseFloatError -> From -> TelemetryError
    Ok(sayi)
}

// dogrulama ayri bir adim; ? ile zincirleniyor
fn validate(deger: f64) -> Result<f64, TelemetryError> {
    if deger < -125.0 || deger > 20.0 {
        return Err(TelemetryError::OutOfRange { field: "sicaklik", value: deger });
    }
    Ok(deger)
}

fn parse_and_validate(line: &str) -> Result<f64, TelemetryError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(TelemetryError::EmptyLine);
    }
    validate(parse_short(line)?)                   // ? ifadenin ortasinda da kullanilir
}

// ?'in yapamadigi: BAGLAM eklemek. Satir numarasini biz sarmaliyoruz.
fn process_file(icerik: &str) -> Result<Vec<f64>, TelemetryError> {
    let mut sonuc = Vec::new();
    for (i, satir) in icerik.lines().enumerate() {
        match parse_and_validate(satir) {
            Ok(d) => sonuc.push(d),
            Err(e) => {
                return Err(TelemetryError::AtLine {
                    line_no: i + 1,
                    source: Box::new(e),
                })
            }
        }
    }
    Ok(sonuc)
}

// ?'IN TAKILDIGI IKI YER - ikisi de yorumda, acip mesaji okuyun

// Engel 1: kap uyusmazligi. Result donduren fonksiyonda Option'a ? koyduk.
// fn engel1(s: &str) -> Result<usize, TelemetryError> {
//     let i = s.find('=')?;        // find Option doner
//     Ok(i)
// }
// error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s,
//               in a function that returns `Result`
// Cozum: .ok_or(TelemetryError::MissingField("sicaklik"))?

// Engel 2: hata tipi uyusmazligi. From yoksa ? cevirim yapamaz.
// fn engel2(s: &str) -> Result<i32, TelemetryError> {
//     let n: i32 = s.parse()?;     // ParseIntError -> TelemetryError cevrimi yok
//     Ok(n)
// }
// error[E0277]: `?` couldn't convert the error to `TelemetryError`
//               the trait `From<ParseIntError>` is not implemented for `TelemetryError`
// Cozum: yukaridaki From<ParseFloatError> gibi bir impl daha yazmak

// ? Option uzerinde de calisir - None ise erken doner
fn username(eposta: &str) -> Option<&str> {
    let at = eposta.find('@')?;
    eposta.get(0..at)
}

// Box<dyn Error>: "herhangi bir hata". Farkli tipler ayni kutuya girer.
fn boxed(line: &str) -> Result<f64, Box<dyn Error>> {
    let d = parse_and_validate(line)?;             // TelemetryError -> Box<dyn Error>
    let _ = "12".parse::<i32>()?;                // ParseIntError  -> Box<dyn Error>
    Ok(d)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:<15}{:?}", "uzun yazim", parse_long("sicaklik=-63.2"));
    println!("{:<15}{:?}", "kisa yazim (?)", parse_short("sicaklik=-63.2"));
    println!("{:<15}{:?}", "From ile", parse_short("sicaklik=abc"));

    println!("---");
    for s in ["sicaklik=-63.2", "sicaklik=999", "nem=40", ""] {
        match parse_and_validate(s) {
            Ok(d) => println!("{:<16} -> {}", s, d),
            Err(e) => println!("{:<16} -> HATA: {}", s, e),   // Display kullaniliyor
        }
    }

    println!("---");
    let dosya = "sicaklik=-63.2\nsicaklik=-70.0\nsicaklik=abc\nsicaklik=-10";
    match process_file(dosya) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("HATA: {}", e),        // "3. satir: 'abc' sayiya cevrilemedi"
    }

    println!("---");
    println!("{:<15}{:?} {:?}", "Option'da ?", username("ada@mars.gov"), username("adamars"));
    println!("{:<15}{:?}", "Box<dyn Error>", boxed("sicaklik=999").map_err(|e| e.to_string()));

    // main de Result dondurur: Err donerse cikis kodu 1 olur
    let son = parse_and_validate("sicaklik=-40")?;
    println!("{:<15}{}", "main'de ?", son);
    Ok(())
}
