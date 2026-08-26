# Gün 5 · Ders 4 — Declarative Makrolar (`macro_rules!`)

## Neden makro var

Bir fonksiyon **değişken sayıda argüman alamaz** ve **tip üretemez**. `println!` bir
fonksiyon olsaydı `println!("{}", a)` ile `println!("{} {}", a, b)` aynı imzaya sığmazdı.
`vec![]` bir fonksiyon olsaydı `vec![0; 100]` yazılamazdı.

Makro, derleyiciye **kod yazdıran koddur**: derleme sırasında genişler, ürettiği kod
normal Rust olarak derlenir. Sonunda `!` gören her şey makrodur.

## Makro derlemenin neresinde çalışır

Gün 1'deki `rustc` boru hattını hatırlayın; makro açılımı orada belli bir yerde durur:

```
kaynak kod (.rs)
  │
  ├── Lexer + Parser ──────► AST        söz dizimi ağacı
  │
  ├── Ad çözümleme ────────► HIR        ◄── MAKROLAR BURADA AÇILIR
  │
  ├── Tip denetimi ────────► MIR        borrow check
  │
  ├── Çeviri ──────────────► LLVM IR
  │
  └── Kod üretimi ─────────► makine kodu
```

Buradan çıkan üç sonuç var ve üçü de makroların davranışını açıklar:

**1. Makro tipleri görmez.** Açılım, tip denetiminden **önce** biter. Makroya gelen şey
token'dır; `42` ile `"metin"` onun için aynı kategoridedir (`expr`). Bu yüzden makro
kolları tipe göre ayrışamaz.

**2. Makro geçerli söz dizimi üretmek zorundadır, doğru kod üretmek zorunda değildir.**
Ürettiği kodun tip güvenliği ve ödünç kuralları **sonraki** aşamalarda denetlenir. Yani
makronuz derlenir ama ürettiği kod tip hatası verebilir.

**3. Hata mesajları açılmış koda bakar.** Derleyici sizin yazdığınız `avec![...]`
satırını değil, onun yerine geçen kodu denetler. Makro hatalarının okunması bu yüzden
zordur; ilk yapılacak iş üretilen kodu görmektir:

```
cargo install cargo-expand
cargo expand
```

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
| `item` | bir öğe — `fn`, `struct`, `impl` bloğu |
| `meta` | öznitelik içi — `derive(Debug)` |
| `vis` | görünürlük — `pub`, `pub(crate)` |
| `lifetime` | `'a` |

En çok `expr` ve `ident` kullanılır. Seçim önemlidir: `expr` yakalarsanız derleyici
gelen şeyi **ifade olarak ayrıştırır** ve bütünlüğünü korur; `tt` yakalarsanız ham
token alırsınız (aşağıdaki parantez tuzağı tam olarak bu farktan çıkıyor).

```rust
macro_rules! type_alias {
    ($t:ty => $ad:ident) => { type $ad = $t; };
}
type_alias!(u32 => Counter);
```

## Tek tek: `$( $eleman:expr ),* $(,)?`

Bu satır makro öğrenirken en çok korkutan şey; hâlbuki beş parçadan ibaret:

| Parça | Ne demek |
|---|---|
| `$` | "burada bir makro değişkeni var" işareti |
| `$eleman` | yakalanan şeye **sizin verdiğiniz ad** — `$x`, `$sayi`, ne isterseniz |
| `:expr` | **ne tür** bir şey yakalanacağı (fragment specifier) |
| `$( ... )` | içindeki desen **tekrarlanabilir** |
| `,*` | `*`'dan hemen önceki karakter **ayırıcıdır**: virgülle ayrılmış, sıfır veya daha fazla |
| `$(,)?` | sondaki **fazladan virgüle izin ver** (sıfır veya bir tane) |

Yani `avec![1, 2, 3]` ve `avec![1, 2, 3,]` ikisi de tutar, `avec![]` de tutar.

### Tekrar işaretleri

```
$( ... ),*     virgülle ayrılmış SIFIR veya daha fazla
$( ... ),+     virgülle ayrılmış BİR veya daha fazla
$( ... );*     noktalı virgülle ayrılmış (ayırıcı istediğiniz token olabilir)
$( ... )*      ayırıcısız tekrar
$( ... )?      sıfır veya bir — **ayırıcı ALAMAZ**
```

`$( $x:expr ),?` yazarsanız derlenmez:

```
error: the `?` macro repetition operator does not take a separator
```

Sebebi mantıklı: en fazla bir tane şey varsa arasına ayıracak bir şey yoktur.

Gövde tarafında da aynı sarmalı kullanırsınız; yakalanan her parça için o satır
yeniden üretilir:

```rust
$( v.push($eleman); )*      // üç eleman geldiyse üç push satırı
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

## Makro, aşırı yükleme (overloading) değildir

Çok kollu makro görünce akla ilk gelen şey bu oluyor, ama değil. Kollar **tipe göre
değil, biçime göre** ayrışır:

```rust
macro_rules! t {
    ($x:expr) => { "tek ifade" };
    ($x:expr, $y:expr) => { "iki ifade" };
}

t!(42)        // "tek ifade"
t!("metin")   // "tek ifade"   <- tip farklı ama AYNI kol
t!(1, 2)      // "iki ifade"   <- argüman sayısı farklı, kol da farklı
```

`42` ile `"metin"` aynı kola düşüyor, çünkü ikisi de birer `expr`. Makro tipleri
görmez; makro genişlediğinde ortada henüz tip denetimi yoktur, sadece token vardır.

Rust'ta aşırı yükleme **yoktur** ve bu bilinçli bir karardır. "Aynı işi farklı tipler
için yapmak" istediğinizde doğru araçlar şunlardır:

| İhtiyaç | Rust'taki karşılığı |
|---|---|
| farklı tipleri kabul eden tek fonksiyon | trait sınırı: `fn f(x: impl Into<String>)` |
| aynı işi farklı tipler için tanımlamak | trait implementasyonu (`impl From<A> for B`) |
| değişken sayıda argüman | **makro** — `println!`, `vec!` |
| isteğe bağlı parametreler | `Option` parametre ya da builder |

Yani makro, aşırı yüklemenin değil, **değişken argüman sayısının** cevabıdır.

Akılda kalacak cümle:

> **Rust'ta method overloading yoktur.** Arity ve söz dizimi esnekliği için **makrolar**,
> tip bazlı çok biçimlilik için **trait ve generic'ler** kullanılır.

## Hijyen — C'de olmayan şey

```rust
macro_rules! increment {
    ($x:ident) => { $x += 1; };
}

let mut sayac = 0;
increment!(sayac);      // çalışır: ismi DIŞARIDAN aldık
```

Ama makronun kendi içinde tanımladığı değişken dışarıyı **kirletmez**:

```rust
macro_rules! no_pollution {
    () => { let x = 42; };      // buradaki x, dışarıdaki x DEĞİLDİR
}
```

Rust makroları **kısmen hijyeniktir** (partially hygienic): derleyici makro içinde
tanımlanan isimlere ayrı bir bağlam etiketi (`SyntaxContext`) verir, böylece o isimler
çağrıldıkları yerdeki isimlerle çakışmaz. C'de bu yüzden `_tmp_1234` gibi isimler
uydurulur; Rust'ta gerek yok.

"Kısmen" demesinin sebebi: yerel değişken adları hijyeniktir, ama tip adları, fonksiyon
adları ve `$ident` ile **dışarıdan aldığınız** isimler çağrıldıkları bağlamda çözülür —
zaten `increment!(sayac)` örneğinin çalışmasının sebebi de budur.

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
macro_rules! impl_max {
    ( $( $t:ty ),+ ) => {
        $( impl MaxValue for $t {
               fn max_value() -> Self { <$t>::MAX }
           } )+
    };
}
impl_max!(u8, u16, u32, i8, i16, i32);
```

Altı tip için altı `impl` bloğu — elle yazsanız otuz satır, üstelik biri unutulur.

## TT muncher — token'ları tek tek yemek

Makrolar **özyinelemeli** olabilir. Karmaşık söz dizimlerini çözmenin klasik yolu,
token'ları baştan bir bir tüketip kalanı kendine geri vermektir; buna *token tree
muncher* denir:

```rust
macro_rules! token_say {
    () => { 0 };                                        // taban durum
    ($ilk:tt $($geri:tt)*) => { 1 + token_say!($($geri)*) };  // birini ye, kalanı devret
}

token_say!()        // 0
token_say!(a b c)   // 3
token_say!(1 + 2)   // 3   — üç token: 1, +, 2
```

`tt` en ilkel yapıtaşıdır: tek bir token ya da parantezle çevrili bir grup. Kendi küçük
dilinizi (DSL) yazacaksanız yöntem budur — ama hata mesajları hızla okunmaz hâle gelir,
o yüzden gerçekten gerekmedikçe uzak durun.

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
