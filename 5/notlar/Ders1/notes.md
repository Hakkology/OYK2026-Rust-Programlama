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

**`unwrap` tek yol değil, hatta son çare.** Tercih sırası şudur:

| Önce şunu deneyin | Ne zaman |
|---|---|
| `match` / `if let` | iki durumu da anlamlı biçimde ele alabiliyorsanız |
| `unwrap_or(x)` | hata olursa kullanılacak bir varsayılan varsa |
| `unwrap_or_else(\|e\| ...)` | varsayılan pahalıysa ya da hataya bakıp karar verecekseniz |
| `expect("...")` | gerçekten devam edilemezse — ama sebebini yazarak |
| `unwrap()` | en sonda; sebebi yazmaya bile değmiyorsa |

Öğrenirken `unwrap` kolay geldiği için refleks hâline gelir; sonra üretim kodunda
gece yarısı panik olarak geri döner. `main.rs`'te altı yol da yan yana duruyor,
ikisi bilerek yorumlu — yorumu açıp panik çıktısını görün.

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
| `unwrap_or_default()` | tipin sıfır değerini verir (`f64` için `0.0`) |
| `ok()` | `Result` → `Option`, **hatayı çöpe atar** |
| `map(\|v\| ...)` | **başarı** değerini dönüştürür, `Err` tarafına dokunmaz |
| `map_err(\|e\| ...)` | **hata** değerini dönüştürür, `Ok` tarafına dokunmaz |

### `map` ve `map_err` — kutunun içini değiştirmek

`Result` bir kutudur; iki gözü vardır. `map` **başarı gözünü**, `map_err` **hata gözünü**
dönüştürür; diğerine dokunmaz:

```rust
let r: Result<f64, TelemetryError> = parse_temperature("sicaklik=-63.2");

r.map(|c| c * 9.0 / 5.0 + 32.0)          // Ok(-63.2) -> Ok(-81.76), Err aynen geçer
 .map_err(|e| format!("{:?}", e))        // Err(...) -> Err("...") , Ok aynen geçer
```

Kutuyu açıp kapamadan içeriği değiştiriyorsunuz. `match` yazıp iki kolu da elle
doldurmaya gerek kalmıyor — çünkü zaten bir kolu olduğu gibi bırakacaktınız.

`Option`'da da aynı `map` var: `Some` ise dönüştürür, `None` ise `None` kalır.

### Bu `|x| ...` nedir?

`map(|c| c * 2.0)` içindeki `|c| c * 2.0` bir **closure**: adı olmayan küçük bir
fonksiyon. `|` işaretleri arasında parametreleri, sonrasında gövdesi yazılır.

```rust
|c| c * 2.0                    // bir parametre, tek ifade
|_| -999.0                     // parametreyi kullanmıyoruz: _
|e| format!("{:?}", e)         // gövdesi bir ifade
```

Şimdilik bu kadarı yeter: `map`, `map_err`, `unwrap_or_else` gibi metotlar "ne
yapacağını" bir closure ile söylersiniz.

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
