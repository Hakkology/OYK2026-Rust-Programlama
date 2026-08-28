# Gün 8 · Ders 2 — `Arc`, `Mutex` ve Paylaşılan Durum

Mutfakta ortak kiler var: dört şef aynı anda stok düşüyor. Gün 7'de öğrendiğimiz tablonun
**sağ sütunu** bugün doluyor.

| tek thread | çoklu thread |
|---|---|
| `Rc<T>` | `Arc<T>` |
| `RefCell<T>` | `Mutex<T>` / `RwLock<T>` |
| `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |

## Neden `Rc` yetmiyor

```rust
let sayac = Rc::new(0);
let kopya = Rc::clone(&sayac);
thread::spawn(move || println!("{}", kopya));
```

```
error[E0277]: `Rc<i32>` cannot be sent between threads safely
```

Sebep somut: **`Rc`'nin sayacı atomik değil.** İki thread aynı anda artırırsa sayaç
bozulur → veri erken düşer → use-after-free. Derleyici bunu baştan engelliyor.

`Arc` = **A**tomically **R**eference **C**ounted. Sayaç atomik olduğu için biraz daha
yavaştır; std ikisini ayrı tip tutar ki tek thread'de bedelini ödemeyesiniz.

## `Arc<Mutex<T>>`

```rust
let pantry = Arc::new(Mutex::new(Pantry { tomatoes: 100, served: 0 }));

for sef in 1..=4 {
    let ortak = Arc::clone(&pantry);        // sayaç artıyor, veri kopyalanmıyor
    thread::spawn(move || {
        let mut kiler = ortak.lock().unwrap();   // KİLİT alındı
        kiler.tomatoes -= 1;
    });                                          // guard düştü -> kilit bırakıldı
}
```

```
kalan domates: 60 | cikan tabak: 40
```

Dört şef × 10 tabak = 40, ve sayı **her çalıştırmada aynı**. `Mutex` olmasaydı bu bir
veri yarışı olurdu — ve Rust onu zaten derletmezdi.

İş bölümü net: **`Arc` paylaştırır, `Mutex` sıraya sokar.** İkisi ayrı sorun çözüyor.

## Kilit RAII ile bırakılıyor

**`unlock()` yoktur.** `lock()` size bir `MutexGuard` verir; guard düşünce kilit bırakılır.

```rust
{
    let mut kayit = log.lock().unwrap();
    kayit.push(...);
}                       // kapsam bitti -> kilit bırakıldı

let mut kayit = log.lock().unwrap();
drop(kayit);            // ya da açıkça düşürün
```

Gün 2'deki `Drop` mekanizmasının en zarif kullanımı. C'de `pthread_mutex_unlock` unutmak
klasik bir bug kaynağıdır; burada dilin kendisi engelliyor.

## Kilidi ne kadar tutmalı — ölçülmüş

Hesabı kilidin **içinde** yaparsanız paralellik kalmaz; herkes sırayla çalışır:

```
kilit icinde hesap     49.7ms  (sonuc 18000006000000)
kilit disinda hesap    12.5ms  (sonuc 18000006000000)
```

Aynı sonuç, dört kat fark. **Kural: kilit yalnızca paylaşılan veriye dokunduğunuz an
tutulmalı.** Hesabı önce yapın, kilidi sonra alın.

## `RwLock` — çok okuyucu, tek yazıcı

`Mutex` okuyucuları da sıraya sokar. Veri çoğunlukla okunuyorsa `RwLock` daha uygun:

```rust
let okunan = menu.read().unwrap();     // üçü de aynı anda okuyabilir
menu.write().unwrap().push(...);       // yazarken kimse okuyamaz
```

Bu, Gün 2'deki ödünç kuralının aynısı: ya çok okuyucu ya tek yazıcı. Fark, kuralın
**çalışma zamanında** uygulanması.

## Poisoning — kilit tutulurken panic

```rust
thread::spawn(move || {
    let _g = kopya.lock().unwrap();
    panic!("sef bicagi dusurdu");
}).join();
```

```
thread sonucu hata mi: true
kilit ZEHIRLENDI; degeri yine de alabiliriz: 10
```

Kilit tutulurken panic olursa `Mutex` **zehirlenir**; sonraki `lock()` `Err` döner.
Mantık: veri yarım kalmış olabilir, sessizce devam etmek tehlikelidir. Riski kabul
ediyorsanız `into_inner()` ile değeri yine de alabilirsiniz.

`lock().unwrap()` yazarken ne yaptığınızı bilin: orada `unwrap` ettiğiniz şey budur.

## Deadlock — dürüstlük anı

**Rust veri yarışını engeller, deadlock'u engellemez.** Derleyici tek satır uyarmaz:

```
thread 1: a.lock() sonra b.lock()
thread 2: b.lock() sonra a.lock()
```

İkisi de karşıdakinin bırakmasını bekler; program sonsuza kadar durur.

**Kural: kilitleri her zaman aynı sırada alın.** Ya da tek kilit kullanın. Bu bir dil
özelliği değil, sizin disiplininiz.

## `Send` ve `Sync`

İki **işaretçi trait** (marker trait — Gün 6). Elle implemente edilmez, derleyici verir.

| | ne demek |
|---|---|
| `Send` | bu tip başka bir thread'e **taşınabilir** |
| `Sync` | bu tipe `&T` ile birden çok thread'den **erişilebilir** |

Sağlamayanlar:

- `Rc<T>` → `Send` değil (sayaç atomik değil)
- `RefCell<T>` → `Sync` değil (çalışma zamanı kontrolü thread-safe değil)
- ham pointer → ikisi de değil

Bu yüzden Gün 7'de `Rc<RefCell<T>>`, bugün `Arc<Mutex<T>>` yazıyoruz. Aldığınız
`E0277` hataları bu iki trait'in eksikliğinden geliyor — hata mesajını artık
okuyabilirsiniz.
