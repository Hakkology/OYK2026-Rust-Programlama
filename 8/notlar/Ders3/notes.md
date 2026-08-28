# Gün 8 · Ders 3 — Kanallar

Mutfağın servis penceresi: siparişler salondan mutfağa akıyor, hazır tabaklar geri
dönüyor.

> **Belleği paylaşarak iletişme; ileterek paylaş.**

Ders 2'de aynı veriye kilitle eriştik. Kanal başka bir yaklaşım: veri **tek** bir yerde
olur, thread'ler onu birbirine **gönderir**. Kilit yok, çünkü paylaşım yok.

## Temel kullanım

```rust
let (tx, rx) = mpsc::channel();
thread::spawn(move || { tx.send(String::from("mercimek corbasi")).unwrap(); });
println!("{}", rx.recv().unwrap());
```

`mpsc` = **m**ulti **p**roducer, **s**ingle **c**onsumer: gönderen çok olabilir, alan tek.
`recv()` **bloklar** — mesaj gelene kadar bekler.

## Kanal sahiplik taşır

```rust
let siparis = String::from("kuru fasulye");
tx.send(siparis).unwrap();
println!("{}", siparis);        // E0382: kanala taşındı
```

Gönderdiğiniz değer artık alıcınındır. Kanal bir **sahiplik borusudur** — bu yüzden
kanalla veri yarışı yapmak mümkün değil. Kilit gerekmemesinin sebebi bu.

## Alıcı bir iterator'dur

```rust
for gelen in rx {
    println!("servis: {}", gelen);
}
```

Döngü, kanal **kapanınca** kendiliğinden biter. Kanal ne zaman kapanır? Tüm göndericiler
düştüğünde.

## En sık takılınan yer: `drop(tx)`

```rust
for istasyon in 1..=3 {
    let tx = tx.clone();                 // her thread kendi klonunu alır
    thread::spawn(move || { tx.send(...).unwrap(); });
}
drop(tx);                                // ORİJİNALİ düşürmeyi UNUTMAYIN
for m in rx { ... }
```

`drop(tx)` satırı olmasaydı döngü **sonsuza kadar** beklerdi: klonlar düştü ama orijinal
`tx` hâlâ `main`'de yaşıyor, dolayısıyla kanal kapanmıyor. Program donar, hata vermez.

Sınıfta o satırı yorum yapıp programın donduğunu gösterin — bir kez görülen bir hatadır.

## İş havuzu

Tek kuyruk, üç şef. Alıcı tek olduğu için `Arc<Mutex<Receiver>>` ile paylaşılıyor:

```rust
let is = {
    let kuyruk = is_rx.lock().unwrap();
    kuyruk.recv()
};                                   // kilit BURADA bırakıldı
let hazirlik = agir_hesap(masa);     // hesap kilit DIŞINDA
```

**Kritik detay:** kilit yalnızca **iş almak** için tutulur. Hesabı kilidin içinde
yaparsanız paralellik kalmaz — Ders 2'de bunu ölçmüştük (49.7ms → 12.5ms).

```
9 siparis dagitildi
```

Dokuz masanın hangi şefe düştüğü **her çalıştırmada değişir** — dağılım işletim
sisteminin kararı, sizin değil. Programı iki kez çalıştırıp satırları karşılaştırın.
Garanti olan tek şey: **her iş tam bir kez** yapılır.

Şefler `recv()` `Err` dönünce döngüden çıkar — yani `drop(is_tx)` onların "mesai bitti"
sinyalidir.

## `sync_channel` — geri basınç

`channel()` sınırsızdır: üretici alıcıdan hızlıysa kuyruk büyür, bellek şişer.
`sync_channel(n)` kapasiteyi sınırlar; kuyruk doluyken `send` **bloklar**:

```
[mutfak] 1. tabak pencereye kondu
[mutfak] 2. tabak pencereye kondu      <- kapasite doldu, mutfak bekliyor
[garson] 1. tabak alindi
[mutfak] 3. tabak pencereye kondu
[garson] 2. tabak alindi
```

Buna **backpressure** denir: yavaş tüketici, hızlı üreticiyi otomatik yavaşlatır.
`sync_channel(0)` ise randevu kanalıdır — gönderen, alan hazır olana kadar bekler.

## Kanal mı, kilit mi?

| | kanal | `Arc<Mutex<T>>` |
|---|---|---|
| veri | taşınır, tek sahip | paylaşılır |
| model | iş akışı, boru hattı | ortak durum |
| tıkanma | kanal kapanmazsa donar | deadlock riski |
| okuma | bir kez tüketilir | herkes okur |

Pratik kural: **iş akışı varsa kanal, ortak durum varsa kilit.** İkisi rakip değil;
iş havuzu örneğinde ikisini birlikte kullandık.
