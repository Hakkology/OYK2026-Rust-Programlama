# Gün 4 · Ders 1 — Struct'lar ve `impl` Blokları

## Struct nedir

Birbirine ait verileri **tek bir tip** altında toplar. Bugüne kadar iki ayrı `Vec`'i
ya da iki `HashMap`'i elle senkron tutuyorduk; artık gerekmiyor.

```rust
struct Point {
    x: f64,
    y: f64,
}

let n = Point { x: 3.0, y: 4.0 };
println!("{} {}", n.x, n.y);
```

Alan sırası oluştururken önemsizdir, isim yeter.

## Üç struct türü

```rust
struct Point { x: f64, y: f64 }   // klasik — isimli alanlar
struct Meters(f64);               // tuple struct — alanlar isimsiz, .0 ile erişilir
struct Origin;                    // unit-like — hiç alanı yok, 0 bayt
```

## Alan bazlı `mut` yok

```rust
let mut n = Point { x: 0.0, y: 0.0 };
n.x = 5.0;      // tüm struct mut olmak zorunda
```

Tek bir alanı değiştirilebilir yapamazsınız. Borrow checker struct'ı **bir bütün**
olarak takip ediyor: `&mut n` aldığınızda bütün alanları ödünç almış olursunuz.

## `impl` — metotlar

```rust
impl Point {
    fn new(x: f64, y: f64) -> Point { Point { x, y } }   // associated function
    fn length(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}

let n = Point::new(3.0, 4.0);   // :: ile çağrılır, self almaz
println!("{}", n.length());     // . ile çağrılır, self alır
```

`self` almayan fonksiyona **associated function** denir; `String::from`, `Vec::new`
tam olarak bunlar. `new` bir dil özelliği değil, sadece yerleşik bir isim geleneği.

`Point { x, y }` yazımı **field init shorthand**: değişken adı alan adıyla aynıysa
`x: x` yazmanıza gerek yok.

## `self` seçimi — dersin kalbi

Bu tablo Gün 3'te öğrendiğiniz ödünç kurallarının aynısı, sadece metot yazımıyla:

| İmza | Ne yapar | Ödünç karşılığı |
|---|---|---|
| `&self` | okur | `&T` |
| `&mut self` | değiştirir | `&mut T` |
| `self` | **tüketir** | taşıma |

```rust
fn length(&self) -> f64          // okur, nesne çağıranda kalır
fn translate(&mut self, dx: f64)      // değiştirir, nesne çağıranda kalır
fn ada_donustur(self) -> String   // tüketir, nesne bir daha kullanılamaz
```

Her metot yazarken kendinize sorun: bu okuyacak mı, değiştirecek mi, yoksa nesneyi
yutup yerine başka bir şey mi verecek? İmza cevabı okuyucuya söyler.

## Struct update syntax

Bir struct'ı başka birinden türetmek için:

```rust
let ikiz = Planet { moons: 5, ..dunya };
```

Yazmadığınız alanlar `dunya`'dan alınır. **Dikkat:** Copy olmayan alanlar
kopyalanmaz, **taşınır**. `name` bir `String` olduğu için `dunya` bundan sonra
**kısmen taşınmış** sayılır. Üç ayrı durum çıkıyor:

```rust
let ikiz = Planet { moons: 5, ..dunya };

dunya.radius_km      // çalışır — f64 Copy, kopyalandı
dunya.name           // E0382: borrow of moved value — bu alan taşındı
dunya                // E0382: use of partially moved value — bütünü artık kullanılamaz
```

Yani `..` bütünü götürmüyor; sadece Copy olmayan alanları alıyor. Kaybettiğiniz şey
o alan ve struct'ı **bir bütün olarak** kullanabilme hakkı. Tüm alanlar Copy olsaydı
(`Point` gibi) hiçbir şey kaybolmazdı, `dunya` da olduğu gibi kullanılabilirdi.

Bu, Gün 2'de tuple üzerinde gördüğünüz **kısmi move**'un struct hâli.

## Tuple struct — bedelsiz tip güvenliği

```rust
struct Meters(f64);
struct Feet(f64);
```

İkisi de içinde `f64` tutuyor ama **ayrı tiplerdir**, birbirinin yerine geçemezler.
`Meters` bekleyen fonksiyona `Feet` verirseniz `E0308` alırsınız. Çalışma zamanında
hiçbir maliyeti yok; sadece derleyiciye "bu sayı öyle bir sayı değil" demiş oluyorsunuz.

> 1999'da NASA'nın Mars Climate Orbiter uydusu, bir ekip pound-force, diğeri newton
> kullandığı için atmosfere fazla yaklaşıp kayboldu. 327 milyon dolar. İki tarafta da
> sayı doğruydu; **birim** yanlıştı. Tuple struct tam olarak bunu derleme zamanında
> yakalar.

## Unit-like struct

```rust
struct Origin;
```

Alanı yok, bellekte **0 bayt** yer kaplar. Veri taşımayan ama tip olarak var olması
gereken şeyler için kullanılır. `Vec<Origin>` bir milyon eleman tutsa bile veri
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

Struct varsayılan olarak **stack**'te durur. `Vec<Point>` yaparsanız `Point`'lar
heap'te yan yana dizilir — araya pointer girmez, bu da onları hızlı gezilebilir kılar.
