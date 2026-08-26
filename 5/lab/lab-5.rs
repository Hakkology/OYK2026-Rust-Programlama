// Gun 5 / Lab - Hata Tipleri, Moduller ve Makrolar
// rustc lab-5.rs && ./lab-5
// testler:  rustc --test lab-5.rs -o test5 && ./test5
//
// Her gorevde TODO'lari doldurun; ustundeki ORNEK nasil calistigini gosteriyor.

// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan degisken/import uyarilari normal.
#![allow(unused)]

fn main() {
    lab_1_hata_tipi();
    lab_2_modul();
    lab_3_makro();
}

// ---------------------------------------------------------------------------
// LAB 1 - Kendi hata tipiniz ve ?
// Gun 4'te fonksiyonlar Option donduruyordu: "olmadi" ama NEDEN bilinmiyordu.
// Simdi Result ve nedeni tasiyan bir enum.
// ---------------------------------------------------------------------------
#[derive(Debug)]
enum RoverError {
    EmptyCommand,
    UnknownCommand(String),
    // TODO 1a: iki varyant daha ekleyin:
    //          BadDistance(String)                 -> sayiya cevrilemedi
    //          TooFar { requested: u32, max: u32 } -> mesafe limiti asildi
}

// ORNEK: ayristirma. "ilerle 120" -> 120 metre
fn parse_command(line: &str) -> Result<u32, RoverError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(RoverError::EmptyCommand);
    }

    let bosluk = match line.find(' ') {
        Some(i) => i,
        None => return Err(RoverError::UnknownCommand(line.to_string())),
    };
    let (komut, arg) = (&line[..bosluk], &line[bosluk + 1..]);

    if komut != "ilerle" {
        return Err(RoverError::UnknownCommand(komut.to_string()));
    }

    // TODO 1b: arg'i u32'ye cevirin.
    //          Basarisizsa RoverError::BadDistance(arg.to_string()) dondurun.
    //          Sonra ayni satiri `?` ile yazmayi deneyin:
    //          bunun icin `impl From<std::num::ParseIntError> for RoverError` yazin.
    let mesafe: u32 = 0;   // <- burayi degistirin

    // TODO 1c: mesafe 500'den buyukse TooFar dondurun (max: 500)

    Ok(mesafe)
}

fn lab_1_hata_tipi() {
    println!("-- lab 1 --");
    for k in ["ilerle 120", "ilerle abc", "ilerle 9000", "don 90", ""] {
        println!("{:<12} -> {:?}", k, parse_command(k));
    }

    // TODO 1d: impl std::fmt::Display for RoverError yazin, her varyant icin
    //          okunakli bir Turkce mesaj uretsin. Sonra yukaridaki {:?} yerine
    //          {} kullanin - fark ne?

    // TODO 1e: fn toplam_mesafe(komutlar: &[&str]) -> Result<u32, RoverError>
    //          Butun komutlari ayristirip toplasin, ILK hatada dursun (? kullanin).
    //          Sonra soru: ilk hatada durmak yerine hatalari toplamak isteseydik
    //          imza ne olurdu?
}

// ---------------------------------------------------------------------------
// LAB 2 - Moduller ve gorunurluk
// ---------------------------------------------------------------------------
mod task {
    // ORNEK: private alan, kontrollu kurucu
    pub struct Task {
        name: String,
        is_done: bool,
    }

    impl Task {
        pub fn new(name: &str) -> Task {
            Task { name: name.to_string(), is_done: false }
        }
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn complete(&mut self) {
            self.is_done = true;
        }
        pub fn is_done(&self) -> bool {
            self.is_done
        }
    }

    // TODO 2a: `report` adinda bir ALT MODUL ekleyin.
    //          Icinde pub fn summary(t: &Task) -> String olsun,
    //          "Krater orneklemesi: tamamlandi" gibi bir metin uretsin.
    //          Ipucu: alt modul ust modulun private alanlarini GORUR.

    // TODO 2b: report::summary'i pub(crate) yapin ve farki gozleyin.
}

fn lab_2_modul() {
    println!("-- lab 2 --");
    let mut g = task::Task::new("Krater orneklemesi");
    println!("{} tamamlandi mi: {}", g.name(), g.is_done());
    g.complete();
    println!("{} tamamlandi mi: {}", g.name(), g.is_done());

    // TODO 2c: asagidaki satirin yorumunu acin, hata kodunu okuyun, sonra
    //          neden boyle oldugunu bir cumleyle yazin:
    // let sahte = task::Task { name: String::from("x"), is_done: true };

    // TODO 2d: lab dosyasinin en ustune `pub use task::Task;` ekleyip
    //          burada sadece `Task::new(...)` yazabilir hale getirin.
}

// ---------------------------------------------------------------------------
// LAB 3 - macro_rules!
// ---------------------------------------------------------------------------

// ORNEK: degisken sayida arguman alan bir makro
macro_rules! report_line {
    ( $( $alan:expr ),* $(,)? ) => {{
        let mut s = String::new();
        $(
            s.push_str(&format!("{} | ", $alan));
        )*
        s
    }};
}

fn lab_3_makro() {
    println!("-- lab 3 --");
    println!("{}", report_line!("Rover", 29300, true));

    // TODO 3a: `min_of!` makrosu yazin.
    //          min_of!(3)          -> 3
    //          min_of!(3, 1, 2)    -> 1
    //          Ipucu: iki kol yazin, ikincisi ozyinelemeli olsun:
    //          ( $ilk:expr, $( $geri:expr ),+ ) => { ... min_of!( $( $geri ),+ ) ... }

    // TODO 3b: `to_celsius!` makrosu yazin: to_celsius!(98.6 F) ve to_celsius!(37.0 C)
    //          ikisi de Celsius degeri versin.
    //          Ipucu: birim icin ident yakalayin, iki ayri kol yazin.

    // TODO 3c: asagidaki iki makroyu deneyin ve SONUCLARI karsilastirin:
    //          macro_rules! kare_expr { ($x:expr) => { $x * $x }; }
    //          macro_rules! kare_tt   { ( $($x:tt)* ) => { $($x)* * $($x)* }; }
    //          Ikisini de kare!(2 + 3) ile cagirin. Sonuclar neden farkli?

    // TODO 3d (ileri): proc_ornek/ klasorundeki #[derive(Label)] makrosuna
    //          `pub fn field_names() -> Vec<&'static str>` uretmesini ekleyin.
    //          Ipucu: label_derive/src/lib.rs icinde govdeyi ':' yerine
    //          alan adlarina gore ayristirmaniz gerekecek.
}

#[cfg(test)]
mod tests {
    use super::*;

    // ORNEK: sinir degeri testi
    #[test]
    fn empty_command_errors() {
        assert!(matches!(parse_command(""), Err(RoverError::EmptyCommand)));
    }

    // TODO T1: "ilerle 120" icin Ok(120) donduruldugunu test edin
    //          (1b'yi tamamladiktan sonra gecer)

    // TODO T2: 500 ve 501 icin sinir davranisini test edin

    // TODO T3: #[should_panic] kullanan bir test yazin
    //          ipucu: parse_command("ilerle abc").unwrap()
}
