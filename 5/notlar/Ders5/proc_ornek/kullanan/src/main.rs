// Makroyu kullanan taraf. Buradan bakinca #[derive(Etiket)] tek satir;
// arkasinda calisan sey etiket_derive crate'indeki Rust kodu.
use etiket_derive::Etiket;

#[derive(Etiket)]
struct Rover {
    ad: String,
    mesafe_km: f64,
}

#[derive(Etiket)]
struct Uydu {
    id: u32,
}

fn main() {
    let r = Rover { ad: String::from("Perseverance"), mesafe_km: 29.3 };
    let u = Uydu { id: 7 };

    // Bu metotlarin hicbirini biz yazmadik - makro uretti
    println!("{} -> {}", Rover::tip_adi(), r.etiket());
    println!("{} -> {}", Uydu::tip_adi(), u.etiket());
    println!("Rover kac alan: {}", Rover::alan_sayisi());
    println!("{} {} km", r.ad, r.mesafe_km);
    println!("uydu id {}", u.id);
}
