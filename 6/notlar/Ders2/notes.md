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
    fn name(&self) -> &str { "Okcu" }
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
Okcu savasa hazir!                             ← varsayılan gövde
Sifaci savasa hazir!                           ← varsayılan gövde
Sovalye kalkanini kaldirdi! (25 zirh)          ← ezilmiş gövde
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

İkisi bir arada yaşar: `okcu.quiver()` sadece okçuda vardır, `sovalye.quiver()` →
`E0599`. Aynı isim çakışırsa tipin kendi metodu öncelik alır; trait sürümünü çağırmak
için `Unit::name(&okcu)` yazarsınız.

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
duel(&okcu, &okcu2)        // çalışır, ikisi de Archer
duel(&okcu, &ejderha)      // E0308: mismatched types
skirmish(&okcu, &ejderha)  // çalışır
```

`T` bir **tek** tipe bağlanır; `impl Trait` her parametre için ayrı bir tip demektir.
Seçim ölçütü budur, estetik değil.

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

## Bugünün duvarı

Dört birimi tek bir orduya koymak isteyin:

```rust
let ordu = vec![okcu, sovalye, ejderha, sifaci];   // E0308: farklı tipler
```

Olmuyor. `Vec<T>` tek tip tutar; `Archer` ile `Dragon` ayrı tiplerdir. Trait onları
*davranışta* birleştirdi ama *tipte* birleştirmedi. Oysa bir savaş simülasyonunda
yapmak isteyeceğiniz ilk şey tam olarak budur: orduyu bir listeye koyup hepsine
`battle_cry()` dedirtmek.

Bu, generic'lerin sınırıdır ve bilinçli olarak burada bırakılıyor: çözümü ayrı bir
mekanizma gerektiriyor.

## Bound'suz generic

```rust
fn kimlik<T>(x: T) -> T { x }
```

Hiçbir söz vermediğiniz bir tiple hiçbir şey yapamazsınız — sadece taşıyabilirsiniz.
Bu bir eksiklik değil, imzanın verdiği **garanti**: bu fonksiyon değerinizi okuyamaz,
kıyaslayamaz, yazdıramaz.
