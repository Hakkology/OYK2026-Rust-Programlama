// garden::tools  -- bu modul KLASOR + mod.rs stiliyle yazildi.
// garden.rs + garden/ stiliyle ayni sonucu verir; ikisi bir arada kullanilamaz.
pub mod shovel;

// pub(super): sadece BIR UST modul (garden) gorur, main.rs goremez
pub(super) fn inventory() {
    // self:: = bu modul (yazilmasa da olur, aciklik icin)
    println!("alet     : {:?}", self::shovel::Shovel::new());
}
