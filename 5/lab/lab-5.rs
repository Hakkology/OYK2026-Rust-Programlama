// Gun 5 / Lab - Hata Tipleri, Moduller ve Makrolar
// rustc lab-5.rs && ./lab-5
// testler:  rustc --test lab-5.rs -o test5 && ./test5
//
// Her gorevde TODO'lari doldurun; ustundeki ORNEK nasil calistigini gosteriyor.

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
mod gorev {
    // ORNEK: private alan, kontrollu kurucu
    pub struct Gorev {
        ad: String,
        tamamlandi: bool,
    }

    impl Gorev {
        pub fn yeni(ad: &str) -> Gorev {
            Gorev { ad: ad.to_string(), tamamlandi: false }
        }
        pub fn ad(&self) -> &str {
            &self.ad
        }
        pub fn tamamla(&mut self) {
            self.tamamlandi = true;
        }
        pub fn tamamlandi(&self) -> bool {
            self.tamamlandi
        }
    }

    // TODO 2a: `rapor` adinda bir ALT MODUL ekleyin.
    //          Icinde pub fn ozet(g: &Gorev) -> String olsun,
    //          "Perseverance: tamamlandi" gibi bir metin uretsin.
    //          Ipucu: alt modul ust modulun private alanlarini GORUR.

    // TODO 2b: rapor::ozet'i pub(crate) yapin ve farki gozleyin.
}

fn lab_2_modul() {
    println!("-- lab 2 --");
    let mut g = gorev::Gorev::yeni("Krater orneklemesi");
    println!("{} tamamlandi mi: {}", g.ad(), g.tamamlandi());
    g.tamamla();
    println!("{} tamamlandi mi: {}", g.ad(), g.tamamlandi());

    // TODO 2c: asagidaki satirin yorumunu acin, hata kodunu okuyun, sonra
    //          neden boyle oldugunu bir cumleyle yazin:
    // let sahte = gorev::Gorev { ad: String::from("x"), tamamlandi: true };

    // TODO 2d: lab dosyasinin en ustune `pub use gorev::Gorev;` ekleyip
    //          burada sadece `Gorev::yeni(...)` yazabilir hale getirin.
}

// ---------------------------------------------------------------------------
// LAB 3 - macro_rules!
// ---------------------------------------------------------------------------

// ORNEK: degisken sayida arguman alan bir makro
macro_rules! rapor_satiri {
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
    println!("{}", rapor_satiri!("Rover", 29300, true));

    // TODO 3a: `en_kucuk!` makrosu yazin.
    //          en_kucuk!(3)          -> 3
    //          en_kucuk!(3, 1, 2)    -> 1
    //          Ipucu: iki kol yazin, ikincisi ozyinelemeli olsun:
    //          ( $ilk:expr, $( $geri:expr ),+ ) => { ... en_kucuk!( $( $geri ),+ ) ... }

    // TODO 3b: `sicaklik_c!` makrosu yazin: sicaklik_c!(98.6 F) ve sicaklik_c!(37.0 C)
    //          ikisi de Celsius degeri versin.
    //          Ipucu: birim icin ident yakalayin, iki ayri kol yazin.

    // TODO 3c: asagidaki iki makroyu deneyin ve SONUCLARI karsilastirin:
    //          macro_rules! kare_expr { ($x:expr) => { $x * $x }; }
    //          macro_rules! kare_tt   { ( $($x:tt)* ) => { $($x)* * $($x)* }; }
    //          Ikisini de kare!(2 + 3) ile cagirin. Sonuclar neden farkli?

    // TODO 3d (ileri): proc_ornek/ klasorundeki #[derive(Etiket)] makrosuna
    //          `pub fn alan_adlari() -> Vec<&'static str>` uretmesini ekleyin.
    //          Ipucu: etiket_derive/src/lib.rs icinde govdeyi ':' yerine
    //          alan adlarina gore ayristirmaniz gerekecek.
}

#[cfg(test)]
mod tests {
    use super::*;

    // ORNEK: sinir degeri testi
    #[test]
    fn bos_komut_hata_verir() {
        assert!(matches!(parse_command(""), Err(RoverError::EmptyCommand)));
    }

    // TODO T1: "ilerle 120" icin Ok(120) donduruldugunu test edin
    //          (1b'yi tamamladiktan sonra gecer)

    // TODO T2: 500 ve 501 icin sinir davranisini test edin

    // TODO T3: #[should_panic] kullanan bir test yazin
    //          ipucu: parse_command("ilerle abc").unwrap()
}
