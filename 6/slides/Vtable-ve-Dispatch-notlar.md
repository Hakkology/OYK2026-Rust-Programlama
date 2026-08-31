# Vtable ve Dispatch — sunum notları

`Vtable-ve-Dispatch.pptx` sunumunun yazılı hâli. **Ders değildir**; Gün 6'nın beş dersi
yerinde durur, bu ~20 dakikalık ek sunumdur.

Buradaki bütün sayılar ölçülerek elde edildi (`g++` ve `rustc` çıktıları).

## 1. Önce kavram: ikisi de polimorfizm

```
                    POLİMORFİZM
                         |
        +----------------+----------------+
   STATİK (derleme)              DİNAMİK (çalışma)
   fn f<T: Trait>(t: T)          fn f(t: &dyn Trait)
   impl Trait                    Box<dyn Trait>
   C++: template                 C++: virtual + pointer
   her tip için ayrı kod         tek kod + vtable araması
   direct call / inline          indirect call
```

Sık yapılan hata: "trait'ler statiktir, `dyn` polimorfizmdir" demek. **İkisi de
polimorfizmdir**; trait ikisinin de aracıdır. Fark, çağrılacak fonksiyonun adresinin
**ne zaman** belli olduğudur.

## 2. Monomorphization

`fn render<T: Draw>(item: &T)` yazdığınızda derleyici kullanılan **her somut tip için
ayrı bir kopya** üretir:

```rust
render(&player);   // -> render'ın Player sürümü
render(&enemy);    // -> render'ın Enemy sürümü
```

Kanıt (`nm` sembol tablosu):

```
$ nm -C vtdemo | grep render_static
... t main::render_static      <- Player sürümü
... t main::render_static      <- Enemy sürümü

$ nm -C vtdemo | grep render_dynamic
... t main::render_dynamic     <- TEK sürüm
```

**Statik dağıtım 2 kopya, dinamik dağıtım 1 kopya.** İkili boyutunun neden şiştiği ve
dinamik dağıtımın neden küçük kaldığı tek satırda burada.

## 3. C++ template mantığı ve farkı

C++ şablonu **iki aşamalı** derlenir:

1. **Tanım aşaması** — şablon yazıldığında yalnızca sözdizimi denetlenir. Tip
   bilinmediği için makine kodu üretilmez; şablon bir taslaktır.
2. **Örneklendirme (instantiation)** — `draw<Player>(p)` çağrıldığı anda `T` yerine
   `Player` konur, o tipe özel fonksiyon türetilip derlenir.

Rust generic'i de aynı şeyi yapar: tek kaynak, çok kopya. **Kritik fark hatanın nerede
çıktığıdır:**

| | C++ template (concepts öncesi) | Rust generic |
|---|---|---|
| Denetim | duck typing — gövde tipe uygulanınca | trait bound — imzada |
| Hata nerede | örneklendirmede, kullanım yerinde | imzada, tanım yerinde |
| Hata mesajı | onlarca satır instantiation izi | tek satır, `E0369` |
| Sözleşme | yazılı değil | `<T: Draw>` olarak okunur |

Tek cümle: **C++ şablonu "dener, tutmazsa patlar"; Rust generic'i "söz verir, söz
tutulmazsa derlenmez."**

## 4. Inlining ve vektörizasyon

Normal bir fonksiyon çağrısında CPU şunları yapar:

```
1. parametreleri register/stack'e yaz
2. CALL <adres>        adrese zıpla
3. yeni stack frame aç
4. gövdeyi çalıştır
5. RET                 geri dön, frame'i kapat
```

Statik dağıtımda derleyici hedefin adresini **derleme anında bilir**. Bunu bilince
`CALL` ve `RET` komutlarını tamamen siler ve gövdeyi çağıran kodun içine yapıştırır.
Çağrının maliyeti gerçekten sıfıra iner, çünkü ortada çağrı kalmaz.

**Vektörizasyon (SIMD)** bunun ikinci hediyesidir: fonksiyon sınırı kalkınca derleyici
veri akışını uçtan uca görür ve dört `f32` toplamasını dört komut yerine tek AVX/SSE
komutuna indirebilir. Döngüyü açabilir, gereksiz yüklemeleri eleyebilir.

**`dyn Trait`'te bunların hiçbiri olmaz:** adres çalışma zamanında vtable'dan okunur
(`CALL [rax+16]`). Derleyici nereye gidileceğini bilmediği için gövdeyi göremez, inline
edemez, dallanmayı öngöremez.

## 5. "Zero-cost abstraction" — sıfır olan ne

> "Kullanmadığın şeyin bedelini ödemezsin. Kullandığın şeyi ise elle yazabileceğinden
> daha performanslı yazamazsın." — Bjarne Stroustrup

Sıfır olan iki şey var:

**Bellekte:** `struct Player { x: f32, y: f32 }` tam 8 bayttır. İçinde gizli işaretçi,
metadata, nesne başlığı yoktur. Trait uygulamak boyutu **değiştirmez**.

**Çağrıda:** üretilen assembly, aynı işi elle yazsanız çıkacak assembly ile aynıdır.
Soyutlama katmanı derleme sırasında buharlaşır.

Sıfır **olmayan** taraf: derleme süresi ve ikili dosya boyutu. Fatura derleme zamanında
ödenir. C++ dünyasında buna *code bloat* denir; `serde` gibi ağır generic kullanan
crate'lerde gözle görülür.

## 6. C++'ta vtable — işaretçi nesnenin içinde

Bir sınıfta en az bir `virtual` metot varsa derleyici nesnenin başına gizli bir **vptr**
yerleştirir. Ölçüm (`g++ -O0`):

```
sizeof(PlainEnemy)  = 4      // virtual yok
sizeof(VirtEnemy)   = 16     // 8 vptr + 4 int + 4 padding
sizeof(VirtEnemy*)  = 8      // işaretçi normal boyutta
```

Nesnenin ilk 8 baytını okursanız gerçekten vtable adresini görürsünüz:

```cpp
void** vptr = *(void***)&v;   // -> 0x58e14e4e0d98
```

```
  VirtEnemy nesnesi            statik bellek
  +----------------+
  | vptr  ---------+---------> +--------------------+
  +----------------+           |  VirtEnemy vtable  |
  | hp = 30        |           +--------------------+
  +----------------+           | &VirtEnemy::draw   |
                               | &~VirtEnemy        |
                               +--------------------+
```

Şemanın sözle anlatımı: `VirtEnemy` nesnesinin **ilk 8 baytı** bir `vptr`'dir ve statik
bellekteki vtable'ı gösterir; hemen ardından `hp` alanı gelir. Vtable'ın içinde sırayla
`draw` ve yıkıcı fonksiyonun adresleri durur.

Nesne polimorfik olduğunu **kendi içinde taşır**; kurucu çalışırken vptr bağlanır.
Bedeli her nesnede +8 bayttır: bir milyon nesne = 8 MB fazladan.

## 7. Rust'ta vtable — işaretçi referansın içinde

Rust struct'ı trait'lerden **tamamen habersizdir**:

```
size_of::<Rock>()   = 4 bayt     // hiçbir trait yok
size_of::<Enemy>()  = 4 bayt     // Draw uyguluyor — AYNI
```

Bedel yalnızca `dyn` yazdığınızda doğar ve nesnede değil **referansta** durur:

```
size_of::<&Enemy>()                = 8 bayt    ince işaretçi
size_of::<&dyn Draw>()             = 16 bayt   FAT POINTER
size_of::<Box<dyn Draw>>()         = 16 bayt
size_of::<Option<Box<dyn Draw>>>() = 16 bayt   None bedava (niche)
```

```
  &dyn Draw  (16 bayt)
  +----------------+----------------+
  | veri işaretçi  | vtable işaretçi|
  +-------+--------+--------+-------+
          |                 |
          v                 v
  +---------------+   +----------------------+
  | Enemy         |   | Draw for Enemy vtable|
  | hp = 30       |   +----------------------+
  | (saf veri)    |   | drop_in_place        |
  +---------------+   | size = 4             |
                      | align = 4            |
                      | &Enemy::draw         |
                      +----------------------+
```

Şemanın sözle anlatımı: `&dyn Draw` 16 bayttır ve iki işaretçiden oluşur. İlki `Enemy`
verisini gösterir — o veri **saf**tır, içinde tablo işaretçisi yoktur. İkincisi
`Draw for Enemy` vtable'ını gösterir; tablonun içinde sırayla `drop_in_place`, `size`,
`align` ve `draw` metodunun adresi durur.

Fat pointer'ı ikiye ayırıp bakabilirsiniz:

```
veri adresi   = 0x7ffea93b7cec
&e adresi     = 0x7ffea93b7cec    (aynı adres)
vtable adresi = 0x57b9697292d8
ikinci Enemy  = 0x57b9697292d8    (AYNI tablo paylaşılıyor)
```

Vtable **tip başına bir kez** üretilir; nesne sayısı artınca tablo çoğalmaz.

### Vtable'ın gerçek düzeni

Belleği okuyup ölçtük (`size = 16`, `align = 8` olan bir tiple):

```
[vtable +  0] = 0x5c9...eb0   drop_in_place
[vtable +  8] = 16            size
[vtable + 16] = 8             align
[vtable + 24] = 0x5c9...ed0   1. TRAIT METODU
[vtable + 32] = 0x5c9...ec0   2. trait metodu
```

**Dikkat:** ilk metot `+16`'da değil, **`+24`**'te. `size` ve `align` ayrı ayrı 8'er
bayt yer kaplar. İnternette sık görülen "`[rax+16]` ilk metottur" ifadesi yanlıştır.

`Box<dyn Draw>` kapsam dışına çıkınca doğru `Drop` implementasyonu bu tablodan bulunur.
`main.rs`'teki `Tracked` örneği tam olarak bunu gösteriyor: derleyici tipi bilmiyordu,
tabloyu takip etti.

### Dinamik çağrı fiziksel olarak nasıl oluyor

**Adım 1 — fat pointer ikiye ayrılır ve iki register'a gider:**

```
fn dinamik(d: &dyn Draw) -> u64 { d.draw() + 1 }

%rdi = veri işaretçisi    -> Enemy nesnesi (self)
%rsi = vtable işaretçisi  -> Draw for Enemy tablosu
```

**Adım 2 — çağrılacak adres tablodan okunur.** Kodun içinde bir adres yazmaz; adres
`%rsi + 24` konumundaki **veridir**.

**Adım 3 — üretilen gerçek assembly** (`rustc -O --emit asm`):

```asm
; DİNAMİK
    pushq  %rax
    callq  *24(%rsi)      ; vtable[24]'teki ADRESE dolaylı çağrı
    incq   %rax
    retq

; STATİK — aynı iş
    movq   (%rdi), %rax   ; çağrı YOK, inline oldu
    incq   %rax
    retq
```

`*` işareti "bu adresteki değere git" demektir. Statik sürümde `draw()` çağrısı
tamamen kaybolmuştur — üç komut kalmıştır. "Zero cost" tam olarak budur.

### "Derleyici nereye gidileceğini bilmez" ne demek

- Statikte hedef **sabittir**: `callq 0x19730` — adres kodun içinde yazılı.
- Dinamikte hedef **veridir**: `callq *24(%rsi)` — adres çalışırken okunur.
- Gövdeyi göremediği için inline edemez, döngüyü açamaz, SIMD uygulayamaz.
- CPU dallanmayı tahmin etmek zorunda kalır (indirect branch prediction).
- Buna karşılık tablo **tip başına bir kez** üretilir; nesne sayısı arttıkça çoğalmaz.

## 8. C++ ve Rust yan yana

| | C++ (`virtual`) | Rust (`dyn Trait`) |
|---|---|---|
| İşaretçi nerede | nesnenin **içinde** (vptr) | referansın **içinde** (fat pointer) |
| Düz nesne boyutu | `sizeof(data) + 8` | `sizeof(data)` |
| Referans boyutu | 8 bayt | 16 bayt |
| Ne zaman doğar | sınıfta `virtual` varsa | `&dyn` / `Box<dyn>` yazınca |
| Kime uygulanır | sınıf hiyerarşisi | her tip + trait ikilisi |
| Sonradan eklenebilir mi | hayır | evet (orphan rule sınırında) |
| Tercih edilebilir mi | hayır, tip polimorfikse hep öder | evet, kullanan öder |

Özet: **C++'ta bedeli nesne öder, Rust'ta kullanım öder.**

## 9. Object safety — her trait `dyn` olamaz

Üç ihlal, üçü de `E0038` (hepsi doğrulandı). Kuralın eski adı *object safety*,
derleyicinin bugünkü dilinde *dyn compatibility*:

```rust
trait T { fn f<X>(&self, x: X); }         // generic metot
trait T { fn clone_me(&self) -> Self; }   // Self döndürüyor
trait T { fn create() -> Self; }          // self almıyor
```

```
error[E0038]: the trait `T` is not dyn compatible
```

Sebep tek ve fizikseldir: **vtable'da her metodun tek bir adresi olmalı ve boyutu
bilinmeli.**

- Generic metot: her `X` için ayrı kod gerekir; tabloya kaç adres koyacaksınız?
- `Self` dönüşü: dönen tipin boyutu çağıranda bilinmiyor.
- `self` almayan metot: hangi nesne üzerinden çağrılacağı belli değil.

**Kaçış kapısı:** metodu `where Self: Sized` ile işaretlersiniz; o metot vtable'a
girmez, trait `dyn` olabilir, metot yalnızca somut tipte çağrılır.

```rust
trait T {
    fn ok(&self);
    fn only_sized(&self) where Self: Sized;
}
let d: &dyn T = &S;   d.ok();       // derlenir
```

std bunu çok kullanır; `Iterator`'ın adaptörleri böyle işaretlidir.

## 10. Mimari kararı

| Durum | Seçim | Neden |
|---|---|---|
| Tek tipli koleksiyon (`Vec<Player>`) | generic | monomorphize, inline, SIMD |
| Karışık sahne (`Vec<Box<dyn Entity>>`) | `dyn` | tek listede farklı tipler |
| Eklenti / plugin sistemi | `dyn` | tip derleme anında bilinmiyor |
| Sıcak döngü, milyonlarca çağrı | generic | indirect call maliyeti yok |
| İkili boyutu kritik (gömülü) | `dyn` | tek kopya, bloat yok |
| Derleme süresi acı veriyor | `dyn` | monomorphization faturası düşer |

Trait ile mimari kurarken:

- **Küçük ve odaklı trait'ler** yazın: `Damageable`, `Renderable`, `Serializable`.
- Bir tip istediği kadarını uygular — mix-in mantığı, kalıtım ağacı yok.
- **Varsayılan gövde**, trait'e metot eklerken mevcut kodu kırmaz.
- **Supertrait** "önce şu olmalısın" der: `Boss: Unit + Display`.
- Sınıf hiyerarşisi kurmaya çalışmayın; Rust'ta kalıtım yoktur.

Varsayılan **generic**'tir. `dyn`'i ihtiyaç kanıtlandığında seçersiniz: heterojen
koleksiyon, eklenti sınırı, ikili boyutu.

## Yedi cümlelik özet

1. Statik ve dinamik dağıtımın ikisi de polimorfizmdir; fark çözülme anıdır.
2. Monomorphization: her tip için ayrı kod — C++ template'iyle aynı fikir.
3. Rust generic'i sözü imzada verir; C++ template'i örneklendirmede patlar.
4. Zero-cost: **çalışma zamanı** sıfır; derleme süresi ve ikili boyutu değil.
5. C++'ta vptr nesnenin içindedir; Rust'ta vtable işaretçisi referansın içinde.
6. Rust vtable'ı: `drop`(+0), `size`(+8), `align`(+16), metotlar(+24'ten itibaren).
7. `dyn` olabilmek için trait object-safe olmalı; ihlaller `E0038`.
