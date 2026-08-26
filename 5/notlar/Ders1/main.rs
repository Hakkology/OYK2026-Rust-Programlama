// Gun 5 / Ders 1 - Hata Felsefesi ve Result
// rustc main.rs && ./main
// panik izini gormek icin: RUST_BACKTRACE=1 ./main

// Mars gezicisinden gelen telemetri satirlarini ayristiriyoruz:
//   "sicaklik=-63.2"   gecerli
//   "sicaklik=abc"     sayi degil
//   "sicaklik=999"     araligin disinda
//   "nem=40"           bekledigimiz alan degil
//   ""                 bos satir

// hata tipi bir ENUM - cagiran match ile ayirt edebilsin
#[derive(Debug)]
enum TelemetryError {
    EmptyLine,
    MissingField(&'static str),
    NotANumber(String),
    OutOfRange { field: &'static str, value: f64 },
}

fn parse_temperature(line: &str) -> Result<f64, TelemetryError> {
    let line = line.trim();

    if line.is_empty() {
        return Err(TelemetryError::EmptyLine);
    }

    // "sicaklik=-63.2" -> ("sicaklik", "-63.2")
    let esit = match line.find('=') {
        Some(i) => i,
        None => return Err(TelemetryError::MissingField("sicaklik")),
    };
    let (alan, deger) = (&line[..esit], &line[esit + 1..]);

    if alan != "sicaklik" {
        return Err(TelemetryError::MissingField("sicaklik"));
    }

    // parse bir Result dondurur; hata tipini KENDI hatamiza ceviriyoruz
    let sayi: f64 = match deger.parse() {
        Ok(n) => n,
        Err(_) => return Err(TelemetryError::NotANumber(deger.to_string())),
    };

    // Mars yuzeyi: -125 ile 20 C arasi
    if sayi < -125.0 || sayi > 20.0 {
        return Err(TelemetryError::OutOfRange { field: "sicaklik", value: sayi });
    }

    Ok(sayi)
}

fn main() {
    let satirlar = ["sicaklik=-63.2", "sicaklik=abc", "sicaklik=999", "nem=40", "   "];

    // match ile: cagiran her hatayi AYIRT EDEBILIYOR
    for s in satirlar {
        match parse_temperature(s) {
            Ok(d) => println!("{:<16} -> {} C", s, d),
            Err(TelemetryError::EmptyLine) => println!("{:<16} -> bos satir, atlandi", s),
            Err(TelemetryError::MissingField(f)) => println!("{:<16} -> '{}' alani yok", s, f),
            Err(TelemetryError::NotANumber(ham)) => println!("{:<16} -> '{}' sayi degil", s, ham),
            Err(TelemetryError::OutOfRange { field, value }) => {
                println!("{:<16} -> {} araligin disinda: {}", s, field, value)
            }
        }
    }

    println!("---");

    // hata tipi String olsaydi cagiran metin karsilastirmak zorunda kalirdi:
    //   if mesaj.contains("sayi degil") { ... }   <- kirilgan, dile bagli

    // Result uzerindeki sik metotlar
    let iyi = parse_temperature("sicaklik=-20");
    let kotu = parse_temperature("sicaklik=abc");

    println!("is_ok        {} {}", iyi.is_ok(), kotu.is_ok());
    println!("unwrap_or    {}", parse_temperature("sicaklik=abc").unwrap_or(0.0));
    println!("unwrap_or_else {}", parse_temperature("sicaklik=abc").unwrap_or_else(|_| -999.0));

    // ok() Result'i Option'a cevirir - HATA BILGISI COPE GIDER
    println!("ok()         {:?}", parse_temperature("sicaklik=abc").ok());

    // map_err hata tipini donusturur, Ok tarafina dokunmaz
    let metne: Result<f64, String> =
        parse_temperature("sicaklik=999").map_err(|e| format!("{:?}", e));
    println!("map_err      {:?}", metne);

    // Option -> Result: eksik olan "neden"i biz ekliyoruz
    let bos: Option<f64> = None;
    let r1: Result<f64, TelemetryError> = bos.ok_or(TelemetryError::EmptyLine);
    println!("ok_or        {:?}", r1);
    // ok_or_else TEMBELDIR: hata nesnesi sadece gerekirse uretilir
    let r2: Result<f64, TelemetryError> =
        bos.ok_or_else(|| TelemetryError::NotANumber(String::from("(hesaplandi)")));
    println!("ok_or_else   {:?}", r2);

    // expect: mesaj, "burada asla hata olamaz" varsayiminin BELGESIDIR
    let kesin = parse_temperature("sicaklik=0")
        .expect("sabit metin gecerli, ayristirma basarisiz olamaz");
    println!("expect       {}", kesin);

    // unwrap yerine expect yazin: panik mesaji sizin cumleniz olur
    // parse_temperature("sicaklik=abc").unwrap();   // panic: NotANumber("abc")

    // panik uretmenin diger yollari:
    //   panic!("mesaj")     dogrudan
    //   unreachable!()      "buraya asla gelinmez"
    //   todo!()             derlenir, cagrilirsa panikler - iskelet yazarken ideal
    //   assert!(kosul)      kosul bozulursa panikler
    assert!(kesin >= -125.0 && kesin <= 20.0, "sicaklik araligin disinda: {}", kesin);
    println!("assert gecti");
}
