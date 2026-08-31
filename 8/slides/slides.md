# Gun 8 - Ders 1: Eszamanlilik, Thread ve Ownership

Gun 8 acilis sunumu

> `Eszamanlilik-Thread-Ownership.pptx` sunumunun metin hali.

---

### 1 - GÜN 8 · DERS 1 · 20 DK

Eşzamanlılık, Thread ve Ownership

Bilgisayarınızda 431 program çalışıyor. Çekirdek sayısı 16.

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

---

### 4 - İşletim sistemi 431 süreci nasıl yönetiyor?

zaman dilimi + bağlam değiştirme

```
cekirdek 0'in zaman cizgisi:

  |--tarayici--|--muzik--|--derleyici--|--tarayici--|--terminal--|
       ~5 ms      ~5 ms       ~5 ms         ~5 ms        ~5 ms
  (tek cekirdegin zaman cizgisi: her surec sirayla ~5 ms calisiyor,
   sonra zamanlayici siradakine geciyor)

Her degisimde ZAMANLAYICI (scheduler) devreye giriyor:
  1. calisan surecin register'larini kaydet
  2. siradakinin register'larini yukle
  3. bellek haritasini (MMU) degistir

Buna BAGLAM DEGISTIRME (context switch) deniyor.
```

Bu makinede iki thread arasında gidiş-dönüş ~14 µs ölçüldü. Bedava değil — ama 5 ms'lik dilimin yanında küçük.

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

---

### 6 - Thread'in üç hâli

zamanlayıcı bunlar arasında gezdiriyor

ÇALIŞIYOR (running) — şu an bir çekirdekte. En fazla 16 tane olabilir.

HAZIR (ready) — işi var, sırasını bekliyor. Çekirdek boşalınca girecek.

BLOKE (blocked) — I/O bekliyor: ağ, disk, kilit. Çekirdeği BIRAKTI.

Bloke thread çekirdek harcamaz — ama 2 MiB stack'ini tutmaya devam eder.

10.000 bağlantı = 10.000 bloke thread = 20 GB. İşte async'in sebebi (Ders 4).

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

Şemanın sözle anlatımı: bir sürecin içinde kod, static alan ve heap **ortaktır** — `Box`,
`Vec`, `String` verileri heap'te durur. Aynı sürecin her thread'inin ise **kendi stack'i ve
kendi register'ları** vardır; onlar paylaşılmaz.

İki SÜREÇ hiçbir şey paylaşmaz. Aynı sürecin iki THREAD'i heap'i paylaşır — bütün mesele burada.

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

---

### 9 - Paralellik de thread mi kullanıyor?

kısa cevap: genelde evet, ama şart değil

Paralelliğin birimi thread'tir: işletim sistemi thread'leri FARKLI çekirdeklere dağıtır.

Ama thread ≠ paralellik. Tek çekirdekte 100 thread açarsanız eşzamanlılık olur,

   paralellik olmaz — sırayla çalışırlar.

Ve paralellik illa thread gerektirmez: SIMD (tek çekirdek, tek komut çok veri),

   GPU, ya da ayrı süreçler de paralellik verir.

Rust'ta: thread::spawn -> OS dağıtır · rayon -> thread havuzu · async -> tek thread'de eşzamanlılık.

---

### 10 - Rust'ta thread = işletim sistemi thread'i

1:1 model, arada katman yok

std::thread::spawn bir İŞLETİM SİSTEMİ thread'i açar (Linux'ta pthread).

Rust'ın kendi thread zamanlayıcısı yoktur; zamanlayan işletim sistemidir.

std bir runtime taşımaz — aynı dil gömülü sistemde de çalışsın diye.

Go farklı: goroutine'ler M:N — runtime binlerce goroutine'i az sayıda

   OS thread'ine dağıtır. Ucuz, ama runtime taşımak zorundasınız.

O modeli isterseniz kütüphaneden eklersiniz: tokio — Ders 4.

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

---

### 16 - Rust ne söz veriyor, ne vermiyor

abartmayalım

SÖZ: veri yarışı içeren kod DERLENMEZ. Ownership kuralı bunu kapatıyor.

SÖZ DEĞİL: kilitlenme (deadlock) hâlâ mümkün — derleyici tek satır uyarmaz.

SÖZ DEĞİL: yanlış sıra, eksik kilit, mantık hatası hâlâ sizin sorununuz.

SÖZ DEĞİL: paralel kod otomatik HIZLI değil — thread açmanın bedeli var.

"Fearless concurrency" = korkmadan deneyebilirsiniz; hatasız demek değil.

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

---

### 18 - Şimdi thread yazalım.

thread::spawn · move closure · thread::scope · ve paralelliğin ne zaman zarar ettiğini ölçmek
