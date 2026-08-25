// Gun 4 / Ders 4 - Pattern Matching
// rustc main.rs && ./main

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
enum Shape {
    Circle { r: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

// oyun/arayuz olaylari - varyantlar farkli sekilde
#[derive(Debug)]
enum Event {
    Key(char),
    Click { x: i32, y: i32 },
    Scroll(i32),
    Quit,
}

// satranc karesi
#[derive(Debug)]
struct Square {
    row: u8,
    col: u8,
}

fn main() {
    // EXHAUSTIVENESS - tum varyantlar ele alinmak zorunda
    for isik in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green] {
        let davranis = match isik {
            TrafficLight::Red => "dur",
            TrafficLight::Yellow => "hazirlan",
            TrafficLight::Green => "gec",
            // birini silin -> E0004 non-exhaustive patterns
        };
        print!("{:?}->{} ", isik, davranis);
    }
    println!();
    let isik = TrafficLight::Yellow;

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
        Square { row: 1, col: 4 },
        Square { row: 8, col: 8 },
        Square { row: 5, col: 3 },
    ];
    for k in &kareler {
        match k {
            Square { row: 1, col } => println!("beyaz taban, {}. sutun", col),
            Square { row: 8, col } => println!("siyah taban, {}. sutun", col),
            Square { row, .. } => println!("{}. sirada bir kare", row),
        }
    }

    // ENUM destructuring - varyantin verisini cikar
    let olaylar = vec![
        Event::Key('q'),
        Event::Click { x: 120, y: 45 },
        Event::Scroll(-3),
        Event::Quit,
    ];
    for o in &olaylar {
        match o {
            Event::Key(k) => println!("tusa basildi: {}", k),
            Event::Click { x, y } => println!("tiklama: ({}, {})", x, y),
            Event::Scroll(miktar) if *miktar < 0 => println!("asagi kaydirma: {}", miktar),
            Event::Scroll(miktar) => println!("yukari kaydirma: {}", miktar),
            Event::Quit => println!("cikis"),
        }
    }

    // match hem "hangisi" sorusunu cevaplar hem icindekini cikarir
    let sekiller = vec![
        Shape::Circle { r: 1.5 },
        Shape::Rectangle { width: 2.0, height: 5.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    for s in &sekiller {
        let area = match s {
            Shape::Circle { r } => 3.14159 * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        };
        println!("{:<32} alan={:.2}", format!("{:?}", s), area);
    }

    // if let - tek dalla ilgileniyorsak
    let bulunan: Option<char> = kelime.chars().next();
    if let Some(ilk) = bulunan {
        println!("ilk harf: {}", ilk);
    }

    // let else - eslesmezse ERKEN CIK, gerisi duz aksin
    let kayitlar = vec![21.5, 22.0, 19.8];
    print_average(&kayitlar);
    print_average(&[]);

    // while let - eslestigi surece don
    let mut yigin = vec![1, 2, 3];
    while let Some(ust) = yigin.pop() {
        print!("{} ", ust);
    }
    println!();

    // matches! - sadece "esliyor mu", bool doner
    println!("{} {}", matches!(isik, TrafficLight::Yellow), matches!(isik, TrafficLight::Green));

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
    let sure = match TrafficLight::Red {
        TrafficLight::Red => 45,
        TrafficLight::Yellow => 4,
        TrafficLight::Green => 30,
    };
    println!("sure = {}", sure);

    // TrafficLight'a yeni bir varyant eklerseniz bu dosyadaki TUM match'ler derlenmez.
    // Derleyici size yapilacaklar listesi cikarir - _ yazsaydiniz cikarmazdi.
}

fn print_average(olcumler: &[f64]) {
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
