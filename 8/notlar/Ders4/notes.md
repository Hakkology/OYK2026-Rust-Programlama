# Gün 8 · Ders 4 — `async` / `await`

Bir şef fırındaki böreği beklerken elini bağlı tutmaz; gidip salatayı hazırlar.
Async'in tek cümlelik özeti bu: **bloklamak yerine sıraya girmek.**

> Bu dersin `main.rs`'i hiçbir crate kullanmıyor. Sebebi önemli: **`async`/`await` dilin
> parçası, runtime değil.** Runtime'ı burada kendimiz yazıyoruz ki ne yaptığı görünsün.
> Gerçek projede `tokio` kullanacaksınız; karşılıkları aşağıda.

## Neden var

10.000 eşzamanlı bağlantı için 10.000 thread açamazsınız: her thread'in ~8 MB stack'i
var, 10.000 × 8 MB = 80 GB. Bir async **task** birkaç yüz bayt.

| iş türü | araç |
|---|---|
| CPU-bound (hesap) | thread / `rayon` |
| IO-bound (ağ, disk, veritabanı) | async |

Ayrım şu: thread **hesap** için, async **bekleme** için. Beklerken thread tutmak israftır.

## Future tembeldir

```rust
let is = prepare("borek", 50);       // hiçbir şey çalışmadı
println!("async fn cagrildi, firin CALISMIYOR");
println!("{}", block_on(is));        // iş ancak şimdi başlıyor
```

`async fn` çağırmak **hiçbir şey çalıştırmaz**; size bir `Future` verir. `.await`
edilene ya da bir runtime'a verilene kadar tek satır işlemez.

> JavaScript'te `Promise` oluşturunca iş **hemen** başlar. Bu farkı bilerek gelin;
> sınıfın en çok şaşırdığı yer burası.

## Runtime dediğimiz şey

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(deger) => return deger,
            Poll::Pending => thread::yield_now(),
        }
    }
}
```

Hepsi bu: **`poll` çağıran bir döngü.** `Future` iki cevap verebilir:

- `Poll::Ready(v)` — iş bitti, değer burada
- `Poll::Pending` — henüz değil, hazır olunca `waker` ile haber veririm

### Runtime neden std'de yok

`poll` çağıracak birinin olması lazım ve bunun *nasıl* yapılacağı ortama göre değişir.
Rust runtime'ı dile koymadı ki gömülü sistemlerde de çalışsın. Go'nun runtime'ı dilin
içinde ve zorunludur — karşılaştırın: Rust size seçim bırakıyor, bedeli seçmek zorunda
olmanız.

## Elle yazılmış bir `Future`

```rust
impl Future for Oven {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        if Instant::now() >= self.ready_at {
            Poll::Ready(format!("{} hazir", self.dish))
        } else {
            cx.waker().wake_by_ref();     // "beni tekrar yokla"
            Poll::Pending
        }
    }
}
```

`async fn` yazdığınızda derleyici tam olarak böyle bir durum makinesi üretir: her
`.await` bir duraklama noktası, aradaki değişkenler o makinenin alanları. Gün 7'de
closure için söylediğimizin aynısı — "adsız bir struct".

## Can alıcı ölçüm

```
sirayla : 300ms
join    : 100ms
```

Aynı sayıda `.await`, üçte bir süre, **tek thread'de**. Farkı yapan şey:

```rust
// sırayla: her .await bir öncekini bekliyor
let a = prepare("borek", 100).await;
let b = prepare("salata", 100).await;

// join: hepsi aynı anda ilerliyor
JoinAll::new(vec![Box::pin(prepare("borek", 100)), ...]).await
```

`JoinAll`'un yaptığı iş `main.rs`'te açık: hepsini sırayla `poll` et, biri bitmişse
atla, hiçbiri bitmediyse `Pending` dön. `tokio::join!` makrosu da bunu yapar.

> **Bu paralellik değil, eşzamanlılık.** Tek thread var; kazanç, beklerken başka işin
> ilerlemesinden geliyor. Üç tane 100 ms'lik *hesap* olsaydı süre yine 300 ms olurdu.

## Gerçek hayatta: `tokio`

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]                          // main'i runtime içinde çalıştırır
async fn main() {
    tokio::time::sleep(Duration::from_millis(100)).await;

    let (a, b) = tokio::join!(fetch("a"), fetch("b"));   // bizim JoinAll

    let handle = tokio::spawn(async { ... });            // task aç (thread değil)
    handle.await.unwrap();
}
```

| bizim yazdığımız | tokio karşılığı |
|---|---|
| `block_on(fut)` | `#[tokio::main]` ya da `Runtime::block_on` |
| `JoinAll` | `tokio::join!` / `futures::future::join_all` |
| `Oven` (zamanlayıcı) | `tokio::time::sleep` |
| — | `tokio::spawn` (task'ı runtime'a bırak) |

## Üç tuzak

1. **`std::thread::sleep` async içinde**: thread'i bloklar, runtime'daki *bütün* task'lar
   durur. → `tokio::time::sleep`
2. **`std::sync::Mutex` tutarken `.await`**: kilit tutulurken thread bloklanabilir,
   deadlock riski. → `tokio::sync::Mutex`
3. **Fonksiyon rengi**: `async fn`'i yalnızca async bağlamdan çağırabilirsiniz. Kod
   tabanı ikiye bölünür (`fn` dünyası / `async fn` dünyası). Async modelinin en çok
   eleştirilen yanı budur; dürüst olun, sınıf bunu zaten hissedecek.
