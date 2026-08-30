# Gün 6 · Ders 3 (v1) — Standart Trait'leri Elle Yazmak

> Ders 3'ün **ikinci anlatımı**. Konular aynı; dünya farklı: **bilet ve tarife**.

Gün 4'te `derive` ettiklerimizi bugün elle yazıyoruz.

## Neden newtype

```rust
struct Fare(i64);        // KURUŞ cinsinden
struct Distance(u32);    // metre
struct Zone(u8);
struct Discount(f64);
```

Dördü de sayı tutuyor ama **ayrı tipler**. Ücrete mesafe ekleyemezsiniz:

```rust
tek_binis + Distance(500)     // error[E0308]: expected `Fare`, found `Distance`
```

Bedeli sıfır — çıktıdaki son satır bunu gösteriyor: `Fare` 8 bayt, `i64` 8 bayt.
Sarmalayıcı çalışma zamanında yok.

> Para neden `f64` değil: `0.1 + 0.2 != 0.3`. Para kuruş cinsinden **tam sayı** tutulur.
> `Fare(i64)` burada tam bu rolü oynuyor.

## `Display` — elle yazılır, `derive` edilemez

```rust
impl fmt::Display for Fare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:02} TL", self.0 / 100, self.0 % 100)
    }
}
```

```
17,50 TL / 8,50 TL
850 m / 12.4 km
```

Neden türetilemiyor? Çünkü kullanıcıya **ne gösterileceği** bir tasarım kararıdır:
`17,50 TL` mi, `1750 krş` mü, `₺17.50` mi? Derleyici bilemez. `Debug` türetilebilir,
çünkü onun cevabı bellidir: alanları olduğu gibi göster.

Bir kere `Display` yazınca `to_string()` **bedavaya gelir** — std'de
`impl<T: Display> ToString for T` blanket impl'i vardır.

## `From` yazınca `Into` bedava

```rust
impl From<Zone> for Fare {
    fn from(z: Zone) -> Fare { Fare(750 + z.0 as i64 * 250) }
}
```

Bunu yazdığınız anda iki şey birden kazanırsınız:

```rust
let ucret: Fare = Fare::from(bolge);      // From
let mesafe: Distance = bolge.into();      // Into — yazmadınız, geldi
```

```
3. bolge -> 15,00 TL + 9.0 km
```

`into()` "bu değeri şu tipe çevir" demektir; hangi tipe çevireceğini **hedefin tipinden**
anlar. Bir şeyi güncellemez, yeni değer üretir — bölge atlayınca ücreti yeniden
türetirsiniz:

```
bolge atlayinca: 4. bolge -> 17,50 TL
```

Sebebi std'deki blanket impl:

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U { U::from(self) }     // gövdesi bu kadar
}
```

Yani `into()` ayrı bir mekanizma değil: `bolge.into()` derlendiğinde `Fare::from(bolge)`
çağrısına dönüşüyor.

**Kural: `From` yazın, `Into`'yu asla elle yazmayın.**

Gün 5'te `?` operatörünün hata tiplerini otomatik çevirdiğini görmüştük — kullandığı
mekanizma tam olarak bu `From` trait'iydi.

### `impl Into<T>` parametresi

```rust
fn print_fare<T: Into<Fare>>(x: T)
```

Çağıran `Zone` de verebilir, `Fare` de. (`Fare` de çalışır çünkü std'de
`impl<T> From<T> for T` vardır — her tip kendine dönüştürülebilir.)

## `TryFrom` — dönüşüm başarısız olabiliyorsa

```rust
impl TryFrom<i64> for Fare {
    type Error = NegativeFare;
    fn try_from(v: i64) -> Result<Fare, Self::Error> {
        if v < 0 { Err(NegativeFare(v)) } else { Ok(Fare(v)) }
    }
}
```

```
Ok(Fare(2500))
Err(NegativeFare(-30))
```

### Hata neden `String` değil de kendi tipi

`NegativeFare` de bir newtype: **reddedilen değeri yanında taşıyor** ve çağıran onu
kalıpla ayıklayabiliyor:

```rust
Err(NegativeFare(v)) => println!("gecersiz: {}", v),
```

`String` olsaydı hata metnini ayrıştırmak zorunda kalırdınız. std de aynısını yapar:
`u8::try_from(300)` size metin değil, `TryFromIntError` döndürür.

Bir hata tipi yazınca sözleşmenin tamamını veriyoruz — Gün 5'ten tanıdık:

```rust
impl fmt::Display for NegativeFare { ... }   // kullanıcıya gösterilen hâli
impl Error for NegativeFare {}               // "bu bir hatadır"
```

`Error`'ı yazmazsanız `?` bu hatayı `Box<dyn Error>`'a çeviremez. Yazınca çalışıyor:

```
? ile   : hata -> negatif ucret olmaz: -30
```

## Operatörler birer trait'tir

`+` demek `Add::add` demektir:

```rust
impl Add for Fare {
    type Output = Fare;
    fn add(self, o: Fare) -> Fare { Fare(self.0 + o.0) }
}
```

`type Output` bir **associated type**: dönüş tipini trait'in kendisi taşıyor. Ayrıntısı
Ders 5'te.

Sağ taraf farklı tip de olabilir — o zaman trait'in generic parametresini verirsiniz:

```rust
impl Mul<Discount> for Fare      // Fare * Discount
impl Add<u8> for Zone            // Zone + u8
```

```
aktarmali yolculuk : 20,00 TL
%40 indirim        : 10,50 TL
bolge atladi       : 3. bolge
sifirin altina inmez: 0,00 TL
```

Son satır bir tasarım kararı: `Sub` içinde `.max(0)` var, ücret sıfırın altına inmiyor.
Operatörün **anlamını siz belirliyorsunuz**.

## `Ord` neden `Zone`'da var, `Discount`'ta yok

`Zone` tam sayı tutuyor: `Eq` ve `Ord` türetilebiliyor, `sort()` çalışıyor.

```
1. bolge | 2. bolge | 3. bolge | 5. bolge |
```

`Discount` içinde `f64` var; `NaN != NaN` olduğu için tam eşitlik ve tam sıralama tanımlı
değil:

```rust
indirimler.sort();
```

```
error[E0277]: the trait bound `Discount: Ord` is not satisfied
```

Çözüm `PartialOrd` ile sıralamak:

```rust
indirimler.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

## Hangi trait nasıl gelir — özet

| Trait | Nasıl |
|---|---|
| `Debug` | `derive` |
| `Clone`, `Copy` | `derive` (Copy için tüm alanlar Copy olmalı) |
| `PartialEq`, `Eq` | `derive` (`f64` varsa `Eq` olmaz) |
| `PartialOrd`, `Ord` | `derive` (`f64` varsa `Ord` olmaz) |
| `Default` | `derive` |
| `Display` | **elle** — tasarım kararı |
| `From` / `TryFrom` | **elle** |
| `Add`, `Sub`, `Mul` | **elle** |
| `Into`, `ToString` | **yazmayın** — blanket impl'den geliyor |
