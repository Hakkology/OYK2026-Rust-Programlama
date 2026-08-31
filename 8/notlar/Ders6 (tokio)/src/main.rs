// Gun 8 / Ders 6 (ek) - tokio ile async
// cargo run
//
// Ders 4'te runtime'i kendimiz yazmistik (block_on + JoinAll).
// Burada gercek hayatta kullanacaginiz haliyle: tokio.

use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

// IO taklidi: gercekte burasi ag istegi, veritabani sorgusu, dosya okuma olurdu.
async fn fetch(ad: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;   // BLOKLAMAZ
    format!("{} geldi", ad)
}

// async fn de Result dondurebilir; ? aynen calisir (Gun 5)
async fn parse_after(ad: &str, ms: u64) -> Result<u32, std::num::ParseIntError> {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    let n: u32 = ad.parse()?;
    Ok(n * 2)
}

// #[tokio::main] main'i bir runtime icinde calistirir.
// Actigi sey aslinda su: Runtime::new().block_on(async { ... })
#[tokio::main]
async fn main() {
    println!("== 1) tek bir await ==");
    println!("  {}", fetch("kullanici", 50).await);

    println!("== 2) sirayla vs join! ==");
    let t = Instant::now();
    let a = fetch("kullanici", 100).await;
    let b = fetch("siparisler", 100).await;
    let c = fetch("adresler", 100).await;
    println!("  sirayla : {:?} ({:.0?})", (a, b, c), t.elapsed());

    let t = Instant::now();
    // join! hepsini AYNI ANDA ilerletir - tek thread'de bile
    let (a, b, c) = tokio::join!(
        fetch("kullanici", 100),
        fetch("siparisler", 100),
        fetch("adresler", 100)
    );
    println!("  join!   : {:?} ({:.0?})", (a, b, c), t.elapsed());

    println!("== 3) tokio::spawn - gorev acmak ==");
    // spawn bir TASK acar (thread degil). Hemen calismaya baslar.
    let gorev = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(30)).await;
        "arka plan isi bitti"
    });
    println!("  main bu sirada baska is yapiyor...");
    println!("  {}", gorev.await.unwrap());        // JoinHandle -> Result

    println!("== 4) task ucuz, thread pahali ==");
    let t = Instant::now();
    let mut gorevler = Vec::new();
    for i in 0..1000 {
        gorevler.push(tokio::spawn(async move { i * 2 }));
    }
    let mut toplam: u64 = 0;
    for g in gorevler {
        toplam += g.await.unwrap();
    }
    println!("  1000 task: {:?} (toplam {})", t.elapsed(), toplam);

    let t = Instant::now();
    let mut handles = Vec::new();
    for i in 0..1000u64 {
        handles.push(std::thread::spawn(move || i * 2));
    }
    let mut toplam2: u64 = 0;
    for h in handles {
        toplam2 += h.join().unwrap();
    }
    println!("  1000 thread: {:?} (toplam {})", t.elapsed(), toplam2);

    println!("== 5) async kanal: tokio::sync::mpsc ==");
    let (tx, mut rx) = mpsc::channel::<String>(8);       // kapasiteli
    for id in 1..=3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10 * id)).await;
            tx.send(format!("{}. isci bitirdi", id)).await.unwrap();
        });
    }
    drop(tx);                                            // std'deki kural aynen gecerli
    while let Some(mesaj) = rx.recv().await {            // recv() da .await
        println!("  {}", mesaj);
    }

    println!("== 6) paylasilan durum: tokio::sync::Mutex ==");
    let sayac = Arc::new(Mutex::new(0u32));
    let mut gorevler = Vec::new();
    for _ in 0..4 {
        let sayac = Arc::clone(&sayac);
        gorevler.push(tokio::spawn(async move {
            for _ in 0..10 {
                let mut k = sayac.lock().await;          // lock() da .await
                *k += 1;
            }
        }));
    }
    for g in gorevler {
        g.await.unwrap();
    }
    println!("  sayac: {}", *sayac.lock().await);
    // NEDEN std::sync::Mutex degil: onun guard'i .await sinirini gecemez.
    // Kilit tutarken await ederseniz thread bloklanir, runtime tikanir.

    println!("== 7) async icinde ? ==");
    println!("  {:?}", parse_after("21", 10).await);
    println!("  {:?}", parse_after("yirmibir", 10).await.is_err());

    println!("== 8) ozet ==");
    println!("  std::thread::spawn -> tokio::spawn      (thread degil, task)");
    println!("  thread::sleep      -> tokio::time::sleep");
    println!("  std mpsc           -> tokio::sync::mpsc (send/recv .await)");
    println!("  std Mutex          -> tokio::sync::Mutex (lock .await)");
    println!("  handle.join()      -> handle.await");
}
