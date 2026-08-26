pub mod dogrulama;                   // icerigi src/telemetri/dogrulama.rs dosyasinda

/// Dogrulanmis bir olcum.
pub struct Reading {
    deger: f64,                      // private: gecersiz Reading uretilemesin
}

impl Reading {
    pub fn deger(&self) -> f64 {
        self.deger
    }
}

/// "sicaklik=-63.2" satirini ayristirir.
///
/// # Ornek
/// ```
/// let r = ornek_proje::parse("sicaklik=-63.2").unwrap();
/// assert!(r.deger() < 0.0);
/// ```
pub fn parse(satir: &str) -> Result<Reading, String> {
    let esit = satir.find('=').ok_or_else(|| String::from("'=' yok"))?;
    if &satir[..esit] != "sicaklik" {
        return Err(String::from("'sicaklik' alani yok"));
    }
    let deger: f64 = satir[esit + 1..]
        .parse()
        .map_err(|_| format!("sayi degil: {}", &satir[esit + 1..]))?;

    if !dogrulama::araligda(deger) {
        return Err(format!("aralik disinda: {}", deger));
    }
    Ok(Reading { deger })            // ayni crate icindeyiz, private alani doldurabiliriz
}
