// Gun 8 / Ders 3 - Kanallar
// rustc main.rs && ./main
//
// Ekibin TELSIZI. Sahadaki uyeler merkeze rapor geciyor.
// Kanal = telsiz frekansi: konusan cok, dinleyen tek.
//
// "Bellegi paylasarak iletisme; ileterek paylas."

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("-- 1) tek gonderici, tek alici --");
    // mpsc = multi producer, single consumer
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(String::from("catiya cikildi")).unwrap();
    });
    // recv() BLOKLAR: mesaj gelene kadar bekler.
    println!("  telsizden: {}", rx.recv().unwrap());

    println!("-- 2) kanal sahiplik TASIR --");
    let (tx, rx) = mpsc::channel();
    let kart = String::from("erisim karti #7");
    tx.send(kart).unwrap();
    // println!("{}", kart);
    //   E0382: kart kanala tasindi - artik alicinin.
    //   Kanal bir "sahiplik borusu": veri yarisi zaten mumkun degil.
    println!("  merkeze ulasan: {}", rx.recv().unwrap());
    drop(tx);

    println!("-- 2b) send ve recv birer Result dondurur --");
    // Alici dusmusse gondermek BASARISIZ olur - ve degerinizi geri verir.
    let (tx, rx) = mpsc::channel::<u8>();
    drop(rx);
    match tx.send(7) {
        Ok(_) => println!("  gonderildi"),
        Err(e) => println!("  gonderilemedi, alici dusmus - deger geri geldi: {}", e.0),
    }

    // Gonderici dusmusse almak da basarisiz olur.
    let (tx2, rx2) = mpsc::channel::<u8>();
    drop(tx2);
    match rx2.recv() {
        Ok(v) => println!("  gelen {}", v),
        Err(_) => println!("  kanal kapali, bir daha mesaj gelmeyecek"),
    }

    // try_recv BLOKLAMAZ: hemen doner, iki farkli Err ayirt edilir.
    let (tx3, rx3) = mpsc::channel::<u8>();
    match rx3.try_recv() {
        Ok(v) => println!("  {}", v),
        Err(mpsc::TryRecvError::Empty) => println!("  su an bos ama kanal ACIK - sonra tekrar bak"),
        Err(mpsc::TryRecvError::Disconnected) => println!("  kanal kapali"),
    }
    drop(tx3);

    println!("-- 3) alici bir ITERATOR'dur --");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for rapor in ["kat 12 temiz", "kamera kapali", "kasa gorundu", "cikis serbest"] {
            tx.send(rapor).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });   // tx burada dustu -> kanal KAPANDI
    // Dongu kanal kapaninca kendiliginden biter.
    for gelen in rx {
        println!("    >> {}", gelen);
    }

    println!("-- 4) cok gonderici (mpsc'nin 'mp'si) --");
    let (tx, rx) = mpsc::channel();
    for uye in 1..=3 {
        let tx = tx.clone();                     // her uye kendi telsizini alir
        thread::spawn(move || {
            tx.send(format!("{}. uye pozisyonda", uye)).unwrap();
        });
    }
    drop(tx);                                    // ORIJINALI dusurmeyi UNUTMAYIN
    //   Bu satir olmasaydi asagidaki dongu SONSUZA KADAR beklerdi:
    //   klonlar dustu ama orijinal tx hala yasiyor -> kanal kapanmiyor.
    //   Siniftaki en sik takilma noktasi budur.
    let mut mesajlar: Vec<String> = rx.iter().collect();
    mesajlar.sort();                             // varis sirasi garanti degil
    for m in &mesajlar {
        println!("    {}", m);
    }

    println!("-- 5) is havuzu --");
    // Kapi kuyrugu: tek alici, uc kasaci. Alici Mutex'le paylasiliyor.
    let (is_tx, is_rx) = mpsc::channel::<u32>();
    let is_rx = Arc::new(Mutex::new(is_rx));
    let (sonuc_tx, sonuc_rx) = mpsc::channel();

    for kasaci in 1..=3u32 {
        let is_rx = Arc::clone(&is_rx);
        let sonuc_tx = sonuc_tx.clone();
        thread::spawn(move || loop {
            // KRITIK: kilit SADECE is almak icin tutuluyor.
            let is = {
                let kuyruk = is_rx.lock().unwrap();
                kuyruk.recv()
            };                                   // kilit burada birakildi
            match is {
                Ok(kapi) => {
                    let kod = sifre_kir(kapi);          // hesap KILIT DISINDA
                    sonuc_tx.send((kasaci, kapi, kod)).unwrap();
                }
                Err(_) => break,                 // kanal kapandi, is bitti
            }
        });
    }
    drop(sonuc_tx);

    for kapi in 1..=9u32 {
        is_tx.send(kapi).unwrap();
    }
    drop(is_tx);                                 // kuyruk kapandi -> kasacilar cikacak

    let mut sonuclar: Vec<(u32, u32, u64)> = sonuc_rx.iter().collect();
    sonuclar.sort_by_key(|(_, kapi, _)| *kapi);
    for (kasaci, kapi, kod) in &sonuclar {
        println!("    kapi {} -> {}. kasaci (kod {})", kapi, kasaci, kod % 97);
    }
    println!("  {} kapi acildi", sonuclar.len());

    println!("-- 6) sync_channel: kuyruk dolunca gonderen bekler --");
    // Kapasite 2: uretici alicidan hizliysa geri basinc uygulanir.
    let (tx, rx) = mpsc::sync_channel(2);
    let uretici = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();                 // kuyruk doluysa BLOKLAR
            println!("    [kasaci] {}. kasa bosaltildi", i);
        }
    });
    thread::sleep(Duration::from_millis(20));    // surucu gec geliyor
    for canta in rx {
        println!("    [surucu] {}. canta araca kondu", canta);
        thread::sleep(Duration::from_millis(3));
    }
    uretici.join().unwrap();
}

fn sifre_kir(tohum: u32) -> u64 {
    let mut t = tohum as u64;
    for i in 1..=200_000u64 {
        t = t.wrapping_add(i);
    }
    t
}
