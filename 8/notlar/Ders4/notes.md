# Gün 8 · Ders 4 — `async` / `await`

Hacker şifrenin çözülmesini beklerken elini bağlı tutmaz; o sırada kamerayı da kırmaya
başlar. Async'in tek cümlelik özeti bu: **bloklamak yerine sıraya girmek.**

> Gereken sürüm: **rustc 1.85+** (`Waker::noop` orada stabil oldu). `rustup update`.
>
> Bu dersin `main.rs`'i hiçbir crate kullanmıyor. Sebebi önemli: **`async`/`await` dilin
> parçası, runtime değil.** Runtime'ı burada kendimiz yazıyoruz ki ne yaptığı görünsün.
> Gerçek projede `tokio` kullanacaksınız; karşılıkları aşağıda.

## Neden var

10.000 eşzamanlı bağlantı için 10.000 thread açamazsınız: Rust'ın açtığı her thread
varsayılan **2 MiB** stack ayırır, 10.000 × 2 MiB = 20 GB. Bir async **task** birkaç yüz bayt.

| iş türü | araç |
|---|---|
| CPU-bound (hesap) | thread / `rayon` |
| IO-bound (ağ, disk, veritabanı) | async |

Ayrım şu: thread **hesap** için, async **bekleme** için. Beklerken thread tutmak israftır.

## Future tembeldir

```rust
let is = breach("kamera agi", 50);   // hiçbir şey çalışmadı
println!("async fn cagrildi, KIRMA BASLAMADI");
println!("{}", block_on(is));        // iş ancak şimdi başlıyor
```

`async fn` çağırmak **hiçbir şey çalıştırmaz**; size bir `Future` verir. `.await`
edilene ya da bir runtime'a verilene kadar tek satır işlemez.

> JavaScript'te `Promise` oluşturunca iş **hemen** başlar. Bu farkı bilerek gelin;
> sınıfın en çok şaşırdığı yer burası.

## Runtime dediğimiz şey

> Aşağıdaki kod `main.rs`'in ALTYAPI bölümünden. Fikri görün, ezberlemeyin.

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

## `async fn` aslında ne oluyor

Derleyici `async fn`'i bir **durum makinesine** çevirir: her `.await` bir duraklama
noktası, aradaki yerel değişkenler o makinenin alanları. Gün 7'de closure için
söylediğimizin aynısı — "adsız bir struct".

Bu makineyi kimin çalıştıracağı ayrı bir mesele; onu bir sonraki başlıkta gördük.

> `main.rs`'in altında **ALTYAPI** diye işaretli bir bölüm var: `block_on`, `Ice` ve
> `JoinAll` orada. Onlar runtime taklidi — gerçek projede `tokio` yapıyor.
> Okumanız gerekmiyor, dersin konusu değil.

### Oradaki `Pin` ne?

Altyapı koduna bakarsanız `poll`'un imzasında görürsünüz:

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String>
```

Tek cümlelik cevap: **durum makinesi kendi içine referans tutabilir**, o yüzden bellekte
taşınmaması gerekir. `Pin<&mut T>` bu sözün tip sistemindeki karşılığı: "bu değer artık
taşınmayacak."

`.await` yazarken `Pin` yazmazsınız; yalnızca elle `Future` implemente ederken çıkar.
Kampta buna girmiyoruz.

## Can alıcı ölçüm

```
sirayla : 300ms
join    : 100ms
```

Aynı sayıda `.await`, üçte bir süre, **tek thread'de**. Farkı yapan şey:

```rust
// sırayla: her .await bir öncekini bekliyor
let a = breach("kamera agi", 100).await;
let b = breach("kapi kilidi", 100).await;

// join: hepsi aynı anda ilerliyor
JoinAll::new(vec![Box::pin(breach("kamera agi", 100)), ...]).await
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
| `Ice` (zamanlayıcı) | `tokio::time::sleep` |
| — | `tokio::spawn` (task'ı runtime'a bırak) |

## Üç tuzak

1. **`std::thread::sleep` async içinde**: thread'i bloklar, runtime'daki *bütün* task'lar
   durur. → `tokio::time::sleep`
2. **`std::sync::Mutex` tutarken `.await`**: kilit tutulurken thread bloklanabilir,
   deadlock riski. → `tokio::sync::Mutex`
3. **Fonksiyon rengi**: `async fn`'i yalnızca async bağlamdan çağırabilirsiniz. Kod
   tabanı ikiye bölünür (`fn` dünyası / `async fn` dünyası). Async modelinin en çok
   eleştirilen yanı budur; dürüst olun, sınıf bunu zaten hissedecek.
