# Gün 8 · Ders 6 (ek) — tokio ile async

> Bu bir **ek derstir**; günün beş dersi yerinde durur. Ders 4'te runtime'ı kendimiz
> yazmıştık (`block_on` + `JoinAll`). Burada aynı fikirlerin gerçek hayatta
> kullanacağınız hâli var.
>
> Çalıştırmak için: `cargo run` (bu klasörde). İnternet gerekir — tokio indirilecek.

## Kurulum

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

`full` her şeyi açar. Gerçek projede ihtiyacınız kadarını seçin:
`features = ["rt-multi-thread", "macros", "time"]`. Gün 9'da feature'ları konuşmuştuk —
tokio bunun en bilinen örneği.

## `#[tokio::main]`

```rust
#[tokio::main]
async fn main() { ... }
```

Bu makro `main`'i bir runtime içinde çalıştırır. Açtığı şey aslında şu:

```rust
fn main() {
    Runtime::new().unwrap().block_on(async { ... })
}
```

Ders 4'te elle yazdığımız `block_on`'un yerini alıyor. **`async fn main` tek başına
çalışmaz** — birinin `poll` etmesi gerekir, o birisi budur.

## `join!` — dersin can alıcı ölçümü

```rust
// sırayla
let a = fetch("kullanici", 100).await;
let b = fetch("siparisler", 100).await;
let c = fetch("adresler", 100).await;

// aynı anda
let (a, b, c) = tokio::join!(
    fetch("kullanici", 100),
    fetch("siparisler", 100),
    fetch("adresler", 100)
);
```

```
sirayla : (303ms)
join!   : (101ms)
```

Ders 4'teki `JoinAll`'ın yaptığı işin hazır hâli. Üç istek üçte bir sürede bitti ve
**paralellik yok** — beklerken başka iş ilerledi.

## `tokio::spawn` — görev açmak

```rust
let gorev = tokio::spawn(async {
    tokio::time::sleep(Duration::from_millis(30)).await;
    "arka plan isi bitti"
});
println!("main bu sirada baska is yapiyor...");
println!("{}", gorev.await.unwrap());
```

`std::thread::spawn`'a benziyor ama **thread açmıyor**: bir *task* açıyor ve runtime'ın
thread havuzuna bırakıyor. Dönen `JoinHandle`'ı `.await` edersiniz (`join()` değil), ve o
da `Result` döner — task panikleyebilir.

### Task ne kadar ucuz — ölçüldü

```
1000 task  :  1.4 ms
1000 thread: 20.1 ms
```

On dört kat fark. Ders 1'deki tablonun sebebi buydu: thread 2 MiB stack ayırıyor, task
birkaç yüz bayt. 10.000 bağlantılı bir sunucuyu thread'le yazamamanızın sebebi de bu.

## `tokio::sync::mpsc` — async kanal

```rust
let (tx, mut rx) = mpsc::channel::<String>(8);     // kapasiteli
tx.send(...).await.unwrap();                       // send de .await
while let Some(mesaj) = rx.recv().await { ... }    // recv de .await
```

Gün 8 Ders 3'teki kurallar **aynen geçerli**: göndericiler düşünce kanal kapanır,
`drop(tx)` yazmayı unutursanız döngü bitmez. Fark sadece `.await`.

## `tokio::sync::Mutex` — neden std'ninki değil

```rust
let sayac = Arc::new(Mutex::new(0u32));
let mut k = sayac.lock().await;      // lock() da .await
*k += 1;
```

```
sayac: 40
```

`std::sync::Mutex`'in guard'ı `.await` sınırını geçemez. Kilidi tutarken `.await`
ederseniz **thread bloklanır** ve o thread'deki bütün task'lar durur. Ders 4'teki "üç
tuzak"tan ikincisi buydu.

Pratik kural: **kilidi `.await` etmeden bırakabiliyorsanız `std::sync::Mutex` kullanın**
(daha hızlı). Kilidi tutarken `.await` etmeniz gerekiyorsa `tokio::sync::Mutex`.

## `async fn` ve `?`

```rust
async fn parse_after(ad: &str, ms: u64) -> Result<u32, ParseIntError> {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    let n: u32 = ad.parse()?;
    Ok(n * 2)
}
```

```
Ok(42)
true          <- "yirmibir" -> Err
```

`async fn` de `Result` döndürebilir; `?` aynen çalışır. Gün 5'te öğrendiğiniz her şey
burada da geçerli.

## Karşılık tablosu

| std (Gün 8, Ders 1-3) | tokio |
|---|---|
| `std::thread::spawn` | `tokio::spawn` (thread değil, **task**) |
| `thread::sleep` | `tokio::time::sleep(...).await` |
| `std::sync::mpsc` | `tokio::sync::mpsc` (`send`/`recv` `.await`) |
| `std::sync::Mutex` | `tokio::sync::Mutex` (`lock().await`) |
| `handle.join()` | `handle.await` |
| — | `#[tokio::main]`, `tokio::join!` |

## Ne zaman async, ne zaman thread

Ders 4'teki tablo geçerli:

| iş türü | araç |
|---|---|
| CPU-bound (hesap) | thread / `rayon` |
| IO-bound (ağ, disk, veritabanı) | async |

Async içinde **ağır hesap yapmayın** — task'ı yürüten thread'i bloklarsınız. Gerekirse
`tokio::task::spawn_blocking` ile ayrı bir havuza atarsınız.
