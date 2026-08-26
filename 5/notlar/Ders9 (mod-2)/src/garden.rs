// `garden` modulunun govdesi. Alt modulleri BURASI tanitir.
pub mod flowers;
pub mod tools;
pub mod vegetables;

// pub use = re-export. Kullanici garden::vegetables::Asparagus yerine
// garden::Asparagus da yazabilir.
pub use vegetables::Asparagus;

/// Bahcedeki her seyi ekiyormus gibi yapar.
pub fn plant_all() {
    // kendi alt modullerine GORELI yolla erisiyor
    // soil_check pub(super): "super" = garden, yani TAM DA BURASI cagirabilir
    vegetables::soil_check();
    println!("bahce    : {} sebze, {} cicek", vegetables::COUNT, flowers::COUNT);
    // tools::inventory() pub(super): "super" = garden, yani BURASI gorebilir
    tools::inventory();
}
