# Gün 8 · Ders 2 — `Arc`, `Mutex` ve Paylaşılan Durum

Kuledeki kasa **tek**: dört ekip üyesi aynı anda kredi çekiyor. Gün 7'de öğrendiğimiz
tablonun **sağ sütunu** bugün doluyor.

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
let vault = Arc::new(Mutex::new(Vault { credits: 100, hauls: 0 }));

for uye in 1..=4 {
    let ortak = Arc::clone(&vault);         // sayaç artıyor, veri kopyalanmıyor
    thread::spawn(move || {
        let mut kasa = ortak.lock().unwrap();    // KİLİT alındı
        kasa.credits -= 1;
    });                                          // guard düştü -> kilit bırakıldı
}
```

```
kasada kalan: 60 kredi | toplam cekim: 40
```

Dört üye × 10 çekim = 40, ve sayı **her çalıştırmada aynı**. `Mutex` olmasaydı bu bir
veri yarışı olurdu — ve Rust onu zaten derletmezdi.

İş bölümü net: **`Arc` paylaştırır, `Mutex` sıraya sokar.** İkisi ayrı sorun çözüyor.

## Kilit RAII ile bırakılıyor

**`unlock()` yoktur.** `lock()` size bir `MutexGuard` verir; guard düşünce kilit bırakılır.

```rust
{
    let mut kayit = log.lock().unwrap();
    kayit.push(String::from("giris 02:14"));
}                       // kapsam bitti -> kilit bırakıldı

let mut kayit = log.lock().unwrap();
drop(kayit);            // ya da açıkça düşürün
```

Gün 2'deki `Drop` mekanizmasının en zarif kullanımı. C'de `pthread_mutex_unlock` unutmak
klasik bir bug kaynağıdır; burada dilin kendisi engelliyor.

## Kilidi ne kadar tutmalı — ölçülmüş

Hesabı kilidin **içinde** yaparsanız paralellik kalmaz; herkes sırayla çalışır:

```
kilit icinde kirma    147.2ms  (sonuc 18000006000000)
kilit disinda kirma    36.4ms  (sonuc 18000006000000)
```

Aynı sonuç, dört kat fark. **Kural: kilit yalnızca paylaşılan veriye dokunduğunuz an
tutulmalı.** Hesabı önce yapın, kilidi sonra alın.

## `RwLock` — çok okuyucu, tek yazıcı

`Mutex` okuyucuları da sıraya sokar. Veri çoğunlukla okunuyorsa `RwLock` daha uygun:

```rust
let okunan = plan.read().unwrap();     // üçü de aynı anda okuyabilir
plan.write().unwrap().push(...);       // yazarken kimse okuyamaz
```

Bu, Gün 2'deki ödünç kuralının aynısı: ya çok okuyucu ya tek yazıcı. Fark, kuralın
**çalışma zamanında** uygulanması.

## Poisoning — kilit tutulurken panic

```rust
thread::spawn(move || {
    let _g = kopya.lock().unwrap();
    panic!("uye yakalandi");
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
