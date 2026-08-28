# Gun 8 - Ders 4: Donanim, Cache ve Eszamanlilik

15:55-17:00 blogunun ilk 15 dakikasi - sunum, sonra kod

> `Donanim-Cache-Eszamanlilik.pptx` dosyasinin markdown aynasi. Sunum her uretildiginde
> bu dosya da yeniden yaziliyor; duzenlemeyi sunum kaynagindan yapin.

---

### 1 - Donanım, Cache ve Eşzamanlılık

*GÜN 8 · DERS 4 · 15 DK*

Thread yazmadan önce: altta gerçekte ne oluyor?

> **Konusmaci notu.** Gün 2'de bellek haritasını çizmiştik. Bugün o haritanın üstüne işlemciyi koyuyoruz — çünkü eşzamanlılığın zor olmasının sebebi donanımda.

---

### 2 - İşlemci hızlandı. Bellek aynı hızda hızlanmadı.

Aradaki uçurum kapanmadı, büyüdü.
Modern bir CPU zamanının çoğunu bekleyerek geçiriyor.

> **Konusmaci notu.** Buna 'memory wall' deniyor. 1980'lerde CPU ve RAM benzer hızdaydı; bugün arada iki kat büyüklük mertebesi fark var. Cache hiyerarşisi bu uçurumu kapatmak için icat edildi — bir çözüm değil, bir yama.

---

### 3 - Bellek hiyerarşisi

*Yukarı çıktıkça hızlı ve küçük, aşağı indikçe yavaş ve büyük*

| Katman | Gecikme | Boyut | Nerede |
|---|---|---|---|
| Register | ~0 | birkaç yüz byte | çekirdeğin içinde |
| L1 cache | ~1 ns | 32–64 KB | her çekirdeğe özel |
| L2 cache | ~4 ns | 256 KB – 1 MB | genelde çekirdeğe özel |
| L3 cache | ~15 ns | 8–32 MB | çekirdekler ARASINDA paylaşılır |
| RAM | ~80–100 ns | GB'lar | anakartta |
| NVMe SSD | ~50–100 µs | TB'lar | diskte |
| HDD | ~10 ms | TB'lar | diskte, dönen tabak |

> **Konusmaci notu.** Sayılar KABACA — işlemciye göre değişir, mertebe doğru. Vurgulanacak nokta: L1'den RAM'e inerken 100 kat, RAM'den SSD'ye inerken 1000 kat yavaşlıyorsunuz. Bu tablo tahtada dursun.

---

### 4 - L1'den bir veri okumak 1 saniye sürseydi...

L2: 4 saniye · L3: 15 saniye · RAM: 1,5 dakika
SSD: bir gün · Disk: dört ay

> **Konusmaci notu.** İnsan ölçeğine çevirmek sayıları hissedilir yapıyor. Söylenecek cümle: 'RAM'e gitmek, L1'e gitmenin yanında bir öğle molası vermek gibi.' Bu yüzden veriyi cache'te tutmak bir optimizasyon değil, performansın kendisi.

---

### 5 - Cache satırı: 64 byte

*Bir byte isteyin, 64 byte gelsin*

```
let x = dizi[0];    // 1 byte istediniz

  RAM'den gelen:  [ dizi[0] dizi[1] ... dizi[63] ]   64 byte
                  <------- tek bir cache satiri ------>

  dizi[1] artik BEDAVA - zaten L1'de
  dizi[64] ise yeni bir satir demek - yine ~100 ns

Donanim tek byte tasimaz. Hep satir tasir.
```

**Sonuç: komşu veriye erişmek neredeyse bedava, uzağa atlamak pahalı.**

> **Konusmaci notu.** Buna 'spatial locality' deniyor — mekânsal yerellik. Bir de 'temporal locality' var: az önce kullandığınız veri muhtemelen hâlâ cache'te. İki ilke de aynı şeyi söylüyor: veriyi bir arada tut, bir arada kullan.

---

### 6 - Aynı iş, on kat fark

*İki döngü, aynı sayıda toplama*

```
// (a) satir satir - HIZLI
for i in 0..n {
    for j in 0..n { toplam += m[i][j]; }   // komsu adresler
}

// (b) sutun sutun - YAVAS
for j in 0..n {
    for i in 0..n { toplam += m[i][j]; }   // her adimda n*4 byte atla
}

Ayni islem sayisi. Buyuk matriste (b) 5-10 kat yavas.
```

**Algoritma aynı, BigO aynı. Fark tamamen bellek erişim düzeninde.**

> **Konusmaci notu.** Gün 1'de 'sabitler önemsiz, büyüme hızı önemli' demiştiniz. Burada dürüst olun: BigO aynı olsa bile sabit 10 kat olabiliyor ve bu gerçek paradır. BigO yanlış değil, sadece hikâyenin tamamı değil. Vec<Vec<T>> yerine düz Vec<T> + indeks hesabı kullanmanın sebebi de bu.

---

### 7 - Şimdi çok çekirdek.

Her çekirdeğin KENDİ L1'i var.
Yani aynı verinin birden çok kopyası dolaşıyor.

> **Konusmaci notu.** Buradan sonrası eşzamanlılığın neden zor olduğunun cevabı. Tek çekirdekte 'bellek' tek bir gerçekti. Çok çekirdekte her çekirdeğin kendi görüşü var ve bunları tutarlı tutmak donanımın işi.

---

### 8 - Cache tutarlılığı ve false sharing

*Paylaşmadığınızı sandığınız şeyi paylaşmak*

```
struct Sayac { a: u64, b: u64 }   // 16 byte -> AYNI cache satirinda

  thread 1:  sayac.a += 1        (cekirdek 0)
  thread 2:  sayac.b += 1        (cekirdek 1)

Mantiken tamamen bagimsiz. Donanimda degil:
ayni 64 byte'lik satir iki cekirdek arasinda surekli gidip geliyor.

Tek thread'den bile YAVAS calisabilir.
```

**Çözüm: aralarına dolgu koyup ayrı cache satırlarına düşürmek.**

> **Konusmaci notu.** Buna 'false sharing' deniyor — sahte paylaşım. Kod doğru, sonuç doğru, performans felaket. Rust bunu ENGELLEMİYOR; Rust veri yarışını engelliyor, yavaşlığı değil. Dürüst olun: güvenli kod otomatik olarak hızlı kod demek değil. crossbeam'in CachePadded tipi tam olarak bu iş için var.

---

### 9 - Veri yarışı donanımda ne demek?

*x += 1 tek bir işlem değil*

```
sayac += 1   aslinda uc adim:

    1. oku      (load)
    2. arttir   (add)
    3. yaz      (store)

  cekirdek 0        cekirdek 1
  oku   -> 5
                    oku   -> 5
  arttir -> 6
                    arttir -> 6
  yaz   -> 6
                    yaz   -> 6      <-- iki artis, sonuc 6

Bir artis kayboldu. Her calistirmada olmaz - iste sorun bu.
```

**Tekrarlanamayan hata: testlerde geçer, üretimde patlar.**

> **Konusmaci notu.** Bu tabloyu tahtaya çizin. Anahtar cümle: hata HER ZAMAN olmuyor, zamanlamaya bağlı. Bu yüzden veri yarışları hata ayıklamanın en zor sınıfı — hata ayıklayıcı eklemek zamanlamayı değiştirip hatayı kaçırıyor ('heisenbug'). Çözüm test etmek değil, MÜMKÜN OLMAMASINI sağlamak.

---

### 10 - İki büyük yaklaşım

*İkisi de Rust'ta var — yarın ikisini de kullanacaksınız*

**Paylaşımlı bellek**

- Aynı veriye birden çok thread erişir
- Kilit (Mutex) ile sıraya sokulur
- Hızlı — kopyalama yok
- Riskler: kilitlenme, unutulan kilit, false sharing
- Rust: Arc<Mutex<T>>, RwLock

**Mesajlaşma**

- Veri paylaşılmaz, sahiplik devredilir
- Kanal üzerinden gönderilir
- Akıl yürütmesi çok daha kolay
- Bedeli: kopyalama ve kanal maliyeti
- Rust: mpsc kanalları

> **Konusmaci notu.** Go'nun sloganı: 'Belleği paylaşarak iletişim kurmayın, iletişim kurarak belleği paylaşın.' Rust ikisine de izin veriyor ama ownership sayesinde mesajlaşmada veriyi gönderdikten sonra ELİNİZDE KALMIYOR — derleyici bunu garanti ediyor. Yarın Gün 9'da ikisini de yazacaksınız.

---

### 11 - Rust'ın cevabı

*Veri yarışı bir çalışma zamanı sorunu değil, tip sorunu*

- **Ownership kuralı zaten yeterliydi** - Ya çok okuyucu ya tek yazıcı — bu kural tek thread'de de, çok thread'de de aynı
- **Send: bu tip başka bir thread'e taşınabilir mi?** - Derleyici otomatik çıkarıyor. Rc<T> Send değil, Arc<T> Send
- **Sync: bu tipe iki thread aynı anda referansla bakabilir mi?** - &T'nin Send olması demek. Cell ve RefCell Sync değil
- **Sonuç: veri yarışı içeren kod DERLENMİYOR** - Gün 2'de öğrendiğiniz kural, bugün bedava eşzamanlılık güvenliği veriyor

> **Konusmaci notu.** Haftanın en tatmin edici bağlantısı bu. Gün 2'de 'ya çok okuyucu ya tek yazıcı' kuralını bellek güvenliği için öğrendiler. Aynı kural, hiçbir ek şey yapmadan, veri yarışlarını da kapatıyor. Rust'ın 'fearless concurrency' dediği şey bu. AMA dürüst olun: kilitlenme (deadlock) hâlâ mümkün, false sharing hâlâ mümkün, mantık hatası hâlâ mümkün. Rust VERİ YARIŞINI kapatıyor, eşzamanlılığı kolaylaştırıyor ama çözmüyor.

---

### 12 - Şimdi thread yazalım

thread::spawn, move closure, thread::scope — ve neden move zorunlu
Yarın: Arc<Mutex>, kanallar, Send/Sync ve async

