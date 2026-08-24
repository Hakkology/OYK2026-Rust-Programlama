# Gün 4 · Ders 2 — Derive, Hata Ayıklama ve Akıcı API

## `derive` ne yapıyor

Bir trait'in gerektirdiği kodu derleyici sizin yerinize yazıyor:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Nokta { x: f64, y: f64 }
```

Tek satır yazdınız, karşılığında `{:?}` ile yazdırma, `.clone()`, atamada kopyalanma
ve `==` kazandınız. Elle yazsanız onlarca satır tutardı.

| Trait | Ne verir | Koşul |
|---|---|---|
| `Debug` | `{:?}` ve `{:#?}` | alanlar da `Debug` olmalı |
| `Clone` | `.clone()` | alanlar `Clone` |
| `Copy` | atamada taşıma yerine kopya | **tüm alanlar Copy** — `String` varsa olmaz |
| `PartialEq` | `==` ve `!=` | alanlar `PartialEq` |
| `Eq` | tam eşitlik | `f64` alan varsa **olmaz** |
| `PartialOrd` / `Ord` | `<`, `>`, `sort()` | alan sırasına göre leksikografik |
| `Hash` | `HashMap`/`HashSet` anahtarı | `Eq` gerekir |
| `Default` | `Tip::default()` | alanların sıfır değeri |

## İki tuzak

**`String` alanı varsa `Copy` olmaz.** `Copy` "bit bit kopyala, sahiplik derdi yok"
demek; `String` heap tutuyor, bit kopyası iki sahip üretirdi. Derleyici izin vermez.

**`f64` alanı varsa `Eq` ve `Ord` olmaz.** Sebep: `NaN != NaN`. Kayan noktada tam
eşitlik ve tam sıralama tanımlı değil, o yüzden Rust size sadece `PartialEq` ve
`PartialOrd` verir. `f64` listesini sıralamak için `sort_by(|a, b| a.partial_cmp(b).unwrap())`
kalıbı kullanılır.

Bu ikisi, Gün 3'te "`f64` neden `HashMap` anahtarı olamıyor" sorusunun aynı cevabı.

## `Ord` leksikografiktir

Türetilmiş sıralama **alanları yazdığınız sırayla** karşılaştırır: önce birinci alan,
eşitse ikinci alan.

```rust
struct Sure { dakika: u32, saniye: u32 }
```

Böyle bir tipte sıralama doğru çalışır. Alan sırasını `saniye, dakika` yapsaydınız
sıralama saçmalardı — derleyici uyarmaz, çünkü ne demek istediğinizi bilemez.

## `Debug` ve `Display` farkı

- `{:?}` → **Debug**, geliştirici için, `derive` edilebilir
- `{:#?}` → aynısı, satır satır girintili
- `{}` → **Display**, kullanıcı için, **elle yazılır**

`Display` neden derive edilemiyor? Çünkü kullanıcıya ne gösterileceğini derleyici
bilemez. `Sure { dakika: 3, saniye: 45 }` ekrana `3:45` mi, `225 sn` mi, `3 dk 45 sn`
mi yazmalı? Bu bir ürün kararı, derleyicinin işi değil.

```rust
use std::fmt;

impl fmt::Display for Sure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}", self.dakika, self.saniye)
    }
}
```

`{:02}` iki haneye sıfır dolduruyor — `3:5` değil `3:05`.

## Hata ayıklama araçları

- `dbg!(x)` — değeri dosya/satır bilgisiyle **stderr**'e yazar ve **geri döndürür**
- `println!` stdout'a, `eprintln!` stderr'e yazar
- `assert!` / `assert_eq!` — beklentiniz bozulursa program orada durur

`dbg!`'ın geri döndürmesi kritik: kodu bozmadan araya sıkıştırabilirsiniz.

```rust
let toplam = dbg!(a + b) * 2;    // hem yazdırır hem hesap devam eder
```

Neden stderr? Programın gerçek çıktısını kirletmesin diye. `./program > cikti.txt`
dediğinizde `dbg!` satırları dosyaya değil ekrana gider.

## Builder pattern — akıcı API

Bir tipin 8 alanı varsa, 8 parametreli bir `yeni` fonksiyonu okunmaz hâle gelir:

```rust
Karakter::yeni("Ejderha", 120, 15, true, false, 3, 0, 250)   // hangisi neydi?
```

Bunun yerine her adımı adıyla söyleyen bir zincir:

```rust
let ejder = Karakter::yeni("Ejderha")
    .can(120)
    .saldiri(15)
    .ucabilir()
    .olustur();
```

Nasıl çalışıyor: her metot `mut self` alır, bir alanı değiştirir ve `self`'i geri
döndürür. Yani nesne halkadan halkaya **taşınır**.

```rust
fn can(mut self, x: u32) -> Self {
    self.can = x;
    self
}
```

Faydası: alan sırası önemsiz, sadece istediğinizi belirtirsiniz, gerisi varsayılan kalır.

Bedeli: her adım nesneyi tükettiği için ara değişken tutup **iki farklı zincire**
sokamazsınız — ikincisinde `E0382` alırsınız.
