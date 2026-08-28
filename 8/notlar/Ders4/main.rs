// Gun 8 / Ders 4 - async / await
// rustc main.rs && ./main
//
// Bu dosya HICBIR CRATE kullanmiyor: async/await dilin parcasi,
// RUNTIME degil. Runtime'i burada kendimiz yaziyoruz ki ne yaptigi gorunsun.
// Gercek projede tokio kullanacaksiniz - notlarda karsiliklari var.
//
// Mutfak benzetmesi: bir sef firindaki boregi beklerken elini bagli tutmaz,
// gidip salatayi hazirlar. Bloklamak yerine SIRAYA GIRMEK.

use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------
// 1) EN KUCUK RUNTIME
// ---------------------------------------------------------------
// Bir Future kendi kendine calismaz. Birinin poll() cagirmasi gerekir.
// Iste "runtime" dedigimiz sey tam olarak bu dongu:
fn block_on<F: Future>(future: F) -> F::Output {
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
struct Oven {
    dish: &'static str,
    ready_at: Instant,
}

impl Oven {
    fn bake(dish: &'static str, ms: u64) -> Oven {
        Oven { dish, ready_at: Instant::now() + Duration::from_millis(ms) }
    }
}

impl Future for Oven {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        if Instant::now() >= self.ready_at {
            Poll::Ready(format!("{} hazir", self.dish))
        } else {
            // "Henuz degil, beni tekrar yokla." Gercek runtime burada
            // zamanlayiciya kaydolur ve THREAD'I BLOKLAMAZ.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// ---------------------------------------------------------------
// 3) ASYNC FN - derleyici bunu bir Future'a cevirir
// ---------------------------------------------------------------
async fn prepare(dish: &'static str, ms: u64) -> String {
    let sonuc = Oven::bake(dish, ms).await;        // .await = "hazir olana kadar sirala"
    format!("[{}]", sonuc)
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

fn main() {
    println!("-- 1) Future TEMBELDIR --");
    let is = prepare("borek", 50);                 // hicbir sey calismadi
    println!("  async fn cagrildi, firin CALISMIYOR");
    println!("  ...simdi block_on ile calistiriyoruz");
    println!("  {}", block_on(is));
    // JavaScript'te Promise olusturunca is HEMEN baslar. Rust'ta baslamaz.
    // .await edilene kadar tek satir islemez.

    println!("-- 2) sirayla: her is bir oncekini bekliyor --");
    let t0 = Instant::now();
    let sirayla = block_on(async {
        let a = prepare("borek", 100).await;
        let b = prepare("salata", 100).await;
        let c = prepare("corba", 100).await;
        vec![a, b, c]
    });
    let sirayla_sure = t0.elapsed();
    println!("  {:?}", sirayla);
    println!("  sure: {:.0?}", sirayla_sure);

    println!("-- 3) join: ayni anda ilerliyor --");
    let t1 = Instant::now();
    let birlikte = block_on(JoinAll::new(vec![
        Box::pin(prepare("borek", 100)),
        Box::pin(prepare("salata", 100)),
        Box::pin(prepare("corba", 100)),
    ]));
    let birlikte_sure = t1.elapsed();
    println!("  {:?}", birlikte);
    println!("  sure: {:.0?}", birlikte_sure);
    println!("  AYNI sayida .await, ucte bir sure - hem de TEK THREAD'de.");
    println!("  Bu paralellik degil, ES ZAMANLILIK: beklerken baska is ilerliyor.");

    println!("-- 4) thread mi async mi --");
    println!("  CPU-bound (hesap)  -> thread / rayon");
    println!("  IO-bound (ag, disk)-> async");
    println!("  10.000 baglanti icin 10.000 thread: ~8 MB stack x 10.000 = 80 GB");
    println!("  10.000 task: birkac yuz bayt");
}
