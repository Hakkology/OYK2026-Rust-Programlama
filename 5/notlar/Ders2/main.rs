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
fn parse_uzun(line: &str) -> Result<f64, TelemetryError> {
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
fn parse_kisa(line: &str) -> Result<f64, TelemetryError> {
    let esit = line.find('=').ok_or(TelemetryError::MissingField("sicaklik"))?;
    if &line[..esit] != "sicaklik" {
        return Err(TelemetryError::MissingField("sicaklik"));
    }
    let sayi: f64 = line[esit + 1..].parse()?;   // ParseFloatError -> From -> TelemetryError
    Ok(sayi)
}

// dogrulama ayri bir adim; ? ile zincirleniyor
fn dogrula(deger: f64) -> Result<f64, TelemetryError> {
    if deger < -125.0 || deger > 20.0 {
        return Err(TelemetryError::OutOfRange { field: "sicaklik", value: deger });
    }
    Ok(deger)
}

fn parse_ve_dogrula(line: &str) -> Result<f64, TelemetryError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(TelemetryError::EmptyLine);
    }
    dogrula(parse_kisa(line)?)                   // ? ifadenin ortasinda da kullanilir
}

// ?'in yapamadigi: BAGLAM eklemek. Satir numarasini biz sarmaliyoruz.
fn dosyayi_isle(icerik: &str) -> Result<Vec<f64>, TelemetryError> {
    let mut sonuc = Vec::new();
    for (i, satir) in icerik.lines().enumerate() {
        match parse_ve_dogrula(satir) {
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

// ? Option uzerinde de calisir - None ise erken doner
fn kullanici_adi(eposta: &str) -> Option<&str> {
    let at = eposta.find('@')?;
    eposta.get(0..at)
}

// Box<dyn Error>: "herhangi bir hata". Farkli tipler ayni kutuya girer.
fn kutulu(line: &str) -> Result<f64, Box<dyn Error>> {
    let d = parse_ve_dogrula(line)?;             // TelemetryError -> Box<dyn Error>
    let _ = "12".parse::<i32>()?;                // ParseIntError  -> Box<dyn Error>
    Ok(d)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("uzun yazim : {:?}", parse_uzun("sicaklik=-63.2"));
    println!("kisa yazim : {:?}", parse_kisa("sicaklik=-63.2"));
    println!("From ile   : {:?}", parse_kisa("sicaklik=abc"));

    println!("---");
    for s in ["sicaklik=-63.2", "sicaklik=999", "nem=40", ""] {
        match parse_ve_dogrula(s) {
            Ok(d) => println!("{:<16} -> {}", s, d),
            Err(e) => println!("{:<16} -> HATA: {}", s, e),   // Display kullaniliyor
        }
    }

    println!("---");
    let dosya = "sicaklik=-63.2\nsicaklik=-70.0\nsicaklik=abc\nsicaklik=-10";
    match dosyayi_isle(dosya) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("HATA: {}", e),        // "3. satir: 'abc' sayiya cevrilemedi"
    }

    println!("---");
    println!("Option'da ?  : {:?} {:?}", kullanici_adi("ada@mars.gov"), kullanici_adi("adamars"));
    println!("Box<dyn Error>: {:?}", kutulu("sicaklik=999").map_err(|e| e.to_string()));

    // main de Result dondurur: Err donerse cikis kodu 1 olur
    let son = parse_ve_dogrula("sicaklik=-40")?;
    println!("main'de ?    : {}", son);
    Ok(())
}
