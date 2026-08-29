# Gun 8 - Ders 1: Eszamanlilik, Thread ve Ownership

Ders 1 blogunun ilk 20 dakikasi - sunum, sonra kod

> `Eszamanlilik-Thread-Ownership.pptx` dosyasinin markdown aynasi.
> Duzenlemeyi sunum kaynagindan yapin: `OYK2026-plan/slides/uret_gun8.py`

---

### 1 - GÜN 8 · DERS 1 · 20 DK

Eşzamanlılık, Thread ve Ownership

Bilgisayarınızda 431 program çalışıyor. Çekirdek sayısı 16.

> **Konusmaci notu.** Baslarken bu celiskiyi sorun: 431 program, 16 cekirdek. Nasil oluyor? Cevap bu 20 dakikanin tamami.

---

### 2 - Şu anda bu makinede

ps ile sayıldı

```
$ nproc                        ->    16   cekirdek
$ ps -e --no-headers | wc -l   ->   431   surec (process)
$ ps -eLf --no-headers | wc -l ->  1870   thread

16 cekirdek, 1870 yurutme akisi.
Ayni anda en fazla 16 tanesi GERCEKTEN calisabilir.
Peki digerleri?
```

Cevap: hiçbiri sürekli çalışmıyor. İşletim sistemi hepsini sırayla, çok hızlı değiştiriyor.

> **Konusmaci notu.** Sinifin kendi makinesinde denemesini isteyin, sayilar benzer cikar. Bu slayt gunun butun sorusunu ortaya koyuyor: paylasilan az sayida cekirdek, cok sayida is.

---

### 3 - İki kelime, iki ayrı soru

gün boyu bu ayrımı koruyacağız

EŞZAMANLILIK (concurrency)

PARALELLİK (parallelism)

Birden çok işi AYNI DÖNEMDE

yürütmek.

İşler iç içe geçer, sırayla

ilerler, araya girilir.

TEK çekirdekte de olur.

Soru: işleri nasıl

DÜZENLERİM?

Birden çok işi AYNI ANDA

fiziksel olarak yürütmek.

Çok çekirdek ŞART.

Bu makinede en fazla 16.

Eşzamanlılığın bir

gerçekleşme biçimidir.

Soru: nasıl HIZLANDIRIRIM?

> **Konusmaci notu.** Klasik cumle: eszamanlilik bir YAPI meselesi, paralellik bir YURUTME meselesi. Tek cekirdekli bir telefon bile eszamanli calisir: muzik calarken ekrani ciziyor. Paralellik ise donanim ister. Her paralel program eszamanlidir; her eszamanli program paralel degildir.

---

### 4 - İşletim sistemi 431 süreci nasıl yönetiyor?

zaman dilimi + bağlam değiştirme

```
cekirdek 0'in zaman cizgisi:

  |--tarayici--|--muzik--|--derleyici--|--tarayici--|--terminal--|
       ~5 ms      ~5 ms       ~5 ms         ~5 ms        ~5 ms

Her degisimde ZAMANLAYICI (scheduler) devreye giriyor:
  1. calisan surecin register'larini kaydet
  2. siradakinin register'larini yukle
  3. bellek haritasini (MMU) degistir

Buna BAGLAM DEGISTIRME (context switch) deniyor.
```

Bu makinede iki thread arasında gidiş-dönüş ~14 µs ölçüldü. Bedava değil — ama 5 ms'lik dilimin yanında küçük.

> **Konusmaci notu.** Anlatirken zaman cizgisini tahtaya cizin. Zaman dilimi (time slice) tipik olarak birkac milisaniye. Insan gozu bunu fark etmez, o yuzden her sey ayni anda calisiyor SANIRIZ. Baglam degistirme maliyeti bos degil: register kaydet/yukle, cache soguyor, MMU tablolari degisiyor. Surecler arasi gecis, thread'ler arasi gecisten PAHALI - cunku bellek haritasi da degisiyor.

---

### 5 - Thread'i aslında kim açıyor?

strace ile yakalandı — Rust'ın açtığı thread

```
$ strace -f -e trace=clone3 ./program

clone3({flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND
               |CLONE_THREAD|CLONE_SETTLS|...,
         stack_size=0x1fff40})           <- 2 MiB

CLONE_VM     : ayni bellek alanini paylas  (heap ORTAK)
CLONE_FILES  : ayni acik dosyalari paylas
CLONE_THREAD : ayni surecin parcasi ol
```

Linux'ta süreç de thread de aynı çekirdek yapısıdır (task). Fark: hangi bayrakları paylaştıkları.

> **Konusmaci notu.** Bu ciktiyi ben aldim, uydurma degil - sinifta canli da gosterebilirsiniz. Ogretici olan: thread ozel bir sey degil, sadece BELLEGI PAYLASAN bir surec. Linux'ta ikisi de task_struct. fork() bu bayraklari vermez, o yuzden ayri bellek alani cikar. stack_size satirina dikkat cekin: 2 MiB rakami burada, sistem cagrisinin icinde goruluyor.

---

### 6 - Thread'in üç hâli

zamanlayıcı bunlar arasında gezdiriyor

ÇALIŞIYOR (running) — şu an bir çekirdekte. En fazla 16 tane olabilir.

HAZIR (ready) — işi var, sırasını bekliyor. Çekirdek boşalınca girecek.

BLOKE (blocked) — I/O bekliyor: ağ, disk, kilit. Çekirdeği BIRAKTI.

Bloke thread çekirdek harcamaz — ama 2 MiB stack'ini tutmaya devam eder.

10.000 bağlantı = 10.000 bloke thread = 20 GB. İşte async'in sebebi (Ders 4).

> **Konusmaci notu.** Uc hali tahtaya cizin, oklarla baglayin. Anahtar nokta: bloke thread cekirdek harcamiyor, isletim sistemi yerine baskasini koyuyor - bu yuzden 1870 thread 16 cekirdekte rahatca duruyor, cogu bloke. AMA bellek harciyor. Ders 4'te async'in derdi tam olarak bu: beklemek icin thread harcamamak.

---

### 7 - Süreç ve thread farkı

ikisi de yürütme akışı — fark neyi paylaştıkları

```
  SUREC (process)                                    
  +--------------------------------------------+     
  |  KOD        (ortak)                        |     
  |  STATIC     (ortak)                        |     
  |  HEAP       (ortak)  <- Box, Vec, String   |     
  |                                            |     
  |  +----------------+   +----------------+   |     
  |  | thread 1 STACK |   | thread 2 STACK |   |     
  |  | + register'lar |   | + register'lar |   |     
  |  +----------------+   +----------------+   |     
  +--------------------------------------------+
```

İki SÜREÇ hiçbir şey paylaşmaz. Aynı sürecin iki THREAD'i heap'i paylaşır — bütün mesele burada.

> **Konusmaci notu.** Kritik cumle: thread'ler heap'i paylasir. Iki thread ayni Vec'e dokunabiliyor cunku Vec'in verisi heap'te. Stack'ler ayri, o yuzden yerel degiskenler cakismaz. Surecler arasinda paylasim yok - bu yuzden guvenli ama iletisim pahali (IPC gerekir). Thread ucuz iletisim verir, bedeli: veri yarisi riski. Gun 2'deki bellek haritasinin ustune bunu koyun.

---

### 8 - Thread bedava değil

bu makinede ölçüldü

Ne

Değer

Ana thread stack (Linux)

8 MB

Rust'ta açılan thread stack

2 MiB (varsayılan)

Bir thread açıp beklemek

~82 µs

İki thread arası gidiş-dönüş

~14 µs

Bu makinedeki çekirdek

16

> **Konusmaci notu.** 82 mikrosaniye kucuk gorunuyor ama 1000 kucuk is icin 82 ms eder; isin kendisi 1 ms ise zarardasiniz. Ders 1'de bunu olcerek gosterecegiz. Ayrica 10.000 baglanti icin 10.000 thread acamazsiniz: 2 MiB x 10.000 = 20 GB. Ders 4'te async tam bu yuzden var.

---

### 9 - Paralellik de thread mi kullanıyor?

kısa cevap: genelde evet, ama şart değil

Paralelliğin birimi thread'tir: işletim sistemi thread'leri FARKLI çekirdeklere dağıtır.

Ama thread ≠ paralellik. Tek çekirdekte 100 thread açarsanız eşzamanlılık olur,

   paralellik olmaz — sırayla çalışırlar.

Ve paralellik illa thread gerektirmez: SIMD (tek çekirdek, tek komut çok veri),

   GPU, ya da ayrı süreçler de paralellik verir.

Rust'ta: thread::spawn -> OS dağıtır · rayon -> thread havuzu · async -> tek thread'de eşzamanlılık.

> **Konusmaci notu.** Bu soruyu sinif mutlaka soruyor. Net cevap: paralellik icin bir yurutme birimi lazim ve o birim genelde thread. Ama iliski tek yonlu degil: thread actiginiz an paralellik garanti degil - cekirdek bosta degilse sirada bekler. Async ornegi onemli: Ders 4'te tek thread'de uc isi es zamanli yurutecegiz, hicbir paralellik yok ama sure ucte bire iniyor.

---

### 10 - Rust'ta thread = işletim sistemi thread'i

1:1 model, arada katman yok

std::thread::spawn bir İŞLETİM SİSTEMİ thread'i açar (Linux'ta pthread).

Rust'ın kendi thread zamanlayıcısı yoktur; zamanlayan işletim sistemidir.

std bir runtime taşımaz — aynı dil gömülü sistemde de çalışsın diye.

Go farklı: goroutine'ler M:N — runtime binlerce goroutine'i az sayıda

   OS thread'ine dağıtır. Ucuz, ama runtime taşımak zorundasınız.

O modeli isterseniz kütüphaneden eklersiniz: tokio — Ders 4.

> **Konusmaci notu.** Bu slayt 'zero-cost abstraction' felsefesinin thread'lerdeki karsiligi: Rust size isletim sisteminin verdigini verir, ustune bedel koymaz. Go karsilastirmasi: goroutine ucuz ama runtime zorunlu. Rust ikisini ayirmis: thread = OS, hafif gorev = tokio task.

---

### 11 - spawn ve join

thread açmanın tamamı

```
let handle = thread::spawn(|| {
    // bu blok AYRI bir thread'de calisir
    "sistem bizim"                  // deger dondurebilir
});

// main bu sirada kendi isini yapar...

let sonuc = handle.join().unwrap();  // bekle + degeri al
```

join() çağırmazsanız main bitince thread yarıda kesilebilir. Çıktı sırası GARANTİ DEĞİL.

> **Konusmaci notu.** spawn bir JoinHandle dondurur. join iki is yapar: bekler ve donen degeri verir. unwrap orada cunku thread panikleyebilir - join Result doner. Ciktinin sirasiz olmasi bir hata degil, tanimin kendisi: zamanlayici karar veriyor.

---

### 12 - Eşzamanlılık neden zor?

x += 1 tek bir işlem değil

```
sayac += 1   aslinda uc adim:   oku / arttir / yaz

  cekirdek 0            cekirdek 1
  oku    -> 5
                        oku    -> 5
  arttir -> 6
                        arttir -> 6
  yaz    -> 6
                        yaz    -> 6     <- iki artis, sonuc 6
```

Bir artış kayboldu. HER ZAMAN olmuyor — testlerde geçer, üretimde patlar.

> **Konusmaci notu.** Bu tabloyu tahtaya cizin. Anahtar cumle: hata zamanlamaya bagli, tekrarlanamiyor. Hata ayiklayici eklemek zamanlamayi degistirip hatayi kacirir (heisenbug). Cozum test etmek degil, MUMKUN OLMAMASINI saglamak.

---

### 13 - Rust'ın cevabı: `move`

thread ne kadar yaşayacak, derleyici bilmiyor

```
let ekipman = vec![String::from("EMP granati")];
thread::spawn(|| println!("{:?}", ekipman));

error[E0373]: closure may outlive the current function,
              but it borrows `ekipman`

thread::spawn(move || println!("{:?}", ekipman));  // TASI
println!("{:?}", ekipman);  // E0382: artik senin degil
```

ekipman main'de düşebilir, thread hâlâ çalışıyor olabilir → sarkan referans.

> **Konusmaci notu.** Haftanin en tatmin edici bagi: Gun 2'de ogrendikleri sarkan referans kurali bugun thread'de karsilarina cikiyor. C'de bu kod DERLENIR ve rastgele coker. Rust'ta derlenmez. Ayni verinin iki sahibi olamadigi icin veri yarisinin yarisi zaten kapandi.

---

### 14 - `move` tam olarak ne yapıyor?

sahiplik thread'e GEÇER — geri alamazsınız

```
let ad = String::from("Kaya");
let h = thread::spawn(move || ad.len());
println!("{}", ad);        // E0382: value moved into closure

let sayi = 5;                        // i32 = Copy
let h = thread::spawn(move || sayi * 2);
println!("{}", sayi);      // CALISIR - kopyalandi, tasinmadi
```

Taşımak istemiyorsanız iki yol var: thread::scope ile ödünç alın, ya da Arc ile paylaşın.

> **Konusmaci notu.** Uc noktayi vurgulayin. BIR: move sahipligi devreder, geri donusu yok - main artik o veriye dokunamaz. IKI: Copy tipler istisna degil, sadece kopyalaniyorlar; move yine oluyor ama orijinal yerinde kaliyor. UC: move 'kac kez cagrilir'i belirlemez - move'lu closure hala Fn olabilir, iki kez cagrilabilir. Tasimak istemiyorsaniz thread::scope var: scope icindeki thread'ler scope bitmeden once bittigi icin derleyici odunc almaya izin veriyor. Kod seansinda ikisini de yazacagiz.

---

### 15 - Send ve Sync

derleyici verir, siz yazmazsınız (marker trait)

Send

Sync

Bu tip başka bir thread'e

TAŞINABİLİR mi?

Arc<T>  -> Send

Rc<T>   -> DEĞİL

Sebep: Rc'nin sayacı atomik

değil. İki thread aynı anda

artırırsa sayaç bozulur,

veri erken düşer.

Bu tipe &T ile birden çok

thread'den ERİŞİLEBİLİR mi?

Mutex<T>   -> Sync

RefCell<T> -> DEĞİL

Sebep: RefCell'in ödünç

sayacı çalışma zamanında

tutuluyor ve thread-safe

değil.

> **Konusmaci notu.** Ikisi de MARKER trait: govdesi yok, derleyici otomatik cikariyor. Ogrenciler bu isimleri hata mesajlarinda gorecek: 'cannot be sent between threads safely'. Gun 7'de Rc<RefCell<T>> yazdik; Ders 2'de ayni yapiyi Arc<Mutex<T>> olarak yazacagiz. Sebep bu slayt.

---

### 16 - Rust ne söz veriyor, ne vermiyor

abartmayalım

SÖZ: veri yarışı içeren kod DERLENMEZ. Ownership kuralı bunu kapatıyor.

SÖZ DEĞİL: kilitlenme (deadlock) hâlâ mümkün — derleyici tek satır uyarmaz.

SÖZ DEĞİL: yanlış sıra, eksik kilit, mantık hatası hâlâ sizin sorununuz.

SÖZ DEĞİL: paralel kod otomatik HIZLI değil — thread açmanın bedeli var.

"Fearless concurrency" = korkmadan deneyebilirsiniz; hatasız demek değil.

> **Konusmaci notu.** Rust bir hata SINIFINI kapatiyor - en sinsi olanini. Digerleri duruyor. Ders 2'de deadlock'u, Ders 1'de paralelligin ne zaman zarar ettigini olcerek gorecegiz.

---

### 17 - İki yaklaşım

ikisi de Rust'ta var, ikisini de yazacağız

PAYLAŞIMLI BELLEK

MESAJLAŞMA

Aynı veriye çok thread erişir,

kilitle sıraya sokulur.

Rust: Arc<Mutex<T>>

+ kopyalama yok

- deadlock riski

Ders 2

Veri paylaşılmaz,

SAHİPLİĞİ devredilir.

Rust: mpsc kanalları

+ akıl yürütmesi kolay

- kopyalama/kanal maliyeti

Ders 3

> **Konusmaci notu.** Go'nun slogani: 'Bellegi paylasarak iletisme; ileterek paylas.' Rust ikisine de izin veriyor. Farki: kanala gonderdiginiz veri ELINIZDE KALMIYOR - ownership bunu garanti ediyor, bu yuzden kanalda veri yarisi imkansiz.

---

### 18 - Şimdi thread yazalım.

thread::spawn · move closure · thread::scope · ve paralelliğin ne zaman zarar ettiğini ölçmek

> **Konusmaci notu.** Kod seansina geciyoruz: Ders 1 main.rs.

