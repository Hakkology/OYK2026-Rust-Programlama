// Gun 8 / Ders 1 - Thread'ler ve Sahiplik
// rustc main.rs && ./main
//
// Dunya: 2087, Neo-Izmir. Bir soygun ekibi Ariva Kulesi'ne giriyor.
// Hacker guvenligi kiriyor, surucu motoru calistiriyor, kasaci kapiyi aciyor
// - hepsi AYNI ANDA.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("-- 1) ilk thread --");
    // spawn bir JoinHandle dondurur. join() beklemek demek.
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("    [hacker] {}. guvenlik katmani kirildi", i);
            thread::sleep(Duration::from_millis(10));
        }
        "sistem bizim"                       // thread bir DEGER dondurebilir
    });

    for i in 1..=2 {
        println!("  [merkez] {}. kamera devre disi", i);
        thread::sleep(Duration::from_millis(15));
    }

    let sonuc = handle.join().unwrap();       // join: bitmesini bekle + degeri al
    println!("  hacker rapor veriyor: {}", sonuc);
    // join() cagirmasaydik main bitince thread yarida kesilebilirdi.

    println!("-- 1b) join() bir Result dondurur --");
    // Thread panikleyebilir. join() bunu Err olarak bildirir; main HAYATTA kalir.
    let riskli = thread::spawn(|| {
        panic!("uye yakalandi");
    });
    match riskli.join() {
        Ok(_) => println!("  gorev tamam"),
        Err(_) => println!("  thread PANIKLEDI -> join Err dondu, main devam ediyor"),
    }
    // unwrap() yazsaydik main de panikleyecekti. Bu yuzden gercek kodda match.

    println!("-- 2) move neden zorunlu --");
    let ekipman = vec![String::from("EMP granati"), String::from("kart klonlayici")];
    // thread::spawn(|| println!("{:?}", ekipman));
    //   E0373: closure may outlive the current function, but it borrows `ekipman`
    //   Derleyici thread'in ne kadar yasayacagini BILMIYOR. ekipman main'de
    //   dusebilir, thread hala calisiyor olabilir -> sarkan referans.
    //   Gun 2'nin ownership kurallari burada odulunu veriyor:
    //   C'de bu kod derlenir ve rastgele coker; Rust'ta derlenmez.
    let h = thread::spawn(move || {
        println!("    [kasaci] canta bende: {:?}", ekipman);
        ekipman.len()
    });
    println!("  cantadaki parca sayisi: {}", h.join().unwrap());
    // println!("{:?}", ekipman);
    //   E0382: ekipman thread'e tasindi

    println!("-- 3) birden cok thread --");
    let mut handles = Vec::new();
    for id in 1..=3 {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * (4 - id) as u64));
            format!("{}. kat temiz", id)
        }));
    }
    // Sira GARANTI DEGIL; join sirasi bizim sirami̇z, calisma sirasi degil.
    for h in handles {
        println!("  {}", h.join().unwrap());
    }

    println!("-- 4) thread::scope: tasimadan odunc almak --");
    // move her seyi tasimak zorunda birakiyordu. scope bu kisiti kaldirir
    // (Rust 1.63+; oncesinde crossbeam crate'i gerekiyordu):
    // scope icindeki thread'ler scope BITMEDEN once biter.
    let kasa: Vec<u32> = (1..=100).collect();          // kasadaki kredi destesi
    let (sol, sag) = kasa.split_at(50);
    let toplam = thread::scope(|s| {
        let a = s.spawn(|| sol.iter().sum::<u32>());     // odunc aliyor, tasimiyor
        let b = s.spawn(|| sag.iter().sum::<u32>());
        a.join().unwrap() + b.join().unwrap()
    });
    println!("  paralel sayim: {} kredi", toplam);
    println!("  kasa hala elimizde: {} deste", kasa.len());   // tasinmadi

    println!("-- 5) paralellik ne zaman kazandirir --");
    // Thread ACMANIN sabit bir maliyeti var (~onlarca mikrosaniye).
    // Kazanip kazanmadigini IS MIKTARI belirler. Ikisini de olcelim.
    for boyut in [1_000usize, 2_000_000] {
        let veri: Vec<u64> = (1..=boyut as u64).collect();

        let t0 = Instant::now();
        let tek: u64 = veri.iter().sum();
        let tek_sure = t0.elapsed();

        let t1 = Instant::now();
        let (p1, p2) = veri.split_at(veri.len() / 2);
        let cift = thread::scope(|s| {
            let a = s.spawn(|| p1.iter().sum::<u64>());
            let b = s.spawn(|| p2.iter().sum::<u64>());
            a.join().unwrap() + b.join().unwrap()
        });
        let cift_sure = t1.elapsed();

        assert_eq!(tek, cift);
        let kazanc = if cift_sure < tek_sure { "iki thread kazandi" } else { "TEK thread kazandi" };
        println!("  {:>9} eleman | tek {:>10.1?} | cift {:>10.1?} | {}",
            boyut, tek_sure, cift_sure, kazanc);
    }
    println!("  Kucuk iste thread acmanin maliyeti isin kendisinden buyuk.");
    println!("  ONCE OLCUN, sonra paralellestirin. (rustc -O ile tekrar olcun!)");
}
