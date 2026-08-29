// Gun 8 / Ders 2 - Arc, Mutex ve Paylasilan Durum
// rustc main.rs && ./main
//
// Kuledeki kasa TEK. Dort ekip uyesi ayni anda kredi cekiyor.
// Gun 7'deki tablonun SAG sutunu bugun doluyor.

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

struct Vault {
    credits: u32,        // kasada kalan
    hauls: u32,          // yapilan cekim sayisi
}

fn main() {
    println!("-- 1) Rc thread'ler arasinda gecmiyor --");
    // use std::rc::Rc;
    // let sayac = Rc::new(0);
    // let kopya = Rc::clone(&sayac);
    // thread::spawn(move || println!("{}", kopya));
    //   E0277: `Rc<i32>` cannot be sent between threads safely
    //   Sebep: Rc'nin sayaci ATOMIK DEGIL. Iki thread ayni anda artirirsa
    //   sayac bozulur -> erken drop -> use-after-free. Derleyici bastan engelliyor.
    println!("  Rc: tek thread | Arc: cok thread (sayac atomik)");

    println!("-- 2) Arc<Mutex<T>>: paylasilan ve degistirilebilir --");
    let vault = Arc::new(Mutex::new(Vault { credits: 100, hauls: 0 }));
    let mut handles = Vec::new();
    for uye in 1..=4 {
        let ortak = Arc::clone(&vault);               // sayac artiyor, veri kopyalanmiyor
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                let mut kasa = ortak.lock().unwrap();    // KILIT alindi
                kasa.credits -= 1;
                kasa.hauls += 1;
            }                                             // MutexGuard dustu -> kilit birakildi
            uye
        }));
    }
    for h in handles {
        let uye = h.join().unwrap();
        println!("  {}. uye cekimini bitirdi", uye);
    }
    let son = vault.lock().unwrap();
    println!("  kasada kalan: {} kredi | toplam cekim: {}", son.credits, son.hauls);
    drop(son);                                        // kilidi erken birak

    println!("-- 3) kilit RAII ile birakiliyor --");
    // unlock() YOK. Unutmaniz mumkun degil - Gun 2'deki Drop'un en zarif kullanimi.
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    {
        let mut kayit = log.lock().unwrap();
        kayit.push(String::from("giris 02:14"));
    }   // kapsam bitti, kilit birakildi
    let mut kayit = log.lock().unwrap();
    kayit.push(String::from("cikis 02:31"));
    drop(kayit);                                      // ya da acikca drop
    println!("  kayit: {:?}", log.lock().unwrap());

    println!("-- 4) kilidi ne kadar tutmali --");
    // Hesabi kilidin ICINDE yaparsaniz paralellik kalmaz: herkes sirayla calisir.
    for (etiket, kilitte_hesapla) in [("kilit icinde kirma", true), ("kilit disinda kirma", false)] {
        let toplam = Arc::new(Mutex::new(0u64));
        let t0 = Instant::now();
        thread::scope(|s| {
            for _ in 0..4 {
                let toplam = Arc::clone(&toplam);
                s.spawn(move || {
                    if kilitte_hesapla {
                        let mut t = toplam.lock().unwrap();
                        *t += sifre_kir();             // KILIT TUTULURKEN hesap
                    } else {
                        let sonuc = sifre_kir();       // hesap once
                        let mut t = toplam.lock().unwrap();
                        *t += sonuc;                   // kilit sadece yazmak icin
                    }
                });
            }
        });
        println!("  {:<20} {:>8.1?}  (sonuc {})", etiket, t0.elapsed(), toplam.lock().unwrap());
    }

    println!("-- 5) RwLock: cok okuyucu, tek yazici --");
    let plan = Arc::new(RwLock::new(vec![String::from("catidan gir"), String::from("asansor sifti")]));
    thread::scope(|s| {
        for id in 1..=3 {
            let plan = Arc::clone(&plan);
            s.spawn(move || {
                let okunan = plan.read().unwrap();     // ucu de AYNI ANDA okuyabilir
                println!("    [uye {}] planda {} adim var", id, okunan.len());
            });
        }
    });
    plan.write().unwrap().push(String::from("kacis: garaj")); // yazarken kimse okuyamaz
    println!("  plan guncellendi: {:?}", plan.read().unwrap());

    println!("-- 6) poisoning: kilit tutulurken panic --");
    let alarm = Arc::new(Mutex::new(10u32));
    let kopya = Arc::clone(&alarm);
    let sonuc = thread::spawn(move || {
        let _g = kopya.lock().unwrap();
        panic!("uye yakalandi");                       // kilit TUTULURKEN panic
    })
    .join();
    println!("  thread sonucu hata mi: {}", sonuc.is_err());
    match alarm.lock() {
        Ok(_) => println!("  kilit temiz"),
        Err(zehirli) => {
            // Mantik: veri tutarsiz kalmis olabilir, sessizce devam etmek tehlikeli.
            println!("  kilit ZEHIRLENDI; degeri yine de alabiliriz: {}", zehirli.into_inner());
        }
    }

    println!("-- 7) Send / Sync --");
    println!("  Send : bu tip baska thread'e TASINABILIR");
    println!("  Sync : bu tipe &T ile birden cok thread'den ERISILEBILIR");
    println!("  Rc: Send degil (sayac atomik degil) | RefCell: Sync degil (kontrol thread-safe degil)");
}

fn sifre_kir() -> u64 {
    let mut t = 0u64;
    for i in 1..=3_000_000u64 {
        t = t.wrapping_add(i);
    }
    t
}
