# Gün 7 · Ders 4 — Closure'lar

Büroda "ipucu süzme" kuralları yazıyoruz. Kurallar büronun o anki durumunu — eşik değer,
yasaklı muhbir listesi — bilmek zorunda. Fonksiyonla yapamayacağımız şey tam olarak bu.

## Fonksiyondan tek farkı: çevreyi yakalayabilmesi

```rust
let threshold = 6;
let is_strong = |l: &Lead| l.weight >= threshold;      // threshold yakalandı

fn strong_fn(l: &Lead) -> bool { l.weight >= threshold }
```

```
error[E0434]: can't capture dynamic environment in a fn item
```

Fonksiyonun **çevresi yoktur**: ya parametre alır ya sabit kullanır. Closure çağrıldığı
yerdeki değişkenleri görebilir. Bunun dışında her şeyi fonksiyon da yapar.

```
esik 6 -> guclu ipuclari: ["otoparktaki bilet", "plaka kaydi"]
```

## Üç yakalama şekli

Derleyici **en az kısıtlayıcı** olanı kendisi seçer:

| trait | nasıl yakalar | kaç kez çağrılır |
|---|---|---|
| `Fn` | `&T` — paylaşımlı ödünç | birden çok |
| `FnMut` | `&mut T` — değiştirebilir ödünç | birden çok, durumu değişir |
| `FnOnce` | `T` — sahiplenir | **bir kez** |

Hiyerarşi: her `Fn` aynı zamanda `FnMut`, her `FnMut` aynı zamanda `FnOnce`. Tersi
geçerli değil.

### `Fn` — sadece okuyor

```rust
let banned = String::from("dedikodu");
let allowed = |l: &Lead| l.note != banned;
...
println!("{}", banned);         // hâlâ kullanılabilir: ödünç alındı, taşınmadı
```

### `FnMut` — kendi durumunu değiştiriyor

```rust
let mut seen = 0;
audit(&leads, |l| { seen += 1; total_weight += l.weight as u32; });
```

```
FnMut   : 4 ipucu, toplam agirlik 22
```

Closure'ı alan fonksiyonun parametresi de `mut` olmalı: `mut record: F`.

### `FnOnce` — sahipleniyor, bir kez çağrılıyor

```rust
let case_code = String::from("47-B");
let close_case = move || format!("{} dosyasi kapatildi", case_code);
finalize(close_case);
finalize(close_case);           // E0382: use of moved value
```

## `move`

Yakalamayı **zorla sahiplenme** yapar:

```rust
let detective = String::from("Alvarez");
let sign = move || format!("imza: {}", detective);
sign();
sign();                          // iki kez çağrılabiliyor
println!("{}", detective);       // E0382: closure'a taşındı
```

Dikkat: `move` closure hâlâ `Fn` olabilir — yukarıdaki iki kez çağrıldı.
**`move` "nasıl yakaladığını" belirler, "kaç kez çağrılabileceğini" değil.**

## Closure aslında adsız bir struct'tır

```rust
let yakalayan = |x| x + threshold;
```

kabaca şuna dönüşür:

```rust
struct AdsizClosure { threshold: u8 }

impl AdsizClosure {
    fn call(&self, x: u8) -> u8 { x + self.threshold }
}
```

Yakalanan değerler struct'ın **alanlarıdır**. Boyut da bu yüzden değişir:

```
yakalamayan closure : 0 bayt
u8 yakalayan        : 1 bayt
String yakalayan    : 24 bayt
(String = ptr+len+cap = 24 bayt)
```

Buradan çıkan sonuç: **her closure kendine özgü bir tiptir.** İkisi aynı tip değildir —
bu yüzden closure alan fonksiyonlar generic yazılır:

```rust
fn filter_leads<F>(leads: &[Lead], rule: F) -> Vec<String>
where
    F: Fn(&Lead) -> bool,
```

## Bound'u ihtiyaca göre seçmek

Gerçek bir fonksiyon birden çok kural alır ve her birinin ihtiyacı farklıdır:

```rust
fn screen_batch<V1, V2>(header: &str, leads: &[Lead], header_check: V1, each: V2) -> usize
where
    V1: FnOnce(&str) -> bool,      // sadece bir kez çağrılıyor
    V2: Fn(&Lead) -> bool,         // her ipucu için çağrılıyor
```

```
KRG-12 dosyasi : 2 ipucu
XYZ-9 dosyasi  : 0 ipucu
```

Kural: **en geniş bound'u seçin.** `FnOnce` en geniştir — `Fn` olan bir closure da geçer,
tersi geçmez. Kaç kez çağıracağınızı biliyorsanız fazlasını istemeyin; istemek çağıranı
gereksiz yere kısıtlar.

## `fn` pointer — closure değil, fonksiyon adresi

| | |
|---|---|
| `fn(&Lead) -> bool` | çevre yakalamaz, sadece kod adresi, 8 bayt |
| `impl Fn(&Lead) -> bool` | çevre yakalar, boyut yakaladığına bağlı |

Çevre yakalamayacaksa generic'e gerek yok, tipi doğrudan yazarsınız:

```rust
fn count_matching(leads: &[Lead], rule: fn(&Lead) -> bool) -> usize
```

Bu parametre hem gerçek bir fonksiyonu hem de **hiçbir şey yakalamayan** closure'ı kabul
eder:

```rust
count_matching(&leads, weight_over_five);        // fonksiyon
count_matching(&leads, |l| l.weight > 7);        // yakalamayan closure
```

Ama yakalayan closure geçmez:

```rust
count_matching(&leads, |l| l.weight >= threshold);
```

```
error[E0308]: closures can only be coerced to `fn` types if they do not capture anything
```

Sebebi yukarıdaki struct benzetmesi: `threshold` yakalandığı anda closure artık bir adres
değil, veri taşıyan bir değer.
