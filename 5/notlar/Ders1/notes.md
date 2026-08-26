# Gün 5 · Ders 1 — Hata Felsefesi ve `Result`

## Rust'ta exception yok

Çoğu dilde bir fonksiyon "sessizce" hata fırlatabilir; imzasına bakarak anlayamazsınız,
`try/catch` yazmayı unutursanız program çalışırken çöker. Rust'ta hata **normal akışın
bir parçası**: fonksiyon hatayı **döndürür**, siz de ele almak zorunda kalırsınız.

Gün 4'ün cümlesi buydu: bilgi tipin içine yazılır. `Option` "değer olmayabilir" diyordu,
`Result` "işlem başarısız olabilir **ve nedeni var**" diyor.

## İki dünya

| | Ne der | Ne zaman |
|---|---|---|
| `Option<T>` | değer olmayabilir | boş liste, bulunamayan anahtar |
| `Result<T, E>` | işlem başarısız olabilir, sebebi de var | ayrıştırma, dosya, ağ, doğrulama |

İkisi de sıradan enum:

```rust
enum Option<T>    { None, Some(T) }
enum Result<T, E> { Ok(T), Err(E) }
```

Aradaki fark tam olarak `E` kadar. Hatta hata tipi olarak `()` seçerseniz
`Result<T, ()>` ile `Option<T>` aynı bilgiyi taşır — ikisi de "ya değer var ya yok"
demektir. **Seçim ölçütü şu: başarısızlığın tek ve apaçık bir sebebi varsa `Option`,
birden fazla sebebi veya anlatılacak detayı varsa `Result`.**

```rust
fn find_user(id: u32) -> Option<User>              // ya var ya yok, açıklamaya gerek yok
fn parse_reading(line: &str) -> Result<Reading, ParseError>   // beş türlü bozuk olabilir
```

## `panic!` ne zaman doğru cevap

`panic!` "bu durumdan dönüş yok, programı durdur" demek. Üç meşru kullanımı var:

- **Programcı hatası** — olması imkânsız bir duruma düşülmüş (invariant ihlali)
- **Kurtarılamaz durum** — program zaten çalışamaz (ayar dosyası yok, bellek yok)
- **Test** — `assert!` ailesi kasten panikler

Kütüphane yazıyorsanız neredeyse hiçbir zaman panic etmeyin: kararı çağırana bırakın.
Uygulama yazıyorsanız `main` içinde panic etmek kabul edilebilir.

Panic üretmenin yolları: `panic!`, `unreachable!`, `todo!`, `unimplemented!`, `assert!`.
`todo!()` özellikle kullanışlıdır — derlenir, çağrılırsa panikler; iskelet yazarken idealdir.

## `unwrap` ve `expect` disiplini

> **`expect` mesajı, ihlal edilemeyeceğine inandığınız varsayımın belgesidir.**

`unwrap()` sadece "burada asla `Err` olamaz" diyebiliyorsanız yazılır. Diyemiyorsanız
`expect("neden olamaz")` yazın; altı ay sonra kendinize açıklama olur. Panik mesajı
"called `Result::unwrap()` on an `Err` value" yerine sizin cümleniz olur.

## Hata tipini `enum` yapmak

`Result<T, String>` çalışır ama çağıran hatayı **ayırt edemez** — elinde bir metin vardır,
ancak metin karşılaştırarak karar verebilir. Enum yaparsanız çağıran `match` ile davranış seçer:

```rust
enum TelemetryError {
    EmptyLine,
    MissingField(&'static str),
    NotANumber(String),
    OutOfRange { field: &'static str, value: f64 },
}
```

Asıl kazanç son iki varyantta: **hata veri taşıyabiliyor.** Hangi alan, hangi değer,
hangi satır — hepsi hatanın içinde gider. Bu, "loglara bakalım" ile "kullanıcıya ne
olduğunu söyleyelim" arasındaki fark.

## `Result` üzerinde sık kullanılan metotlar

| Metot | Ne yapar |
|---|---|
| `is_ok()` / `is_err()` | sadece sorar, değeri almaz |
| `unwrap()` | `Ok` ise değeri verir, `Err` ise **panikler** |
| `expect("...")` | aynısı, panik mesajını siz yazarsınız |
| `unwrap_or(varsayılan)` | `Err` ise varsayılanı döndürür |
| `unwrap_or_else(\|e\| ...)` | varsayılanı hesaplayarak üretir |
| `ok()` | `Result` → `Option`, **hatayı çöpe atar** |
| `map_err(\|e\| ...)` | hata tipini dönüştürür, `Ok` tarafına dokunmaz |

Ters yön de var — `Option`'dan `Result`'a geçerken eksik olan "neden" bilgisini
siz eklersiniz:

```rust
opt.ok_or(TelemetryError::EmptyLine)                  // hata değeri hazırsa
opt.ok_or_else(|| TelemetryError::NotANumber(s))      // hata üretmek pahalıysa
```

`ok_or_else` tembeldir: hata nesnesi sadece gerçekten gerektiğinde üretilir.
`format!` gibi tahsis yapan bir şey kullanıyorsanız bunu tercih edin.

## panic mekaniği

| | unwind (varsayılan) | abort |
|---|---|---|
| Stack | çözülür | çözülmez |
| `Drop` | çalışır | çalışmaz |
| İkili boyut | büyük | küçük |

`Cargo.toml` içinde `[profile.release] panic = "abort"` ile değiştirilir; gömülü
sistemlerde ve WebAssembly'de yaygındır.

`RUST_BACKTRACE=1 ./program` panik izini gösterir — hangi çağrı zincirinden gelindiğini
görürsünüz.
