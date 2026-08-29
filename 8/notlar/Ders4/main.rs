// Gun 8 / Ders 4 - async / await
// rustc main.rs && ./main
//
// Bu dosya HICBIR CRATE kullanmiyor: async/await dilin parcasi,
// RUNTIME degil. Runtime'i burada kendimiz yaziyoruz ki ne yaptigi gorunsun.
// Gercek projede tokio kullanacaksiniz - notlarda karsiliklari var.
//
// Soygun benzetmesi: hacker sifre cozulmesini beklerken elini bagli tutmaz,
// o sirada kamerayi da kirmaya baslar. Bloklamak yerine SIRAYA GIRMEK.

// Bu dosyanin ALTINDA bir "altyapi" bolumu var: block_on ve JoinAll.
// Onlar runtime taklidi; bugunun konusu degil, okumaniz gerekmiyor.
// NOT: rustc 1.85+ gerekir.
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------
// ASYNC FN - derleyici bunu bir durum makinesine cevirir
// ---------------------------------------------------------------
async fn breach(layer: &'static str, ms: u64) -> String {
    let sonuc = Ice::crack(layer, ms).await;       // .await = "hazir olana kadar sirala"
    format!("[{}]", sonuc)
}

fn main() {
    println!("-- 1) Future TEMBELDIR --");
    let is = breach("kamera agi", 50);             // hicbir sey calismadi
    println!("  async fn cagrildi, KIRMA BASLAMADI");
    println!("  ...simdi block_on ile calistiriyoruz");
    println!("  {}", block_on(is));
    // JavaScript'te Promise olusturunca is HEMEN baslar. Rust'ta baslamaz.
    // .await edilene kadar tek satir islemez.

    println!("-- 2) sirayla: her is bir oncekini bekliyor --");
    let t0 = Instant::now();
    let sirayla = block_on(async {
        let a = breach("kamera agi", 100).await;
        let b = breach("kapi kilidi", 100).await;
        let c = breach("kasa paneli", 100).await;
        vec![a, b, c]
    });
    let sirayla_sure = t0.elapsed();
    println!("  {:?}", sirayla);
    println!("  sure: {:.0?}", sirayla_sure);

    println!("-- 3) join: ayni anda ilerliyor --");
    let t1 = Instant::now();
    let birlikte = block_on(JoinAll::new(vec![
        Box::pin(breach("kamera agi", 100)),
        Box::pin(breach("kapi kilidi", 100)),
        Box::pin(breach("kasa paneli", 100)),
    ]));
    let birlikte_sure = t1.elapsed();
    println!("  {:?}", birlikte);
    println!("  sure: {:.0?}", birlikte_sure);
    println!("  AYNI sayida .await, ucte bir sure - hem de TEK THREAD'de.");
    println!("  Bu paralellik degil, ES ZAMANLILIK: beklerken baska is ilerliyor.");

    println!("-- 4) thread mi async mi --");
    println!("  CPU-bound (hesap)  -> thread / rayon");
    println!("  IO-bound (ag, disk)-> async");
    println!("  10.000 baglanti icin 10.000 thread: 2 MiB stack x 10.000 = 20 GB");
    println!("  10.000 task: birkac yuz bayt");
}

// ===============================================================
// ALTYAPI - bugunun konusu DEGIL
// ===============================================================
use std::pin::{pin, Pin};

// Bir Future kendi kendine calismaz; birinin poll() cagirmasi gerekir.
// Gercek projede bunu tokio yapar. Burada crate indirmemek icin
// en kucuk halini kendimiz yaziyoruz. Okumadan gecebilirsiniz.
//
// Buradaki Pin: async fn'in urettigi durum makinesi KENDI ICINE referans
// tutabilir, o yuzden tasinmamasi gerekir. Pin bunun sozudur.
// .await yazarken Pin yazmazsiniz - sadece elle Future yazarken cikar.

// ---------------------------------------------------------------
// 1) EN KUCUK RUNTIME
// ---------------------------------------------------------------
// Bir Future kendi kendine calismaz. Birinin poll() cagirmasi gerekir.
// Iste "runtime" dedigimiz sey tam olarak bu dongu:
fn block_on<F: Future>(future: F) -> F::Output {
    // pin!: degeri stack'te SABITLER. Future kendi icine referans tutabilir,
    // tasinirsa o referans bozulur - Pin bunu engelleyen soz. (Notlarda detayi var.)
    let mut future = pin!(future);
    let waker = Waker::noop();                    // "hazir olunca haber ver" mekanizmasi
    let mut cx = Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(deger) => return deger,    // is bitti
            Poll::Pending => thread::yield_now(),  // hazir degil, sonra tekrar sor
        }
    }
}

// ---------------------------------------------------------------
// 2) ELLE YAZILMIS BIR FUTURE
// ---------------------------------------------------------------
// Bir sure bekleyen "IO" taklidi. Gercekte burasi soket/dosya olurdu.
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
            // "Henuz degil, beni tekrar yokla." Gercek runtime burada
            // zamanlayiciya kaydolur ve THREAD'I BLOKLAMAZ.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// ---------------------------------------------------------------
// 4) JOIN - ayni anda birden cok isi ilerletmek
// ---------------------------------------------------------------
// tokio::join! makrosunun yaptigi is: hepsini SIRAYLA poll et,
// hicbiri bitmediyse Pending don. Tek thread, es zamanli ilerleme.
struct JoinAll {
    isler: Vec<Pin<Box<dyn Future<Output = String>>>>,
    sonuclar: Vec<Option<String>>,
}

impl JoinAll {
    fn new(isler: Vec<Pin<Box<dyn Future<Output = String>>>>) -> JoinAll {
        let n = isler.len();
        JoinAll { isler, sonuclar: vec![None; n] }
    }
}

impl Future for JoinAll {
    type Output = Vec<String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Vec<String>> {
        let me = self.get_mut();
        let mut hepsi_bitti = true;
        for (i, is) in me.isler.iter_mut().enumerate() {
            if me.sonuclar[i].is_some() {
                continue;                          // bu is zaten bitmis
            }
            match is.as_mut().poll(cx) {
                Poll::Ready(v) => me.sonuclar[i] = Some(v),
                Poll::Pending => hepsi_bitti = false,
            }
        }
        if hepsi_bitti {
            Poll::Ready(me.sonuclar.iter().map(|s| s.clone().unwrap()).collect())
        } else {
            Poll::Pending
        }
    }
}
