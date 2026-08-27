# Gün 6 · Ders 2 — Trait Tanımı ve Bound'lar

## Trait bir sözleşmedir

> "Bu tip şunları yapabilir."

Küçük bir savaş simülasyonu düşünün: okçu, şövalye, ejderha, şifacı. Yaptıkları iş
tamamen farklı — biri ok atar, biri kalkan tutar, biri alev püskürtür. Ama hepsinin
**canı** vardır, **vuruş gücü** vardır ve **nara atarlar**. Ortak olan davranıştır,
veri değil.

```rust
trait Unit {
    fn name(&self) -> &str;          // zorunlu
    fn hp(&self) -> i32;             // zorunlu
    fn attack_power(&self) -> i32;   // zorunlu
}

impl Unit for Archer {
    fn name(&self) -> &str { "Archer" }
    fn hp(&self) -> i32 { self.hp }
    fn attack_power(&self) -> i32 { 12 }
}
```

Java'nın `interface`'ine benziyor, ama iki büyük farkı var:

1. **Varsayılan gövde yazabilirsiniz** (aşağıda)
2. **Başkasının tipine de uygulayabilirsiniz** — `i32`'ye kendi trait'inizi ekleyebilirsiniz
   (sınırları Ders 4'te)

## Varsayılan metotlar

```rust
trait Unit {
    fn name(&self) -> &str;
    fn hp(&self) -> i32;
    fn attack_power(&self) -> i32;

    fn battle_cry(&self) -> String {                   // varsayılan
        format!("{} savasa hazir!", self.name())
    }
    fn is_alive(&self) -> bool {                       // varsayılan
        self.hp() > 0
    }
}
```

Dikkat edilecek iki şey:

- Varsayılan metot **zorunlu metotları çağırabilir**. Henüz yazılmamış bir gövdeyi
  çağırıyor; derleyici bunu, implemente eden tipin yazacağını bildiği için kabul eder.
- İsteyen ezer, istemeyen bedava alır:

```
Archer savasa hazir!                           ← varsayılan gövde
Healer savasa hazir!                           ← varsayılan gövde
Knight kalkanini kaldirdi! (25 zirh)           ← ezilmiş gövde
GRAAAH! Alevler yukseliyor!                    ← ezilmiş gövde
```

Bu, kütüphane yazarının en çok kullandığı tekniktir: trait'e yeni bir metot eklerken
**varsayılan gövde** verirseniz, mevcut kullanıcıların kodu kırılmaz. std bunu sürekli
yapar.

## Trait metodu ile tipin kendi metodu

```rust
impl Archer { fn quiver(&self) -> String { ... } }   // sadece Archer'da (inherent)
impl Unit for Archer { ... }                          // sözleşmenin karşılığı
```

İkisi bir arada yaşar: `archer.quiver()` sadece okçuda vardır, `knight.quiver()` →
`E0599`. Aynı isim çakışırsa tipin kendi metodu öncelik alır; trait sürümünü çağırmak
için `Unit::name(&archer)` yazarsınız.

## Bound'un üç yazımı

Üçü de **aynı** şeyi söyler:

```rust
fn announce_a<T: Unit>(u: &T) -> String          // (1) doğrudan
fn announce_b<T>(u: &T) -> String where T: Unit  // (2) where
fn announce_c(u: &impl Unit) -> String           // (3) impl Trait
```

Peki neden üç tane var? Fark **iki parametre** olunca ortaya çıkar:

| İmza | Anlamı |
|---|---|
| `fn duel<T: Unit>(a: &T, b: &T)` | ikisi **aynı tip** olmak zorunda — aynı sınıf düellosu |
| `fn skirmish(a: &impl Unit, b: &impl Unit)` | ikisi **farklı tip** olabilir — karma savaş |

```rust
duel(&archer, &archer2)      // çalışır, ikisi de Archer
duel(&archer, &dragon)       // E0308: mismatched types
skirmish(&archer, &dragon)   // çalışır
```

`T` bir **tek** tipe bağlanır; `impl Trait` her parametre için ayrı bir tip demektir.
Seçim ölçütü budur, estetik değil.

`duel` içinde iki tarafa da 1d6 atıyoruz — aynı sınıfın iki üyesi eşit vuruşa sahip
olduğu için sonucu zar belirliyor:

```
Archer kazandi (12+5 zar vs 12+3 zar)
```

Rust'ın std'sinde hazır rastgele sayı üreteci **yoktur**; gerçek projede `rand` crate'i
kullanılır. Burada dışarıdan bir şey indirmemek için saatten tohum alıp *xorshift* ile
ilerleten küçük bir `Dice` yazdık — bildiğimiz şeyler: struct, `&mut self` metodu, bit
işlemleri. Her çalıştırmada sonuç değişir.

## Çoklu bound

```rust
fn debug_spawn<T: Unit + Debug>(u: &T)
```

"Hem `Unit` hem `Debug` olacak." `Debug` türetmediğiniz bir tiple çağırırsanız:

```
error[E0277]: `Archer` doesn't implement `Debug`
```

## Dönüşte `impl Trait`

```rust
fn spawn_starter() -> impl Unit { Archer { hp: 80, arrows: 20 } }
```

"Bir `Unit` döndürüyorum, hangisi olduğu sizi ilgilendirmiyor." Çağıran somut tipi
göremez, sadece trait'in metotlarını çağırabilir.

**Ama tek bir somut tip olmak zorunda:**

```rust
fn spawn(boss: bool) -> impl Unit {
    if boss { Dragon { .. } } else { Goblin { .. } }   // E0308
}
```

Sebebi teknik: derleyicinin dönüş değerinin **boyutunu** derleme zamanında bilmesi
gerekir. `Dragon` ile `Goblin` farklı boyutta; "ikisinden biri" diye bir tip yok.

## Duvar — dört birimi tek listeye koyamıyoruz

Dört birimi tek bir orduya koymak isteyin:

```rust
let army = vec![archer, knight, dragon, healer];   // E0308: farklı tipler
```

Olmuyor. `Vec<T>` tek tip tutar; `Archer` ile `Dragon` ayrı tiplerdir. Trait onları
*davranışta* birleştirdi ama *tipte* birleştirmedi. Oysa bir savaş simülasyonunda
yapmak isteyeceğiniz ilk şey tam olarak budur: orduyu bir listeye koyup hepsine
`battle_cry()` dedirtmek.

Bu, generic'lerin sınırıdır. Çözümü ayrı bir mekanizma gerektiriyor: `dyn`.

## Duvarı yıkmak — `dyn Trait`

```rust
let army: Vec<Box<dyn Unit>> = vec![
    Box::new(archer),
    Box::new(knight),
    Box::new(dragon),
    Box::new(healer),
];

for u in &army {
    println!("{}", u.status());
}
```

`Box<dyn Unit>` şu demek: "`Unit`'i implemente eden, ama hangisi olduğunu derleme
zamanında bilmediğim bir şey." `Vec` yine tek tip tutuyor — o tip artık `Box<dyn Unit>`.

`dyn Unit`'in boyutu derleme zamanında bilinmez (bir `Archer` mı, bir `Dragon` mı?).
Bu yüzden her zaman bir pointer'ın arkasında durur: `Box<dyn Unit>` ya da `&dyn Unit`.
Sahiplik gerekmiyorsa referans yeter:

```rust
let front: Vec<&dyn Unit> = vec![&archer, &dragon];
```

### Dönüşte if/else artık mümkün

Yukarıda `impl Unit` ile yapamadığımız şey:

```rust
fn spawn(boss: bool) -> Box<dyn Unit> {
    if boss { Box::new(Dragon { hp: 500, rage: 15 }) }
    else    { Box::new(Archer { hp: 80, arrows: 20 }) }
}
```

Neden şimdi oluyor: `impl Unit` derleme zamanında **tek bir somut tipe** bağlanmak
zorundaydı, çünkü dönüş değerinin boyutu gerekiyordu. `Box<dyn Unit>` her zaman aynı
boyutta — bir pointer. Asıl veri heap'te; `Dragon` ile `Archer`'ın boyut farkı imzayı
ilgilendirmiyor.

### İki dispatch

```rust
fn static_report<T: Unit>(u: &T) -> String { u.status() }   // derleme zamanı
fn dynamic_report(u: &dyn Unit)     -> String { u.status() }  // çalışma zamanı
```

Kaynak kodda aynı satır, çözülme biçimi farklı:

| | statik (`impl` / generic) | dinamik (`dyn`) |
|---|---|---|
| çözülme | derleme zamanı | çalışma zamanı |
| kod boyutu | her tip için ayrı kopya | tek kopya |
| çağrı maliyeti | sıfır, inline olabilir | bir pointer atlaması |
| heterojen liste | ✗ | ✓ |
| dönüşte if/else | ✗ | ✓ |
| derleme süresi | uzar | kısalır |

Ders 1'de monomorphization'ın karşıtı diye geçtiğimiz şey tam olarak bu sütun.

### `dyn` neden iki pointer

```rust
size_of::<&Archer>()        // 8
size_of::<&dyn Unit>()      // 16
size_of::<Box<Archer>>()    // 8
size_of::<Box<dyn Unit>>()  // 16
```

`dyn` bir **fat pointer**: veri pointeri + vtable pointeri.

```
Archer için vtable:
  [ drop | boyut | hizalama | name() | hp() | attack_power() | ... ]

Box<dyn Unit> = (Archer verisine ptr, Archer'ın vtable'ına ptr)
u.attack_power()  ->  vtable'daki ilgili adresteki fonksiyonu çağır
```

Sınıf fat pointer'ı ikinci kez görüyor: Gün 3'te slice ve `&str` de fat pointer'dı
(ptr + uzunluk). Aynı fikir; ikinci alan bu sefer uzunluk değil, vtable.

vtable derleme zamanında üretilir ve ikili dosyanın salt okunur bölümünde durur;
çalışma zamanında sadece okunur. Her **(tip, trait)** çifti için bir tane vardır —
`Archer` üç trait implemente ediyorsa üç ayrı vtable oluşur.

Not: vtable pointeri `Archer`'ın **içinde** durmuyor, referansın içinde. C++'ta tersi:
`virtual` yazdığınız anda o sınıftan her nesne kendi içinde bir vptr taşır, kullansanız
da kullanmasanız da. Rust'ta `Archer`'ı doğrudan kullanırsanız hiçbir şey taşımaz;
bedeli yalnızca `dyn` olarak kullandığınız yerde ödersiniz.

### Karar kuralı

Önce statik deneyin. Heterojen koleksiyon ya da çalışma zamanında tip seçimi
gerekiyorsa dinamiğe geçin. Aradaki fark çoğu uygulamada ölçülemez; asıl kazanç
`Vec<Box<dyn Unit>>` gibi ifade edemediğiniz yapıları ifade edebilmek.

## Bound'suz generic

```rust
fn kimlik<T>(x: T) -> T { x }
```

Hiçbir söz vermediğiniz bir tiple hiçbir şey yapamazsınız — sadece taşıyabilirsiniz.
Bu bir eksiklik değil, imzanın verdiği **garanti**: bu fonksiyon değerinizi okuyamaz,
kıyaslayamaz, yazdıramaz.
