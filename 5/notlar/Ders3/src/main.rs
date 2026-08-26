// Ince kabuk. Is mantigi kutuphanede; burasi cagirir ve yazdirir.
//
// main.rs ile lib.rs AYRI crate'lerdir: kutuphaneye PAKET ADIYLA erisilir.
use ders3::{in_range, parse, summary, table, Reading};

fn main() {
    let satirlar = ["sicaklik=-63.2", "sicaklik=-70.0", "sicaklik=999", "nem=40"];

    let mut olcumler: Vec<Reading> = Vec::new();
    for s in satirlar {
        match parse(s) {
            Ok(r) => olcumler.push(r),
            Err(e) => println!("{:<16} -> atlandi: {}", s, e),
        }
    }

    println!("{}", summary(&olcumler));
    println!();
    print!("{}", table(&olcumler));
    println!("in_range(-63.2) = {}", in_range(-63.2));

    // --- NE ACIK, NE KAPALI ---

    // 1) UZUN YOL CALISIR: lib.rs'te `pub mod telemetry;` yazdik
    println!("uzun yol        = {:?}", ders3::telemetry::parser::parse("sicaklik=0").is_ok());
    // 2) KISA YOL da calisir: lib.rs'te `pub use telemetry::parse;` var
    println!("kisa yol        = {:?}", parse("sicaklik=0").is_ok());

    // 3) BU DERLENMEZ - calibrate re-export EDILMEDI:
    // ders3::calibrate(-63.2);
    //   error[E0425]: cannot find function `calibrate` in crate `ders3`
    // ama modul yolu aciktir, sunu yazabilirsiniz:
    println!("modul yoluyla   = {:.3}", ders3::telemetry::calibrate(-63.2));

    // 4) BU DA DERLENMEZ - alan private:
    // let sahte = Reading { value: 5.0 };
    //   error[E0451]: field `value` of struct `Reading` is private

    // 5) BU DA DERLENMEZ - pub(crate) oge disariya cikmaz:
    // ders3::report::summary::internal_label();
    //   error[E0603]: function `internal_label` is private
}
