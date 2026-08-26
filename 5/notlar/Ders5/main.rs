// Gun 5 / Ders 5 - Prosedurel Makrolar (kullanan taraf)
// rustc main.rs && ./main
//
// Prosedurel makro AYRI BIR CRATE gerektirir, tek dosyada yazilamaz.
// Calisan tam ornek yanindaki klasorde:   cd proc_ornek && cargo run
// Bu dosya, prosedurel makrolari KULLANAN tarafi ve iki aile arasindaki
// farki gosteriyor.

use std::collections::HashMap;

// #[derive(...)] her biri bir PROSEDUREL MAKRODUR.
// Derleyici degil, o makrolar yaziyor bu kodu.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Rover {
    ad: String,
    mesafe_m: u32,
}

// declarative makro: desen -> sablon
macro_rules! rover {
    ($ad:expr, $m:expr) => {
        Rover { ad: String::from($ad), mesafe_m: $m }
    };
}

fn main() {
    let r = rover!("Perseverance", 29300);

    // Debug: #[derive(Debug)] uretti. Elle yazsaydik ~10 satir olurdu.
    println!("{:?}", r);
    println!("{:#?}", r);

    // Clone, PartialEq: yine derive
    let kopya = r.clone();
    println!("esit mi: {}", r == kopya);

    // Hash + Eq: HashMap anahtari olabiliyor
    let mut kayit: HashMap<Rover, &str> = HashMap::new();
    kayit.insert(r.clone(), "gorev suruyor");
    println!("{:?}", kayit.get(&kopya));

    // Default: alanlarin sifir degeri
    println!("{:?}", Rover::default());

    // Iki aile yan yana:
    //   macro_rules!  -> desen eslestirir, sablon uretir      (rover! makrosu)
    //   prosedurel    -> DERLEME ZAMANINDA Rust kodu calisir  (derive'lar)
    //
    // Prosedurel makro ne uretiyor gormek icin:
    //   cargo install cargo-expand && cargo expand
    //
    // Kendi derive makronuzu yazip calistirmak icin:
    //   cd proc_ornek && cargo run
    println!("kendi derive ornegimiz icin: cd proc_ornek && cargo run");
}
