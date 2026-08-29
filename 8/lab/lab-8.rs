// Gun 8 / Lab - Ariva Kulesi Soygunu
// rustc lab-8.rs && ./lab-8
//
// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan uyarilari normal.
#![allow(unused)]
//
// SENARYO
// 2087, Neo-Izmir. Ekip Ariva Kulesi'nde. Uyeler ayni anda calisiyor,
// kasa ortak, telsiz surekli konusuyor, sifre cozulurken tezgah bos durmuyor.
//
// Bugunun bes dersi burada sirayla kullaniliyor:
//   LAB 1 -> thread, move, scope        (Ders 1)
//   LAB 2 -> Arc, Mutex, RwLock         (Ders 2)
//   LAB 3 -> kanallar, is havuzu        (Ders 3)
//   LAB 4 -> async/await                (Ders 4)
//   LAB 5 -> bagli liste                (Ders 5)

// NOT: Waker::noop() icin rustc 1.85+ gerekir.
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    lab_1_threadler();
    lab_2_paylasim();
    lab_3_kanallar();
    lab_4_async();
    lab_5_bagli_liste();
}

// ===========================================================================
// LAB 1 - Thread'ler
// ===========================================================================
fn lab_1_threadler() {
    println!("-- lab 1: threadler --");

    // ORNEK: bir thread ac, degerini al
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(5));
        "catiya cikildi"
    });
    println!("  {}", handle.join().unwrap());

    // TODO 1a: bir Vec<String> ekipman listesi olusturup thread'e `move` OLMADAN vermeyi deneyin.
    //          Hata kodu ne? Derleyici tam olarak neyi bilmiyor?

    // TODO 1b: `move` ile duzeltin. Sonra Vec'i thread'den SONRA kullanmayi deneyin.
    //          Bu sefer hangi hatayi aldiniz?

    // TODO 1c: 3 thread acin, her biri farkli sure uyusun (30/20/10 ms),
    //          hepsini bir Vec<JoinHandle<_>> icinde toplayip sirayla join edin.
    //          Cikti sirasi uyuma suresine gore mi, join sirasina gore mi?

    // TODO 1d: thread::scope ile bir Vec<u32>'yi split_at ile ikiye bolun,
    //          iki thread'de toplayip birlestirin. Vec scope'tan SONRA hala
    //          kullanilabiliyor mu? Neden?

    // TODO 1e (olcum): ayni toplamayi 1_000 ve 2_000_000 elemanla,
    //          tek thread ve iki thread olarak olcun (Instant::now()).
    //          Hangi boyutta hangisi kazandi? Sonucu bir cumleyle yazin.
    //          Sonra `rustc -O lab-8.rs` ile derleyip TEKRAR olcun.
}

// ===========================================================================
// LAB 2 - Paylasilan durum
// ===========================================================================
struct Vault {
    credits: u32,
    hauls: u32,
}

fn lab_2_paylasim() {
    println!("-- lab 2: paylasim --");

    // TODO 2a: Rc<RefCell<Vault>> olusturup bir thread'e vermeyi DENEYIN.
    //          Hata kodu ne? Hangi trait eksik? Neden Rc bu trait'i saglamiyor?

    // TODO 2b: Arc<Mutex<Vault>> ile 4 uye acin, her biri 10 kredi ceksin.
    //          Kasada kalan kac? Her calistirmada AYNI mi?

    // TODO 2c: kilidi ne kadar tuttugunuz onemli. Ayni isi iki turlu yazin:
    //            (1) agir hesabi kilidin ICINDE yapin
    //            (2) agir hesabi kilit DISINDA yapip sadece yazarken kilitleyin
    //          Ikisini Instant ile olcup farki yazdirin.
    //          Agir hesap icin asagidaki sifre_kir fonksiyonunu kullanabilirsiniz.

    // TODO 2d: bir Arc<RwLock<Vec<String>>> kacis plani olusturun.
    //          3 thread AYNI ANDA okusun (read), sonra ana thread yeni adim yazsin (write).
    //          Mutex ile RwLock farkini bir cumleyle yazin.

    // TODO 2e: bir thread kilidi TUTARKEN panic etsin.
    //          Sonra ana thread lock() cagirsin. Ne dondu?
    //          `into_inner()` ne ise yariyor?

    // TODO 2f: deadlock'u KURMAYIN ama kagida yazin:
    //          iki kilit, iki thread, ters sira. Neden donuyor?
    //          Rust bunu neden engellemiyor? Kural ne?
}

fn sifre_kir() -> u64 {
    let mut t = 0u64;
    for i in 1..=3_000_000u64 {
        t = t.wrapping_add(i);
    }
    t
}

// ===========================================================================
// LAB 3 - Kanallar
// ===========================================================================
fn lab_3_kanallar() {
    println!("-- lab 3: kanallar --");

    // ORNEK: tek gonderici
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send("kat 12 temiz").unwrap());
    println!("  {}", rx.recv().unwrap());

    // TODO 3a: bir String'i send edin, sonra ayni String'i yazdirmayi deneyin.
    //          Hata kodu ne? Kanal sahiplik acisindan ne yapiyor?

    // TODO 3b: bir thread 4 telsiz raporu gondersin, ana thread `for gelen in rx` ile alsin.
    //          Dongu neden kendiliginden bitiyor?

    // TODO 3c: 3 thread'e tx.clone() dagitin. `drop(tx)` satirini YAZMADAN
    //          calistirin. Program ne yapti? Simdi drop(tx) ekleyin.
    //          (Donarsa Ctrl+C ile durdurun - bu bir hata degil, tasarim.)

    // TODO 3d (is havuzu): bir is kanali (u32 kapi numaralari) ve bir sonuc kanali kurun.
    //          Alici Arc<Mutex<Receiver>> ile 3 kasaciya paylastirilsin.
    //          KRITIK: kilit sadece isi ALMAK icin tutulsun, sifre kirma kilit disinda olsun.
    //          9 kapi gonderip sonuclari toplayin. Her is tam bir kez mi yapildi?

    // TODO 3e: mpsc::sync_channel(2) ile ayni ureticiyi kurun, alici 3 ms uyusun.
    //          Cikti sirasina bakin: uretici ne zaman bekliyor? Bu neden istenir?
}

// ===========================================================================
// LAB 4 - async / await
// ===========================================================================
// ORNEK (verildi): en kucuk runtime. Bir Future kendi kendine calismaz.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => thread::yield_now(),
        }
    }
}

// ORNEK (verildi): IO taklidi eden bir Future
struct Ice {
    layer: &'static str,
    ready_at: Instant,
}

impl Ice {
    fn crack(layer: &'static str, ms: u64) -> Ice {
        Ice { layer, ready_at: Instant::now() + Duration::from_millis(ms) }
    }
}

impl Future for Ice {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        if Instant::now() >= self.ready_at {
            Poll::Ready(format!("{} kirildi", self.layer))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// TODO 4c: iki Future'i AYNI ANDA ilerleten bir Join2 yazin.
//          struct Join2<A, B> { a: Pin<Box<A>>, b: Pin<Box<B>>, ... }
//          poll icinde: bitmemis olanlari poll et, ikisi de bittiyse Ready don.
//          Ipucu: sonuclari Option<String> alanlarinda saklayin.

fn lab_4_async() {
    println!("-- lab 4: async --");

    // TODO 4a: `async fn kir(katman: &'static str, ms: u64) -> String` yazin;
    //          icinde Ice::crack(...).await kullansin.
    //          Once SADECE cagirin, block_on ETMEYIN. Ekrana bir sey yazildi mi?
    //          Future'in tembelligini bir cumleyle aciklayin.

    // TODO 4b: uc isi SIRAYLA await edip toplam sureyi olcun (her biri 100 ms).
    //          Beklediginiz sure ne, olctugunuz ne?

    // TODO 4d: 4c'deki Join2 ile iki isi birlikte calistirip sureyi olcun.
    //          Sirali sureye gore ne degisti? Kac thread kullandiniz?
    //          "Paralellik" mi "es zamanlilik" mi oldu, farki yazin.

    // TODO 4e: async icinde thread::sleep kullanmak neden yanlis?
    //          (Calistirmayin, sadece cevaplayin: runtime'a ne olur?)
}

// ===========================================================================
// LAB 5 - Bagli liste
// ===========================================================================
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct Stash<T> {
    head: Link<T>,
    len: usize,
}

// TODO 5a: impl<T> Stash<T> yazin:
//            fn new() -> Stash<T>
//            fn push(&mut self, elem: T)      -> yeni dugum EN USTE
//            fn pop(&mut self) -> Option<T>
//            fn len(&self) / fn is_empty(&self)
//          push icinde `let eski = self.head;` yazmayi deneyin: hata kodu ne?
//          take() bu sorunu nasil cozuyor, bir cumleyle yazin.

// TODO 5b: fn peek(&self) -> Option<&T> yazin.
//          as_ref() olmadan yazmayi deneyin - hangi hata?

// TODO 5c: fn peek_mut(&mut self) -> Option<&mut T> ekleyip bastaki fisi degistirin.

// TODO 5d: odunc alan bir iterator yazin:
//            struct Iter<'a, T> { simdiki: Option<&'a Node<T>> }
//            impl<'a, T> Iterator for Iter<'a, T> { type Item = &'a T; ... }
//          Sonra listede sum() / max() / filter() calistirin (Gun 7 kombinatorleri).
//          Ipucu: as_deref() ile Option<Box<Node<T>>> -> Option<&Node<T>>

// TODO 5e: Stash'in KENDISI icin Iterator yazin (next = pop).
//          Bu surum listeyi neden tuketiyor?

// TODO 5f: 200_000 dugumlu bir liste olusturup drop edin.
//          Ne oldu? Simdi iteratif bir Drop yazip tekrar deneyin.
//          Yigin tasmasi bir bellek guvenligi ihlali mi? Neden?

fn lab_5_bagli_liste() {
    println!("-- lab 5: bagli liste --");
    // TODO: 5a-5f bitince buradan deneyin
}
