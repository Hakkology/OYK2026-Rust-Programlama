// Ince kabuk: is mantigi kutuphanede, burasi sadece cagiriyor.
// Kutuphaneye PAKET ADIYLA erisilir.
use ornek_proje::{araligda, parse};

fn main() {
    for satir in ["sicaklik=-63.2", "sicaklik=999", "nem=40"] {
        match parse(satir) {
            Ok(r) => println!("{:<16} -> {}", satir, r.deger()),
            Err(e) => println!("{:<16} -> HATA: {}", satir, e),
        }
    }
    println!("araligda(0) = {}", araligda(0.0));
}
