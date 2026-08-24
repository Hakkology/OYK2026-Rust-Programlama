# Gün 4 · Ders 1 — Struct'lar ve `impl` Blokları

## Struct nedir

Birbirine ait verileri **tek bir tip** altında toplar. Bugüne kadar iki ayrı `Vec`'i
ya da iki `HashMap`'i elle senkron tutuyorduk; artık gerekmiyor.

```rust
struct Nokta {
    x: f64,
    y: f64,
}

let n = Nokta { x: 3.0, y: 4.0 };
println!("{} {}", n.x, n.y);
```

Alan sırası oluştururken önemsizdir, isim yeter.

## Üç struct türü

```rust
struct Nokta { x: f64, y: f64 }   // klasik — isimli alanlar
struct Metre(f64);                // tuple struct — alanlar isimsiz, .0 ile erişilir
struct Baslangic;                 // unit-like — hiç alanı yok, 0 bayt
```

## Alan bazlı `mut` yok

```rust
let mut n = Nokta { x: 0.0, y: 0.0 };
n.x = 5.0;      // tüm struct mut olmak zorunda
```

Tek bir alanı değiştirilebilir yapamazsınız. Borrow checker struct'ı **bir bütün**
olarak takip ediyor: `&mut n` aldığınızda bütün alanları ödünç almış olursunuz.

## `impl` — metotlar

```rust
impl Nokta {
    fn yeni(x: f64, y: f64) -> Nokta { Nokta { x, y } }   // associated function
    fn uzunluk(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}

let n = Nokta::yeni(3.0, 4.0);   // :: ile çağrılır, self almaz
println!("{}", n.uzunluk());     // . ile çağrılır, self alır
```

`self` almayan fonksiyona **associated function** denir; `String::from`, `Vec::new`
tam olarak bunlar. `yeni` bir dil özelliği değil, sadece yerleşik bir isim geleneği.

`Nokta { x, y }` yazımı **field init shorthand**: değişken adı alan adıyla aynıysa
`x: x` yazmanıza gerek yok.

## `self` seçimi — dersin kalbi

Bu tablo Gün 3'te öğrendiğiniz ödünç kurallarının aynısı, sadece metot yazımıyla:

| İmza | Ne yapar | Ödünç karşılığı |
|---|---|---|
| `&self` | okur | `&T` |
| `&mut self` | değiştirir | `&mut T` |
| `self` | **tüketir** | taşıma |

```rust
fn uzunluk(&self) -> f64          // okur, nesne çağıranda kalır
fn otele(&mut self, dx: f64)      // değiştirir, nesne çağıranda kalır
fn ada_donustur(self) -> String   // tüketir, nesne bir daha kullanılamaz
```

Her metot yazarken kendinize sorun: bu okuyacak mı, değiştirecek mi, yoksa nesneyi
yutup yerine başka bir şey mi verecek? İmza cevabı okuyucuya söyler.

## Struct update syntax

Bir struct'ı başka birinden türetmek için:

```rust
let ikiz = Gezegen { uydu: 5, ..dunya };
```

Yazmadığınız alanlar `dunya`'dan alınır. **Dikkat:** bu alanlar kopyalanmaz,
**taşınır**. `ad` bir `String` olduğu için `dunya` artık kullanılamaz (`E0382`).
Sayı alanları Copy olduğu için onlarda böyle bir sorun yok — yani struct'ın içinde
`String` varsa `..` bütünü götürür.

## Tuple struct — bedelsiz tip güvenliği

```rust
struct Metre(f64);
struct Ayak(f64);
```

İkisi de içinde `f64` tutuyor ama **ayrı tiplerdir**, birbirinin yerine geçemezler.
`Metre` bekleyen fonksiyona `Ayak` verirseniz `E0308` alırsınız. Çalışma zamanında
hiçbir maliyeti yok; sadece derleyiciye "bu sayı öyle bir sayı değil" demiş oluyorsunuz.

> 1999'da NASA'nın Mars Climate Orbiter uydusu, bir ekip pound-force, diğeri newton
> kullandığı için atmosfere fazla yaklaşıp kayboldu. 327 milyon dolar. İki tarafta da
> sayı doğruydu; **birim** yanlıştı. Tuple struct tam olarak bunu derleme zamanında
> yakalar.

## Unit-like struct

```rust
struct Baslangic;
```

Alanı yok, bellekte **0 bayt** yer kaplar. Veri taşımayan ama tip olarak var olması
gereken şeyler için kullanılır. `Vec<Baslangic>` bir milyon eleman tutsa bile veri
için ek bellek harcamaz — saklanacak bir şey yok.

## Bellekte struct

```rust
size_of::<(u8, u32, u8)>()   // 8, 6 değil
```

Aradaki fark **hizalama** (alignment): `u32` 4'ün katı bir adreste durmak ister,
derleyici araya boşluk (padding) koyar.

Rust alanların sırasını **değiştirebilir** ve bunu daha az padding için yapar; C'de
sıra yazdığınız gibi kalır. Bu yüzden Rust'ta alanları büyükten küçüğe dizmek gibi
bir el işçiliğine gerek yok.

Struct varsayılan olarak **stack**'te durur. `Vec<Nokta>` yaparsanız `Nokta`'lar
heap'te yan yana dizilir — araya pointer girmez, bu da onları hızlı gezilebilir kılar.
