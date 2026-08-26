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

// fn parse_temperature(line: &str) -> Result<f64, TelemetryError> {
//     let line = line.trim();
//     if line.is_empty() {
//         return Err(TelemetryError::EmptyLine);
//     }

//     let (alan, deger) = line
//         .split_once('=')
//         .filter(|&(k, _)| k == "sicaklik")
//         .ok_or(TelemetryError::MissingField("sicaklik"))?;

//     let sayi: f64 = deger
//         .parse()
//         .map_err(|_| TelemetryError::NotANumber(deger.to_string()))?;

//     if !(-125.0..=20.0).contains(&sayi) {
//         return Err(TelemetryError::OutOfRange {
//             field: "sicaklik",
//             value: sayi,
//         });
//     }

//     Ok(sayi)
// }

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

    println!("{:<18}{} {}", "is_ok", iyi.is_ok(), kotu.is_ok());

    // -----------------------------------------------------------------
    // KUTUYU ACMANIN YOLLARI - unwrap TEK yol degil, hatta EN KOTU yol
    // Asagidaki hepsi ayni bozuk girdiyle calisiyor: "sicaklik=abc"
    // -----------------------------------------------------------------

    // 1) unwrap()  -> Err ise PANIKLER, program orada durur.
    //    Sadece "burada asla hata olamaz" diyebiliyorsaniz.
    // println!("{}", parse_temperature("sicaklik=abc").unwrap());
    //    ^ yorumu acin: thread 'main' panicked ... NotANumber("abc")

    // 2) expect("...")  -> yine panikler AMA mesaji siz yazarsiniz.
    //    unwrap yerine hep bunu tercih edin: panik ciktisi ise yarar.
    // println!("{}", parse_temperature("sicaklik=abc").expect("sensor verisi bozuk"));

    // 3) unwrap_or(varsayilan)  -> Err ise varsayilani verir, PANIK YOK.
    //    "hata olursa su degeri kullan" diyebiliyorsaniz dogru secim.
    println!("{:<18}{}", "unwrap_or", parse_temperature("sicaklik=abc").unwrap_or(0.0));

    // 4) unwrap_or_else(|e| ...)  -> varsayilani HESAPLAYARAK uretir.
    //    Varsayilan pahaliysa ya da hataya bakip karar verecekseniz.
    println!("{:<18}{}", "unwrap_or_else", parse_temperature("sicaklik=abc").unwrap_or_else(|_| -999.0));

    // 5) unwrap_or_default()  -> tipin sifir degerini verir (f64 icin 0.0)
    println!("{:<18}{}", "unwrap_or_default", parse_temperature("sicaklik=abc").unwrap_or_default());

    // 6) match  -> iki durumu da ELLE ele alirsiniz. En acik, en uzun yol.
    let secim = match parse_temperature("sicaklik=abc") {
        Ok(d) => d,
        Err(_) => {
            println!("(bozuk olcum atlandi, son gecerli deger kullaniliyor)");
            -63.2
        }
    };
    println!("{:<18}{}", "match ile", secim);

    // ORNEKLERDE unwrap gorurseniz "burasi kisaltilmis" demektir.
    // Gercek kodda sirasiyla: match / unwrap_or* / expect. unwrap en sonda.

    // ok() Result'i Option'a cevirir - HATA BILGISI COPE GIDER
    println!("{:<18}{:?}", "ok()", parse_temperature("sicaklik=abc").ok());

    // map BASARI degerini donusturur, Err tarafina dokunmaz
    // (|c| ... bir closure: adi olmayan kucuk bir fonksiyon)
    let fahrenheit = parse_temperature("sicaklik=-40").map(|c| c * 9.0 / 5.0 + 32.0);
    println!("{:<18}{:?}   (-40 C = -40 F, tek kesisme noktasi)", "map", fahrenheit);
    // Err ise map hicbir sey yapmaz, hata aynen gecer
    println!("{:<18}{:?}   (Err ise map hicbir sey yapmaz)", "map + Err", parse_temperature("sicaklik=abc").map(|c| c * 2.0).is_err());

    // Option'da da ayni map var
    let bos_olcum: Option<f64> = None;
    println!("{:<18}{:?} {:?}", "Option map", Some(3.0).map(|c: f64| c * 2.0), bos_olcum.map(|c| c * 2.0));

    // map_err hata tipini donusturur, Ok tarafina dokunmaz
    let metne: Result<f64, String> =
        parse_temperature("sicaklik=999").map_err(|e| format!("{:?}", e));
    println!("{:<18}{:?}", "map_err", metne);
    // ikisi zincirlenebilir: bir goz Ok'i, digeri Err'i isler
    let zincir: Result<String, String> = parse_temperature("sicaklik=-63.2")
        .map(|c| format!("{:.1} C", c))
        .map_err(|e| format!("hata: {:?}", e));
    println!("{:<18}{:?}", "map + map_err", zincir);

    // Option -> Result: eksik olan "neden"i biz ekliyoruz
    let bos: Option<f64> = None;
    let r1: Result<f64, TelemetryError> = bos.ok_or(TelemetryError::EmptyLine);
    println!("{:<18}{:?}", "ok_or", r1);
    // ok_or_else TEMBELDIR: hata nesnesi sadece gerekirse uretilir
    let r2: Result<f64, TelemetryError> =
        bos.ok_or_else(|| TelemetryError::NotANumber(String::from("(hesaplandi)")));
    println!("{:<18}{:?}", "ok_or_else", r2);

    // expect: mesaj, "burada asla hata olamaz" varsayiminin BELGESIDIR
    let kesin = parse_temperature("sicaklik=0")
        .expect("sabit metin gecerli, ayristirma basarisiz olamaz");
    println!("{:<18}{}", "expect", kesin);

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
