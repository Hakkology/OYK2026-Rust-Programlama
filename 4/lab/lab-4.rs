// Gun 4 / Lab - Tiplerle Modelleme
// rustc lab-4.rs && ./lab-4
//
// Her gorevde TODO'lari doldurun; ustundeki ORNEK nasil calistigini gosteriyor.

fn main() {
    lab_1_option();
    lab_2_geometri();
    lab_3_enum();
    lab_4_gezinme();
}

// ---------------------------------------------------------------------------
// LAB 1 - Option<T>
// Option std'de tanimli siradan bir enum:  enum Option<T> { None, Some(T) }
// Rust'ta null YOK. "Olmayabilir" bilgisi TIPTE yaziyor.
// ---------------------------------------------------------------------------
fn lab_1_option() {
    println!("-- lab 1 --");

    let olcumler = vec![21.5, 22.0, 19.8, 25.3];

    // ORNEK: get sinir disinda None doner
    println!("{:?} {:?}", olcumler.get(1), olcumler.get(99));

    // ORNEK: icini cikarmanin uc yolu
    println!("{}", olcumler.get(1).copied().unwrap());        // yoksa PANIC
    println!("{}", olcumler.get(99).copied().unwrap_or(0.0)); // yoksa varsayilan

    // ORNEK: Option donduren fonksiyon - bulamazsa None
    println!("{:?} {:?}", en_sicak(&olcumler), en_sicak(&[]));

    // ORNEK: 0 ile "yok" ayni sey degil
    let sensorler = [Some(0.0), None];
    for s in &sensorler {
        match s {
            Some(d) => println!("okundu: {}", d),
            None => println!("okunamadi"),
        }
    }

    // TODO 1a: fn ilk_donma(s: &[f64]) -> Option<f64>
    //          sifirin altindaki ILK olcumu dondursun, yoksa None
    //          ipucu: dongude bulunca return Some(x), fonksiyon sonunda None

    // TODO 1b: fn bol(a: f64, b: f64) -> Option<f64>
    //          b sifirsa None dondursun. Sonucu match ile yazdirin.

    // TODO 1c: asagidaki dorttte fark ne? Her birini deneyin, ne zaman
    //          hangisini kullanacaginizi bir cumleyle yazin:
    //          unwrap()   expect("mesaj")   unwrap_or(0.0)   unwrap_or_default()

    // TODO 1d: let bulunan: Option<String> = ... tanimlayin,
    //          icini once if let ile, sonra match ile yazdirin
}

fn en_sicak(olcumler: &[f64]) -> Option<f64> {
    if olcumler.is_empty() {
        return None;                    // olcum yoksa "en sicak" diye bir sey de yok
    }
    let mut enb = olcumler[0];
    for o in olcumler {
        if *o > enb {
            enb = *o;
        }
    }
    Some(enb)
}

// ---------------------------------------------------------------------------
// LAB 2 - Nokta, Vektor (struct + impl)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Nokta {
    x: f64,
    y: f64,
}

impl Nokta {
    fn yeni(x: f64, y: f64) -> Nokta {
        Nokta { x, y }
    }

    // ORNEK: &self sadece okur
    fn uzaklik(&self, digeri: &Nokta) -> f64 {
        let dx = self.x - digeri.x;
        let dy = self.y - digeri.y;
        (dx * dx + dy * dy).sqrt()
    }

    // TODO 2a: fn orta_nokta(&self, digeri: &Nokta) -> Nokta
    //          iki noktanin tam ortasindaki noktayi dondursun

    // TODO 2b: fn olcekle(&mut self, kat: f64)
    //          x ve y'yi kat ile carpsin. Neden &mut self?

    // TODO 2c: fn ceyrek(&self) -> u8
    //          noktanin hangi ceyrekte oldugunu dondursun (1-4), eksen uzerindeyse 0
    //          ipucu: match (self.x > 0.0, self.y > 0.0)
}

fn lab_2_geometri() {
    println!("-- lab 2 --");

    let a = Nokta::yeni(0.0, 0.0);
    let b = Nokta::yeni(3.0, 4.0);
    println!("{:?} {:?} uzaklik={}", a, b, a.uzaklik(&b));

    let yol = vec![
        Nokta::yeni(0.0, 0.0),
        Nokta::yeni(3.0, 4.0),
        Nokta::yeni(3.0, 8.0),
        Nokta::yeni(-1.0, 8.0),
    ];
    println!("{} nokta", yol.len());

    // TODO 2d: yol uzunlugunu hesaplayin (ardisik noktalar arasi toplam uzaklik)
    //          ipucu: for i in 1..yol.len()

    // TODO 2e: yoldaki hangi nokta baslangica en uzak? Indeksini yazdirin.

    // TODO 2f: tuple struct'larla birim guvenligi:
    //          struct Metre(f64);  struct Ayak(f64);
    //          fn ayaktan_metreye(a: Ayak) -> Metre   (1 ayak = 0.3048 metre)
    //          Sonra bir Metre degerini ayaktan_metreye'ye vermeyi deneyin, hatayi okuyun.
}

// ---------------------------------------------------------------------------
// LAB 3 - enum + match
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
enum Isik {
    Kirmizi,
    Sari,
    Yesil,
}

impl Isik {
    // ORNEK: durum makinesi
    fn sonraki(&self) -> Isik {
        match self {
            Isik::Kirmizi => Isik::Yesil,
            Isik::Yesil => Isik::Sari,
            Isik::Sari => Isik::Kirmizi,
        }
    }
}

fn lab_3_enum() {
    println!("-- lab 3 --");

    let mut isik = Isik::Kirmizi;
    for _ in 0..4 {
        print!("{:?} -> ", isik);
        isik = isik.sonraki();
    }
    println!("{:?}", isik);

    // TODO 3a: enum Sekil tanimlayin - Cember { r }, Kare { kenar }, Ucgen(f64, f64, f64)
    //          impl Sekil { fn alan(&self) -> f64 }  yazin, match ile

    // TODO 3b: birkac sekli bir Vec'e koyup en buyuk alanliyi bulun

    // TODO 3c: DNA - enum Baz { A, T, G, C }
    //          fn tamamlayici(b: &Baz) -> Baz     (A-T, G-C eslesir)
    //          fn tamamlayici_dizi(d: &[Baz]) -> Vec<Baz>
    //          Sonra dizinin GC oranini yuzde olarak yazdirin.

    // TODO 3d: Isik'a YanipSonen diye YENI bir varyant ekleyin ve derleyin.
    //          Kac ayri yerde hata aldiniz? Hata kodu neydi?
    //          Ayni deneyi `_ => ...` dali ekleyerek tekrarlayin - ne degisti?
    //          Dersin en onemli sorusu bu: derleyici sizin icin ne yapti?
}

// ---------------------------------------------------------------------------
// LAB 4 - Gezinme: iter / iter_mut / into_iter
//   for x in &v      = v.iter()       -> &T,      liste bizde kalir
//   for x in &mut v  = v.iter_mut()   -> &mut T,  liste bizde kalir
//   for x in v       = v.into_iter()  -> T,       liste TUKENIR
// ---------------------------------------------------------------------------
fn lab_4_gezinme() {
    println!("-- lab 4 --");

    let noktalar = vec![
        Nokta::yeni(1.0, 1.0),
        Nokta::yeni(4.0, 5.0),
        Nokta::yeni(-2.0, 3.0),
    ];

    // ORNEK: okumak - liste bizde kalir
    for n in &noktalar {
        print!("({}, {}) ", n.x, n.y);
    }
    println!();
    println!("{} nokta hala elimizde", noktalar.len());

    // TODO 4a: iter_mut ile tum noktalari 2 kat olcekleyin
    //          (once let mut ile yeni bir liste yapin)
    //          ipucu: for n in &mut liste { n.x *= 2.0; ... }

    // TODO 4b: enumerate ile "0. nokta: (1, 1)" seklinde yazdirin

    // TODO 4c: baslangica en yakin noktanin INDEKSINI bulun
    //          ipucu: enumerate + uzaklik + bir "en iyi" degiskeni

    // TODO 4d: asagidaki blok DERLENMIYOR. Once nedenini soyleyin,
    //          sonra TEK karakterle duzeltin:
    //   let adlar = vec![String::from("ada"), String::from("ege")];
    //   for a in adlar {
    //       println!("{}", a);
    //   }
    //   println!("{:?}", adlar);

    // TODO 4e: into_iter gercekten gerektigi durum:
    //          adlar listesini TUKETEREK her ismi buyuk harfe ceviren
    //          yeni bir Vec<String> uretin
}
