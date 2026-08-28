// Gun 8 / Ders 3 - Kanallar
// rustc main.rs && ./main
//
// Mutfagin servis penceresi: siparisler salondan mutfaga akiyor,
// hazir tabaklar geri donuyor.
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
        tx.send(String::from("mercimek corbasi")).unwrap();
    });
    // recv() BLOKLAR: mesaj gelene kadar bekler.
    println!("  gelen: {}", rx.recv().unwrap());

    println!("-- 2) kanal sahiplik TASIR --");
    let (tx, rx) = mpsc::channel();
    let siparis = String::from("kuru fasulye");
    tx.send(siparis).unwrap();
    // println!("{}", siparis);
    //   E0382: siparis kanala tasindi - artik alicinin.
    //   Kanal bir "sahiplik borusu": veri yarisi zaten mumkun degil.
    println!("  gelen: {}", rx.recv().unwrap());
    drop(tx);

    println!("-- 3) alici bir ITERATOR'dur --");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for yemek in ["corba", "pilav", "tavuk", "tatli"] {
            tx.send(yemek).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });   // tx burada dustu -> kanal KAPANDI
    // Dongu kanal kapaninca kendiliginden biter.
    for gelen in rx {
        println!("    servis: {}", gelen);
    }

    println!("-- 4) cok gonderici (mpsc'nin 'mp'si) --");
    let (tx, rx) = mpsc::channel();
    for istasyon in 1..=3 {
        let tx = tx.clone();                     // her thread kendi klonunu alir
        thread::spawn(move || {
            tx.send(format!("{}. istasyon hazir", istasyon)).unwrap();
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
    // Siparis kuyrugu: tek alici, uc sef. Alici Mutex'le paylasiliyor.
    let (is_tx, is_rx) = mpsc::channel::<u32>();
    let is_rx = Arc::new(Mutex::new(is_rx));
    let (sonuc_tx, sonuc_rx) = mpsc::channel();

    for sef in 1..=3u32 {
        let is_rx = Arc::clone(&is_rx);
        let sonuc_tx = sonuc_tx.clone();
        thread::spawn(move || loop {
            // KRITIK: kilit SADECE is almak icin tutuluyor.
            let is = {
                let kuyruk = is_rx.lock().unwrap();
                kuyruk.recv()
            };                                   // kilit burada birakildi
            match is {
                Ok(masa) => {
                    let hazirlik = agir_hesap(masa);    // hesap KILIT DISINDA
                    sonuc_tx.send((sef, masa, hazirlik)).unwrap();
                }
                Err(_) => break,                 // kanal kapandi, is bitti
            }
        });
    }
    drop(sonuc_tx);

    for masa in 1..=9u32 {
        is_tx.send(masa).unwrap();
    }
    drop(is_tx);                                 // kuyruk kapandi -> sefler cikacak

    let mut sonuclar: Vec<(u32, u32, u64)> = sonuc_rx.iter().collect();
    sonuclar.sort_by_key(|(_, masa, _)| *masa);
    for (sef, masa, kod) in &sonuclar {
        println!("    masa {} -> {}. sef (kontrol {})", masa, sef, kod % 97);
    }
    println!("  {} siparis dagitildi", sonuclar.len());

    println!("-- 6) sync_channel: kuyruk dolunca gonderen bekler --");
    // Kapasite 2: uretici alicidan hizliysa geri basinc uygulanir.
    let (tx, rx) = mpsc::sync_channel(2);
    let uretici = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();                 // kuyruk doluysa BLOKLAR
            println!("    [mutfak] {}. tabak pencereye kondu", i);
        }
    });
    thread::sleep(Duration::from_millis(20));    // garson gec geliyor
    for tabak in rx {
        println!("    [garson] {}. tabak alindi", tabak);
        thread::sleep(Duration::from_millis(3));
    }
    uretici.join().unwrap();
}

fn agir_hesap(tohum: u32) -> u64 {
    let mut t = tohum as u64;
    for i in 1..=200_000u64 {
        t = t.wrapping_add(i);
    }
    t
}
