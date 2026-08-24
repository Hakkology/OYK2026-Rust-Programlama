// Gun 4 / Ders 4 - Pattern Matching
// rustc main.rs && ./main

#[derive(Debug, Clone, Copy, PartialEq)]
enum Isik {
    Kirmizi,
    Sari,
    Yesil,
}

#[derive(Debug)]
enum Sekil {
    Cember { r: f64 },
    Dikdortgen { en: f64, boy: f64 },
    Ucgen(f64, f64, f64),
}

// oyun/arayuz olaylari - varyantlar farkli sekilde
#[derive(Debug)]
enum Olay {
    Tus(char),
    Tiklama { x: i32, y: i32 },
    Kaydirma(i32),
    Cikis,
}

// satranc karesi
#[derive(Debug)]
struct Kare {
    satir: u8,
    sutun: u8,
}

fn main() {
    // EXHAUSTIVENESS - tum varyantlar ele alinmak zorunda
    for isik in [Isik::Kirmizi, Isik::Sari, Isik::Yesil] {
        let davranis = match isik {
            Isik::Kirmizi => "dur",
            Isik::Sari => "hazirlan",
            Isik::Yesil => "gec",
            // birini silin -> E0004 non-exhaustive patterns
        };
        print!("{:?}->{} ", isik, davranis);
    }
    println!();
    let isik = Isik::Sari;

    // deger ve ARALIK desenleri
    for zar in 1..=6 {
        let yorum = match zar {
            1 => "en kotu",
            2..=4 => "orta",
            5 | 6 => "iyi",             // | ile coklu desen
            _ => "zar boyle olmaz",
        };
        print!("{}:{} ", zar, yorum);
    }
    println!();

    // guard - desene ek kosul (klasik FizzBuzz)
    for n in 1..=15 {
        let s = match n {
            n if n % 15 == 0 => String::from("FizzBuzz"),
            n if n % 3 == 0 => String::from("Fizz"),
            n if n % 5 == 0 => String::from("Buzz"),
            n => n.to_string(),
        };
        print!("{} ", s);
    }
    println!();

    // coklu desen - Turkce sesli harfler
    let kelime = "cumhuriyet";
    let mut sesli = 0;
    for h in kelime.chars() {
        match h {
            'a' | 'e' | 'i' | 'o' | 'u' | 'ı' | 'ö' | 'ü' => sesli += 1,
            _ => {}
        }
    }
    println!("{} -> {} sesli harf", kelime, sesli);

    // @ ile hem eslesip hem degeri yakalamak
    for puan in [95, 72, 30] {
        let sonuc = match puan {
            p @ 90..=100 => format!("mukemmel ({})", p),
            p @ 50..=89 => format!("gecer ({})", p),
            p => format!("kaldi ({})", p),
        };
        print!("{} | ", sonuc);
    }
    println!();

    // TUPLE destructuring - koordinat duzlemi
    for nokta in [(0, 0), (0, 5), (3, 0), (2, 7)] {
        let yer = match nokta {
            (0, 0) => String::from("orijin"),
            (0, y) => format!("y ekseninde, y={}", y),
            (x, 0) => format!("x ekseninde, x={}", x),
            (x, y) => format!("duzlemde ({}, {})", x, y),
        };
        print!("{} | ", yer);
    }
    println!();

    // STRUCT destructuring - bazi alanlari sabitle, bazilarini yakala
    let kareler = [
        Kare { satir: 1, sutun: 4 },
        Kare { satir: 8, sutun: 8 },
        Kare { satir: 5, sutun: 3 },
    ];
    for k in &kareler {
        match k {
            Kare { satir: 1, sutun } => println!("beyaz taban, {}. sutun", sutun),
            Kare { satir: 8, sutun } => println!("siyah taban, {}. sutun", sutun),
            Kare { satir, .. } => println!("{}. sirada bir kare", satir),
        }
    }

    // ENUM destructuring - varyantin verisini cikar
    let olaylar = vec![
        Olay::Tus('q'),
        Olay::Tiklama { x: 120, y: 45 },
        Olay::Kaydirma(-3),
        Olay::Cikis,
    ];
    for o in &olaylar {
        match o {
            Olay::Tus(k) => println!("tusa basildi: {}", k),
            Olay::Tiklama { x, y } => println!("tiklama: ({}, {})", x, y),
            Olay::Kaydirma(miktar) if *miktar < 0 => println!("asagi kaydirma: {}", miktar),
            Olay::Kaydirma(miktar) => println!("yukari kaydirma: {}", miktar),
            Olay::Cikis => println!("cikis"),
        }
    }

    // match hem "hangisi" sorusunu cevaplar hem icindekini cikarir
    let sekiller = vec![
        Sekil::Cember { r: 1.5 },
        Sekil::Dikdortgen { en: 2.0, boy: 5.0 },
        Sekil::Ucgen(3.0, 4.0, 5.0),
    ];
    for s in &sekiller {
        let alan = match s {
            Sekil::Cember { r } => 3.14159 * r * r,
            Sekil::Dikdortgen { en, boy } => en * boy,
            Sekil::Ucgen(a, b, c) => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        };
        println!("{:<32} alan={:.2}", format!("{:?}", s), alan);
    }

    // if let - tek dalla ilgileniyorsak
    let bulunan: Option<char> = kelime.chars().next();
    if let Some(ilk) = bulunan {
        println!("ilk harf: {}", ilk);
    }

    // let else - eslesmezse ERKEN CIK, gerisi duz aksin
    let kayitlar = vec![21.5, 22.0, 19.8];
    ortalama_yazdir(&kayitlar);
    ortalama_yazdir(&[]);

    // while let - eslestigi surece don
    let mut yigin = vec![1, 2, 3];
    while let Some(ust) = yigin.pop() {
        print!("{} ", ust);
    }
    println!();

    // matches! - sadece "esliyor mu", bool doner
    println!("{} {}", matches!(isik, Isik::Sari), matches!(isik, Isik::Yesil));

    // DESENDE SAHIPLIK - & ile odunc, & olmadan tasima
    let sahipli = Some(String::from("veri"));
    match &sahipli {
        Some(s) => println!("odunc aldik: {}", s),
        None => println!("bos"),
    }
    println!("sahipli hala bizde: {:?}", sahipli);

    match sahipli {
        Some(s) => println!("bu sefer tasindi: {}", s),
        None => println!("bos"),
    }
    // println!("{:?}", sahipli);       // E0382 - tasindi

    // match bir IFADEDIR - tum kollarin tipi ayni olmali
    let sure = match Isik::Kirmizi {
        Isik::Kirmizi => 45,
        Isik::Sari => 4,
        Isik::Yesil => 30,
    };
    println!("sure = {}", sure);

    // Isik'a yeni bir varyant eklerseniz bu dosyadaki TUM match'ler derlenmez.
    // Derleyici size yapilacaklar listesi cikarir - _ yazsaydiniz cikarmazdi.
}

fn ortalama_yazdir(olcumler: &[f64]) {
    // ilk olcum yoksa devam etmenin anlami yok - erken cik
    let Some(ilk) = olcumler.first() else {
        println!("olcum yok");
        return;
    };
    // buradan sonra ilk duz bir &f64, girinti yok
    let mut toplam = 0.0;
    for o in olcumler {
        toplam += o;
    }
    println!("ilk={} ortalama={:.2}", ilk, toplam / olcumler.len() as f64);
}
