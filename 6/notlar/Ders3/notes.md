# Gün 6 · Ders 3 — Standart Trait'leri Elle Yazmak

Gün 4'te `#[derive(...)]` yazıp geçtiğimiz şeyleri bugün **elle** yazıyoruz. Amaç:
`derive`'ın sihir olmadığını, sadece bu kodu sizin yerinize yazdığını görmek.

Dünya aynı: karakter istatistikleri — `Hp`, `Mana`, `Level`, `CritMultiplier`.

## Neden newtype

```rust
struct Hp(i32);
struct Mana(i32);
```

İkisi de içinde `i32` tutuyor ama **ayrı tipler**. Cana mana ekleyemezsiniz:

```
error[E0308]: expected `Hp`, found `Mana`
```

Bedeli sıfır — çıktıdaki son satır bunu gösteriyor: `Hp` 4 bayt, `i32` 4 bayt.
Sarmalayıcı çalışma zamanında yok.

> Aynı fikir para için de geçerlidir: para `f64` ile tutulmaz, kuruş cinsinden tam sayı
> tutulur (`0.1 + 0.2 != 0.3` sorunu). `Hp(i32)` burada aynı rolü oynuyor.

## `Display` — elle yazılır, `derive` edilemez

```rust
impl fmt::Display for Hp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} can", self.0)
    }
}
```

Neden türetilemiyor? Çünkü kullanıcıya **ne gösterileceği** bir tasarım kararıdır:
`80 can` mı, `HP: 80` mi, `80/100` mü? Derleyici bilemez. `Debug` türetilebilir,
çünkü onun cevabı bellidir: alanları olduğu gibi göster.

Bir kere `Display` yazınca `to_string()` **bedavaya gelir** — std'de şu blanket impl
vardır:

```rust
impl<T: Display> ToString for T
```

## `From` yazınca `Into` bedava

```rust
impl From<Level> for Hp {
    fn from(l: Level) -> Hp { Hp(l.0 as i32 * 20) }    // her seviye 20 can
}
```

Bunu yazdığınız anda üç şey birden kazanırsınız:

```rust
let hp = Hp::from(level);         // From
let mana: Mana = level.into();    // Into — yazmadınız, geldi
```

Sebebi yine std'deki bir blanket impl:

```rust
impl<T, U: From<T>> Into<U> for T
```

**Kural: `From` yazın, `Into`'yu asla elle yazmayın.**

Gün 5'te `?` operatörünün hata tiplerini otomatik çevirdiğini görmüştük — kullandığı
mekanizma tam olarak bu `From` trait'iydi. Bugün aynı şeyin adı kondu.

### `impl Into<T>` parametresi

```rust
fn print_hp<T: Into<Hp>>(x: T)
```

Çağıran `Level` de verebilir, `Hp` de. (`Hp` de çalışır çünkü std'de
`impl<T> From<T> for T` vardır — her tip kendine dönüştürülebilir.) API tasarımında çok
kullanılan kalıptır; `&str` yerine `impl Into<String>` almak da aynı fikir.

## `TryFrom` — dönüşüm başarısız olabiliyorsa

```rust
impl TryFrom<i32> for Hp {
    type Error = NegativeHp;
    fn try_from(v: i32) -> Result<Hp, Self::Error> {
        if v < 0 { Err(NegativeHp(v)) } else { Ok(Hp(v)) }
    }
}
```

`From` kayıpsız ve **her zaman başarılı** dönüşümler içindir. Başarısız olabiliyorsa
`TryFrom` kullanılır ve `Result` döner:

```
Hp::try_from(250)  ->  Ok(Hp(250))
Hp::try_from(-30)  ->  Err(NegativeHp(-30))
```

Negatif can diye bir şey yoktur; tip sistemi bunu tutuyor.

## Operatörler birer trait'tir

`+` demek `Add::add` demektir:

```rust
impl Add for Hp {
    type Output = Hp;
    fn add(self, o: Hp) -> Hp { Hp(self.0 + o.0) }
}
```

`type Output` bir **associated type**: dönüş tipini trait'in kendisi taşıyor. Ayrıntısı
Ders 5'te; bugün "toplamanın sonucu ne tipte olacak, onu burada söylüyoruz" düzeyinde yeter.

| Operatör | Trait |
|---|---|
| `+` `-` `*` `/` `%` | `Add` `Sub` `Mul` `Div` `Rem` |
| `+=` `-=` | `AddAssign` `SubAssign` |
| `-x` | `Neg` |
| `==` | `PartialEq` |
| `<` `>` | `PartialOrd` |
| `x[i]` | `Index` |

Sağ taraf farklı tip de olabilir — o zaman trait'in generic parametresini verirsiniz:

```rust
impl Mul<CritMultiplier> for Hp { ... }   // Hp * CritMultiplier — kritik vuruş
impl Add<u32> for Level { ... }           // Level + u32 — seviye atlama
```

## `Ord` neden `Level`'da var, `CritMultiplier`'da yok

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)] struct Level(u32);           // olur
#[derive(PartialEq, PartialOrd)]          struct CritMultiplier(f64);  // Eq/Ord OLMAZ
```

`CritMultiplier` içinde `f64` var; `NaN != NaN` olduğu için tam eşitlik ve tam sıralama
tanımlı değil. Sonuç pratikte şöyle görünür:

```rust
levels.sort();                                   // çalışır
crits.sort();                                    // E0277: `CritMultiplier: Ord` sağlanmıyor
crits.sort_by(|a, b| a.partial_cmp(b).unwrap()); // PartialOrd yetiyor
```

Buradaki zincir bir **supertrait** zinciridir: `Ord: Eq + PartialOrd`, `Eq: PartialEq`.
`Eq` kopunca `Ord` da kopuyor. Supertrait'ler Ders 4'ün konusu.

## Hangi trait nasıl gelir — özet

| Trait | Ne verir | Nasıl |
|---|---|---|
| `Debug` | `{:?}` | derive |
| `Display` | `{}` ve `to_string()` | **elle** |
| `Clone` / `Copy` | kopyalama | derive |
| `PartialEq` / `Eq` | `==` | derive |
| `PartialOrd` / `Ord` | `<`, `sort`, `min`, `max` | derive |
| `Hash` | `HashMap` anahtarı | derive |
| `Default` | `Tip::default()` | derive ya da elle |
| `From` / `Into` | kayıpsız dönüşüm | **elle** (`Into` bedava) |
| `TryFrom` | başarısız olabilen dönüşüm | **elle** |
| `Add`, `Mul`, … | operatörler | **elle** |
| `Drop` | temizlik | **elle** |

## Marker trait'ler

Bazı trait'lerin hiç metodu yoktur; sadece "bu tip şu özelliğe sahip" der. `Copy` bunun
en tanıdık örneğidir: gövdesi boştur, derleyiciye "bu tipi taşımak yerine kopyala"
demekten ibarettir. `Sized`, `Send`, `Sync` da böyledir.

Metotsuz trait garip görünür ama tip sistemine bilgi taşımak da bir iştir.
