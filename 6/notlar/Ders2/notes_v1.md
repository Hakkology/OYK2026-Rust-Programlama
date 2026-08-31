# Gün 6 · Ders 2 (v1) — Trait Tanımı ve Bound'lar

> Ders 2'nin **ikinci anlatımı**. Konular birebir aynı; dünya farklı: **şehir ulaşım ağı**.

Vapur, metro, tramvay, fünikülerin yaptıkları iş tamamen farklı. Ama hepsinin ortak bir
**sözleşmesi** var: bir hattı vardır, kapasitesi vardır, ücreti vardır.

## Trait bir sözleşmedir

```rust
trait Vehicle {
    fn line(&self) -> &str;             // zorunlu
    fn capacity(&self) -> u32;          // zorunlu
    fn fare(&self) -> u32;              // zorunlu

    fn announce(&self) -> String {      // VARSAYILAN gövde
        format!("{} hatti kalkiyor", self.line())
    }
}
```

Java'nın `interface`'ine benziyor, ama iki büyük farkı var:

1. **Varsayılan gövde yazabilirsiniz**
2. **Başkasının tipine de uygulayabilirsiniz** — `i32`'ye kendi trait'inizi ekleyebilirsiniz
   (sınırları Ders 4'te)

## Varsayılan metotlar

İsteyen ezer, istemeyen bedava alır:

```
Konak-Karsiyaka hatti kalkiyor   (varsayılan gövde)
Funikuler hatti kalkiyor   (varsayılan gövde)
M1 Fahrettin Altay: 5 vagon, kapilar kapaniyor   (ezilmiş)
T1 Konak: saga dikkat, tramvay geciyor   (ezilmiş)
```

Dikkat edilecek iki şey:

- Varsayılan metot **zorunlu metotları çağırabilir**. `status()` içinde `is_large()`,
  onun içinde `capacity()` var. Henüz yazılmamış bir gövdeyi çağırıyor; derleyici bunu,
  implemente eden tipin yazacağını bildiği için kabul ediyor.
- Kütüphane yazarının en çok kullandığı teknik budur: trait'e yeni metot eklerken
  **varsayılan gövde** verirseniz mevcut kullanıcıların kodu kırılmaz.

## Trait metodu ile tipin kendi metodu

```rust
impl Ferry {
    fn deck_report(&self) -> String { ... }    // trait'e ait DEĞİL
}
```

İkisi bir arada yaşar: `ferry.deck_report()` sadece vapurda vardır, `metro.deck_report()`
→ `E0599`.

## Bound'un üç yazımı

```rust
fn announce_a<T: Vehicle>(v: &T) -> String
fn announce_b<T>(v: &T) -> String where T: Vehicle
fn announce_c(v: &impl Vehicle) -> String
```

Üçü de **aynı** şeyi söyler. Peki neden üç tane var? Fark **iki parametre** olunca ortaya
çıkar:

| | |
|---|---|
| `fn race<T: Vehicle>(a: &T, b: &T)` | ikisi **aynı tip** olmak zorunda — aynı türden iki araç |
| `fn transfer(a: &impl Vehicle, b: &impl Vehicle)` | ikisi **farklı tip** olabilir — aktarmalı yolculuk |

```rust
race(&tram, &tram2)        // çalışır, ikisi de Tram
race(&tram, &metro)        // E0308: mismatched types
transfer(&ferry, &metro)   // çalışır
```

Seçim ölçütü budur, estetik değil.

`race` içinde iki tarafa da 0–5 dakika gecikme atıyoruz — aynı türden iki araç eşit
olduğu için sonucu zar belirliyor:

```
ayni tur : T1 Konak once vardi (1 dk gecikme, digeri 4 dk)
```

Rust'ın std'sinde hazır rastgele sayı üreteci **yoktur**; gerçek projede `rand` crate'i
kullanılır. Burada dışarıdan bir şey indirmemek için saatten tohum alıp *xorshift* ile
ilerleten küçük bir `Dice` yazdık. Her çalıştırmada sonuç değişir.

## Çoklu bound

```rust
fn debug_dispatch(v: &(impl Vehicle + Debug))
```

"Hem `Vehicle` hem `Debug` olacak." `Debug` türetmediğiniz bir tiple çağırırsanız:

```
error[E0277]: `Tram` doesn't implement `Debug`
```

## Dönüşte `impl Trait`

```rust
fn first_departure() -> impl Vehicle { Tram { ... } }
```

"Bir `Vehicle` döndürüyorum, hangisi olduğu sizi ilgilendirmiyor." **Ama tek bir somut tip
olmak zorunda:**

```rust
fn pick(rush: bool) -> impl Vehicle {
    if rush { Metro { .. } } else { Tram { .. } }   // E0308
}
```

Sebebi teknik: derleyicinin dönüş değerinin **boyutunu** derleme zamanında bilmesi
gerekir.

## Duvar — dört aracı tek listeye koyamıyoruz

```rust
let sefer = vec![ferry, metro, tram, funicular];   // E0308: farklı tipler
```

`Vec<T>` tek tip tutar. Trait onları *davranışta* birleştirdi ama *tipte* birleştirmedi.
Oysa bir sefer listesinde yapmak isteyeceğiniz ilk şey tam olarak budur.

## Duvarı yıkmak — `dyn Trait`

```rust
let schedule: Vec<Box<dyn Vehicle>> = vec![
    Box::new(ferry), Box::new(metro), Box::new(tram), Box::new(funicular),
];
```

```
Konak-Karsiyaka        450 kisi   1750 krs  [buyuk]
M1 Fahrettin Altay    1100 kisi   1500 krs  [buyuk]
T1 Konak               180 kisi   1500 krs  [kucuk]
Funikuler               90 kisi   1500 krs  [kucuk]
sefer listesi kapasitesi: 1820 kisi
```

`Box<dyn Vehicle>` şu demek: "`Vehicle`'ı implemente eden, ama hangisi olduğunu derleme
zamanında bilmediğim bir şey." `Vec` yine tek tip tutuyor — o tip artık `Box<dyn Vehicle>`.

`dyn Vehicle`'ın boyutu derleme zamanında bilinmez, bu yüzden her zaman bir pointer'ın
arkasında durur: `Box<dyn Vehicle>` ya da `&dyn Vehicle`. Sahiplik gerekmiyorsa referans
yeter:

```rust
let peron: Vec<&dyn Vehicle> = vec![&ferry, &metro];
```

### Dönüşte if/else artık mümkün

```rust
fn pick(rush: bool) -> Box<dyn Vehicle> {
    if rush { Box::new(Metro { .. }) } else { Box::new(Tram { .. }) }
}
```

`Box<dyn Vehicle>` her zaman aynı boyutta — bir pointer. Asıl veri heap'te.

### İki dispatch

| | statik (`impl` / generic) | dinamik (`dyn`) |
|---|---|---|
| çözülme | derleme zamanı | çalışma zamanı |
| kod boyutu | her tip için ayrı kopya | tek kopya |
| çağrı maliyeti | sıfır, inline olabilir | bir pointer atlaması |
| heterojen liste | yok | var |
| dönüşte if/else | yok | var |
| derleme süresi | uzar | kısalır |

Ders 1'de monomorphization'ın karşıtı diye geçtiğimiz şey tam olarak bu sütun.

### `dyn` neden iki pointer

```
&Ferry              8 bayt
&dyn Vehicle       16 bayt
Box<Ferry>          8 bayt
Box<dyn Vehicle>   16 bayt
```

`dyn` bir **fat pointer**: veri pointeri + vtable pointeri. Sınıf fat pointer'ı ikinci kez
görüyor — Gün 3'te slice ve `&str` de fat pointer'dı (ptr + uzunluk). Aynı fikir; ikinci
alan bu sefer uzunluk değil, vtable.

vtable derleme zamanında üretilir, ikili dosyanın salt okunur bölümünde durur. Her
**(tip, trait)** çifti için bir tane vardır.

Not: vtable pointeri `Ferry`'nin **içinde** durmuyor, referansın içinde. C++'ta tersi:
`virtual` yazdığınız anda o sınıftan her nesne kendi içinde bir vptr taşır. Rust'ta
`Ferry`'yi doğrudan kullanırsanız hiçbir şey taşımaz.

### Karar kuralı

Önce statik deneyin. Heterojen koleksiyon ya da çalışma zamanında tip seçimi gerekiyorsa
dinamiğe geçin. Aradaki fark çoğu uygulamada ölçülemez; asıl kazanç
`Vec<Box<dyn Vehicle>>` gibi ifade edemediğiniz yapıları ifade edebilmek.

## Bound'suz generic

```rust
fn kimlik<T>(x: T) -> T { x }
```

Hiçbir söz vermediğiniz bir tiple hiçbir şey yapamazsınız — sadece taşıyabilirsiniz.
Bu bir eksiklik değil, imzanın verdiği **garanti**.
