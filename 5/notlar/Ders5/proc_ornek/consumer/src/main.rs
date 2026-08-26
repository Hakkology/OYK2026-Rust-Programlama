// Makroyu kullanan taraf. Buradan bakinca #[derive(Label)] tek satir;
// arkasinda calisan sey label_derive crate'indeki Rust kodu.
use label_derive::Label;

#[derive(Label)]
struct Rover {
    name: String,
    distance_km: f64,
}

#[derive(Label)]
struct Satellite {
    id: u32,
}

fn main() {
    let r = Rover { name: String::from("Perseverance"), distance_km: 29.3 };
    let u = Satellite { id: 7 };

    // Bu metotlarin hicbirini biz yazmadik - makro uretti
    println!("{} -> {}", Rover::type_name(), r.label());
    println!("{} -> {}", Satellite::type_name(), u.label());
    println!("Rover kac alan: {}", Rover::field_count());
    println!("{} {} km", r.name, r.distance_km);
    println!("uydu id {}", u.id);
}
