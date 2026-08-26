// IKINCI IKILI:  cargo run --bin report_cli
//
// Bu program, kutuphanedeki `report` MODULUNU dogrudan kullaniyor.
// Modul yolu acikca goruluyor: ders3::report::summary
//
//   src/report/mod.rs      <- modul burada tanimli
//   src/bin/report_cli.rs  <- bu dosya onu kullaniyor
//
// Bunun calismasinin sebebi lib.rs'te `pub mod report;` yazmasidir.
// Sadece `mod report;` yazsaydik: error[E0603] module `report` is private
use ders3::report::{summary, table};
use ders3::telemetry::{parse, Reading};

fn main() {
    let ham = "sicaklik=-63.2\nsicaklik=-70.0\nsicaklik=abc\nsicaklik=-10\nsicaklik=999";

    let mut olcumler: Vec<Reading> = Vec::new();
    let mut hatali = 0;
    for satir in ham.lines() {
        match parse(satir) {
            Ok(r) => olcumler.push(r),
            Err(_) => hatali += 1,
        }
    }

    println!("=== TELEMETRI RAPORU ===");
    println!("{}", summary(&olcumler));       // report::summary
    println!("atlanan satir: {}", hatali);
    println!();
    print!("{}", table(&olcumler));           // report::table

    // Modulun ICINDEKI her sey acik degil: asagidaki satir DERLENMEZ.
    // ders3::report::summary::internal_label();
    //   -> pub(crate): kutuphane icinde gorunur, disaridan gorunmez
}
