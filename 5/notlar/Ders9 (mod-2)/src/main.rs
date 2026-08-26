// CRATE KOKU. Modul agaci bu dosyadan baslar.
//
//   src/main.rs                 <- crate koku
//   src/garden.rs               mod garden;
//   src/garden/vegetables.rs    garden.rs icinde: pub mod vegetables;
//   src/garden/flowers.rs       garden.rs icinde: pub mod flowers;
//   src/garden/tools/mod.rs     garden.rs icinde: pub mod tools;   (klasor + mod.rs stili)
//   src/garden/tools/shovel.rs  tools/mod.rs icinde: pub mod shovel;

// mod = modulu VAR EDER. Bu satir olmazsa garden.rs derlemeye girmez.
mod garden;

// use = uzun yolu KISALTIR. Islevsel bir sey yapmaz, sadece yazim kolayligi.
use crate::garden::vegetables::Asparagus;
use crate::garden::flowers::{Color, Rose};
// as ile yeniden adlandirma
use crate::garden::tools::shovel::Shovel as Kurek;

fn main() {
    // 1) use ile kisaltilmis
    let plant = Asparagus::new(30);
    println!("kuskonmaz  : {:?} ({} cm)", plant, plant.height_cm());

    // 2) use YAZMADAN, tam mutlak yol - ayni sey
    let domates = crate::garden::vegetables::Tomato::new("Cherry");
    println!("domates    : {:?}", domates);

    // 3) enum varyantlari, enum pub ise OTOMATIK pub olur
    let gul = Rose::new(Color::Red);
    println!("gul        : {:?}", gul);

    // 4) as ile takma ad
    println!("alet       : {:?}", Kurek::new());

    // 5) re-export: garden.rs'te `pub use vegetables::Asparagus;` yazdigi icin
    //    kisa yol da calisiyor - ic yapiyi bilmeye gerek yok
    let kisa = garden::Asparagus::new(12);
    println!("kisa yol   : {:?}", kisa);

    // 6) pub ALAN okunabilir, private alan okunamaz
    println!("cesit      : {}", domates.variety);
    println!("gul rengi  : {:?}", gul.color);
    println!("kurek boyu : {}", Kurek::new().size);

    // 7) enum varyantlari otomatik pub
    for renk in [Color::Red, Color::White, Color::Yellow] {
        print!("{:?} ", renk);
    }
    println!();

    // 8) modulun kendi ust duzey fonksiyonu
    garden::plant_all();

    // --- BUNLAR DERLENMEZ ---
    // garden::vegetables::soil_check();
    //   E0603: function `soil_check` is private
    // let _ = Asparagus { height_cm: 5 };
    //   E0451: field `height_cm` is private
    // garden::tools::inventory();
    //   E0603: function `inventory` is private (pub(super): sadece garden gorur)
}
