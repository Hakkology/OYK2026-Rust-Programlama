// Gun 8 / Ders 1 - Thread'ler ve Sahiplik
// rustc main.rs && ./main
//
// Dunya: gece servisi mutfagi. Sefler ayni anda calisiyor,
// siparisler birikiyor, kiler ortak.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("-- 1) ilk thread --");
    // spawn bir JoinHandle dondurur. join() beklemek demek.
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("    [sef] {}. tabak hazir", i);
            thread::sleep(Duration::from_millis(10));
        }
        "servis bitti"                       // thread bir DEGER dondurebilir
    });

    for i in 1..=2 {
        println!("  [salon] {}. masa siparis verdi", i);
        thread::sleep(Duration::from_millis(15));
    }

    let sonuc = handle.join().unwrap();       // join: bitmesini bekle + degeri al
    println!("  sef ne dedi: {}", sonuc);
    // join() cagirmasaydik main bitince thread yarida kesilebilirdi.

    println!("-- 2) move neden zorunlu --");
    let siparis = vec![String::from("mercimek corbasi"), String::from("kuru fasulye")];
    // thread::spawn(|| println!("{:?}", siparis));
    //   E0373: closure may outlive the current function, but it borrows `siparis`
    //   Derleyici thread'in ne kadar yasayacagini BILMIYOR. siparis main'de
    //   dusebilir, thread hala calisiyor olabilir -> sarkan referans.
    //   Gun 2'nin ownership kurallari burada odulunu veriyor:
    //   C'de bu kod derlenir ve rastgele coker; Rust'ta derlenmez.
    let h = thread::spawn(move || {
        println!("    [sef] siparis alindi: {:?}", siparis);
        siparis.len()
    });
    println!("  hazirlanan tabak sayisi: {}", h.join().unwrap());
    // println!("{:?}", siparis);
    //   E0382: siparis thread'e tasindi

    println!("-- 3) birden cok thread --");
    let mut handles = Vec::new();
    for id in 1..=3 {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * (4 - id) as u64));
            format!("{}. istasyon tamam", id)
        }));
    }
    // Sira GARANTI DEGIL; join sirasi bizim sirami̇z, calisma sirasi degil.
    for h in handles {
        println!("  {}", h.join().unwrap());
    }

    println!("-- 4) thread::scope: tasimadan odunc almak --");
    // move her seyi tasimak zorunda birakiyordu. scope bu kisiti kaldirir:
    // scope icindeki thread'ler scope BITMEDEN once biter.
    let stok: Vec<u32> = (1..=100).collect();
    let (sol, sag) = stok.split_at(50);
    let toplam = thread::scope(|s| {
        let a = s.spawn(|| sol.iter().sum::<u32>());     // odunc aliyor, tasimiyor
        let b = s.spawn(|| sag.iter().sum::<u32>());
        a.join().unwrap() + b.join().unwrap()
    });
    println!("  paralel toplam: {}", toplam);
    println!("  stok hala kullanilabilir: {} kalem", stok.len());   // tasinmadi

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
