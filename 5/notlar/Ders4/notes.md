# Gün 5 · Ders 4 — Declarative Makrolar (`macro_rules!`)

## Neden makro var

Bir fonksiyon **değişken sayıda argüman alamaz** ve **tip üretemez**. `println!` bir
fonksiyon olsaydı `println!("{}", a)` ile `println!("{} {}", a, b)` aynı imzaya sığmazdı.
`vec![]` bir fonksiyon olsaydı `vec![0; 100]` yazılamazdı.

Makro, derleyiciye **kod yazdıran koddur**: derleme sırasında genişler, ürettiği kod
normal Rust olarak derlenir. Sonunda `!` gören her şey makrodur.

## `macro_rules!` anatomisi

```rust
macro_rules! isim {
    ( DESEN )        => { ÜRETİLECEK KOD };
    ( BAŞKA DESEN )  => { ... };
}
```

Bir `match`'e benziyor, ama değerler üzerinde değil **kod parçaları üzerinde** eşleşiyor.
Kollar yukarıdan aşağıya denenir, ilk uyan kazanır.

Çağırırken üç parantez de aynıdır: `avec!(...)`, `avec![...]`, `avec!{...}`.
Gelenek: `vec![]` köşeli, `println!()` yuvarlak, `macro_rules!{}` süslü.

## Yakalama tipleri (fragment specifier)

| Tip | Ne yakalar |
|---|---|
| `expr` | bir ifade — `2 + 3`, `f(x)` |
| `ident` | bir isim — değişken, fonksiyon adı |
| `ty` | bir tip — `u32`, `Vec<String>` |
| `literal` | sabit — `42`, `"abc"` |
| `pat` | desen — `Some(x)` |
| `block` | `{ ... }` |
| `stmt` | bir deyim |
| `path` | `std::io::Read` |
| `tt` | tek bir token ağacı — en esnek, en zor |

En çok `expr` ve `ident` kullanılır.

```rust
macro_rules! tip_takma_ad {
    ($t:ty => $ad:ident) => { type $ad = $t; };
}
tip_takma_ad!(u32 => Sayac);
```

## Tekrar

```
$( ... ),*     virgülle ayrılmış SIFIR veya daha fazla
$( ... ),+     virgülle ayrılmış BİR veya daha fazla
$( ... )?      sıfır veya bir
```

Düzenli ifadelerdeki `*` ve `+` ile aynı mantık. Yakalanan her parça için gövde tekrar
üretilir:

```rust
macro_rules! avec {
    ( $( $eleman:expr ),* ) => {{
        let mut v = Vec::new();
        $( v.push($eleman); )*      // her eleman için bir push satırı üretilir
        v
    }};
}
```

Gövdenin **çift süslü parantezle** (`{{ ... }}`) yazıldığına dikkat edin: dıştaki makro
gövdesi, içteki üretilen blok. Blok bir ifadedir, son satırı (`v`) değeri olur.

Küçük ayrıntı: `avec![]` boş çağrıldığında hiç `push` üretilmez, `let mut v` de gereksiz
`mut` olur ve derleyici uyarır. Üretilen koda uyarı bastırmak makro yazarken normaldir:

```rust
#[allow(unused_mut)]
let mut v = Vec::new();
```

### Sondaki virgül

`avec![1, 2, 3,]` yazılınca desen tutmaz. Çözüm ayrı bir kol ya da `$(,)?`:

```rust
( $( $eleman:expr ),* $(,)? ) => { ... }
```

Bu yüzden std makroları sondaki virgülü kabul eder — biri oturup bu kolu yazmıştır.

## Parantez tuzağı — C'de var, Rust'ta `expr` ile yok

C'nin klasik makro derdi şudur:

```c
#define KARE(x) x * x
KARE(2 + 3)      // 2 + 3 * 2 + 3  =  11
```

Aynısını Rust'ta yazarsanız **sonuç 25 çıkar**:

```rust
macro_rules! kare { ($x:expr) => { $x * $x }; }
kare!(2 + 3)     // 25
```

Sebep önemli: `expr` yakalaması metin kopyalamaz, **ayrıştırılmış tek bir ifade düğümü**
yakalar. Yerine konurken bütünlüğü korunur, yani derleyici zaten `(2 + 3) * (2 + 3)`
görür. Rust burada C'den daha güvenlidir.

Peki tuzak hiç yok mu? Token seviyesinde yakalarsanız (`tt`) geri gelir:

```rust
macro_rules! kare_tt { ( $($x:tt)* ) => { $($x)* * $($x)* }; }
kare_tt!(2 + 3)  // 11
```

Aynı üç satır, üç farklı sonuç: C → 11, Rust `tt` → 11, Rust `expr` → 25.
**Kural: elinizde bir ifade varsa `expr` yakalayın.** `tt` en esnek yakalamadır ama
ifade bütünlüğünü korumaz; gerçekten token'larla oynamanız gerekmedikçe kullanmayın.

## Hijyen — C'de olmayan şey

```rust
macro_rules! artir {
    ($x:ident) => { $x += 1; };
}

let mut sayac = 0;
artir!(sayac);      // çalışır: ismi DIŞARIDAN aldık
```

Ama makronun kendi içinde tanımladığı değişken dışarıyı **kirletmez**:

```rust
macro_rules! kirletmez {
    () => { let x = 42; };      // buradaki x, dışarıdaki x DEĞİLDİR
}
```

Rust makroları hijyeniktir: makro içinde üretilen isimler ayrı bir "renk" taşır.
C'de bu yüzden `_tmp_1234` gibi isimler uydurulur; Rust'ta gerek yok.

## Dışa açmak: `#[macro_export]` ve `$crate`

```rust
#[macro_export]
macro_rules! avec { ... }
```

`#[macro_export]` makroyu crate kökünden dışarıya açar. Makro gövdesinde kendi
crate'inizin bir öğesine atıf yapacaksanız `$crate::` yazın — kullanıcının kodunda
`crate` başka bir şeyi işaret eder:

```rust
const C: usize = $crate::say![@SAY; $($eleman),*];
```

`@SAY` gibi başlangıçlar bir dil özelliği değil, **iç kol işaretlemek için gelenek**:
"bu kol kullanıcıya değil, makronun kendine ait".

## Tekrar eden `impl`'leri makroyla yazmak

Makronun en meşru kullanımlarından biri: aynı gövdeyi çok sayıda tip için üretmek.

```rust
macro_rules! max_uygula {
    ( $( $t:ty ),+ ) => {
        $( impl EnBuyuk for $t {
               fn en_buyuk() -> Self { <$t>::MAX }
           } )+
    };
}
max_uygula!(u8, u16, u32, i8, i16, i32);
```

Altı tip için altı `impl` bloğu — elle yazsanız otuz satır, üstelik biri unutulur.

## Ne zaman makro yazmalı

- Değişken sayıda argüman gerekiyorsa
- Aynı `impl` çok sayıda tip için tekrarlanıyorsa
- İsimleri metin olarak kullanmanız gerekiyorsa (`stringify!`)

**Bunların dışında fonksiyon yazın.** Makro hata mesajlarını bozar, IDE tamamlamasını
zayıflatır, okunması zordur. "Fonksiyonla olmuyor mu?" sorusunun cevabı "oluyor" ise
makro yazmayın.

## Genişlemeyi görmek

```
cargo install cargo-expand
cargo expand
```

Makronun ürettiği gerçek kodu gösterir. Makro hata ayıklamanın tek pratik yolu budur.
