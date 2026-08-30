# Gün 8 · Ders 3 — Kanallar

Ekibin **telsizi**. Sahadaki üyeler merkeze rapor geçiyor: konuşan çok, dinleyen tek.

> **Belleği paylaşarak iletişme; ileterek paylaş.**

Ders 2'de aynı veriye kilitle eriştik. Kanal başka bir yaklaşım: veri **tek** bir yerde
olur, thread'ler onu birbirine **gönderir**. Kilit yok, çünkü paylaşım yok.

## Temel kullanım

```rust
let (tx, rx) = mpsc::channel();
thread::spawn(move || { tx.send(String::from("catiya cikildi")).unwrap(); });
println!("{}", rx.recv().unwrap());
```

`mpsc` = **m**ulti **p**roducer, **s**ingle **c**onsumer: gönderen çok olabilir, alan tek.
`recv()` **bloklar** — mesaj gelene kadar bekler.

## Kanal sahiplik taşır

```rust
let kart = String::from("erisim karti #7");
tx.send(kart).unwrap();
println!("{}", kart);           // E0382: kanala taşındı
```

Gönderdiğiniz değer artık alıcınındır. Kanal bir **sahiplik borusudur** — bu yüzden
kanalla veri yarışı yapmak mümkün değil. Kilit gerekmemesinin sebebi bu.

## `send` ve `recv` birer `Result` döndürür

Kanalın iki ucu var; biri düşerse diğerinin işlemi başarısız olur. İkisi de `Result`
döndürüyor:

```rust
drop(rx);
match tx.send(7) {
    Ok(_)  => println!("gonderildi"),
    Err(e) => println!("alici dusmus - deger geri geldi: {}", e.0),
}
```

```
gonderilemedi, alici dusmus - deger geri geldi: 7
```

`SendError` içinde **gönderemediğiniz değer** duruyor (`e.0`) — kaybolmuyor, size iade
ediliyor. Sahiplik mantığının doğal sonucu: değeri alacak kimse yoksa geri sizin olur.

`recv()` de aynı şekilde: bütün göndericiler düştüyse `Err` döner. Zaten
`for gelen in rx` döngüsünün bitme sebebi budur.

### `try_recv` — beklemeden bakmak

`recv()` **bloklar**. Beklemeden bakmak isterseniz `try_recv()` var ve iki farklı
başarısızlığı ayırt eder:

```rust
match rx.try_recv() {
    Ok(v)                              => println!("{}", v),
    Err(TryRecvError::Empty)           => println!("su an bos ama kanal ACIK"),
    Err(TryRecvError::Disconnected)    => println!("kanal kapali"),
}
```

Fark önemli: **Empty** "sonra tekrar bak" demek, **Disconnected** "bir daha hiç gelmeyecek"
demek. Aynı `Err`'in içinde iki bambaşka karar — Gün 4'te enum'ların neden `bool`'dan
iyi olduğunu konuşmuştuk, örneği bu.

## Alıcı bir iterator'dur

```rust
for gelen in rx {
    println!(">> {}", gelen);
}
```

Döngü, kanal **kapanınca** kendiliğinden biter. Kanal ne zaman kapanır? Tüm göndericiler
düştüğünde.

## En sık takılınan yer: `drop(tx)`

```rust
for uye in 1..=3 {
    let tx = tx.clone();                 // her üye kendi telsizini alır
    thread::spawn(move || { tx.send(...).unwrap(); });
}
drop(tx);                                // ORİJİNALİ düşürmeyi UNUTMAYIN
for m in rx { ... }
```

`drop(tx)` satırı olmasaydı döngü **sonsuza kadar** beklerdi: klonlar düştü ama orijinal
`tx` hâlâ `main`'de yaşıyor, dolayısıyla kanal kapanmıyor. Program donar, hata vermez.

## İş havuzu

Tek kuyruk, üç kasacı. Alıcı tek olduğu için `Arc<Mutex<Receiver>>` ile paylaşılıyor:

```rust
let is = {
    let kuyruk = is_rx.lock().unwrap();
    kuyruk.recv()
};                                   // kilit BURADA bırakıldı
let kod = sifre_kir(kapi);           // hesap kilit DIŞINDA
```

**Kritik detay:** kilit yalnızca **iş almak** için tutulur. Hesabı kilidin içinde
yaparsanız paralellik kalmaz — Ders 2'de bunu ölçmüştük (49.7ms → 12.5ms).

```
9 kapi acildi
```

Dokuz kapının hangi kasacıya düştüğü **her çalıştırmada değişir** — dağılım işletim
sisteminin kararı, sizin değil. Programı iki kez çalıştırıp satırları karşılaştırın.
Garanti olan tek şey: **her iş tam bir kez** yapılır.

Kasacılar `recv()` `Err` dönünce döngüden çıkar — `drop(is_tx)` onların "iş bitti"
sinyalidir.

## `sync_channel` — geri basınç

`channel()` sınırsızdır: üretici alıcıdan hızlıysa kuyruk büyür, bellek şişer.
`sync_channel(n)` kapasiteyi sınırlar; kuyruk doluyken `send` **bloklar**:

```
[kasaci] 1. kasa bosaltildi
[kasaci] 2. kasa bosaltildi      <- kapasite doldu, kasaci bekliyor
[surucu] 1. canta araca kondu
[kasaci] 3. kasa bosaltildi
[surucu] 2. canta araca kondu
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
