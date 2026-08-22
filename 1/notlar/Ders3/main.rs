// Gun 1 / Ders 3 - Cargo ve Ekosistem
// Bu dosyayi IKI KEZ calistirin, cikti degisir:
//   1) rustc main.rs && ./main
//   2) cargo new ders3 -> bu dosyayi ders3/src/main.rs'e kopyala -> cargo run

fn main() {
    // option_env! DERLEME zamaninda ortam degiskenine bakar
    // CARGO_PKG_* degiskenlerini Cargo tanimlar, duz rustc tanimlamaz
    match option_env!("CARGO_PKG_NAME") {
        Some(ad) => println!("cargo: {} v{}", ad, option_env!("CARGO_PKG_VERSION").unwrap_or("?")),
        None => println!("duz rustc, CARGO_PKG_NAME yok"),
    }

    let profil = if cfg!(debug_assertions) { "dev" } else { "release" };
    println!("{}", profil);

    // komut satiri argumanlari:  cargo run -- birinci ikinci
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    println!("{}", std::env::consts::OS);
    println!("{}", std::env::consts::ARCH);
}
