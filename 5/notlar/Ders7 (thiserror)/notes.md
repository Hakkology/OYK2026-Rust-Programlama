# Gün 5 · Ek Not — `thiserror`

Ders 1-2'de kendi hata tipimizi elle yazdık: `enum`, `impl Display`, `impl Error`,
her hata kaynağı için bir `impl From`. Otuz satır. `thiserror` bunların hepsini
**derive ile** yazar.

## Kurulum

```bash
cargo add thiserror
```

Bu bir kütüphane crate'i; `rustc main.rs` ile derlenmez, Cargo projesi gerekir.

## Elle yazılan hâliyle karşılaştırma

| Elle yazdığımız | `thiserror` karşılığı |
|---|---|
| `impl fmt::Display for AppError { ... }` | `#[error("...")]` satırı |
| `impl From<io::Error> for AppError { ... }` | `#[from]` işareti |
| `impl std::error::Error for AppError {}` | `#[derive(Error)]` |

```rust
#[derive(Debug, Error)]
enum AppError {
    #[error("G/Ç hatası: {0}")]
    Io(#[from] std::io::Error),

    #[error("Ayrıştırma hatası: {0}")]
    Parse(#[from] ParseIntError),
}
```

`#[from]` en kritik olanı: `From` implementasyonunu yazdığı için `?` operatörü
otomatik dönüşüm yapabiliyor. Fonksiyonun gövdesi bu yüzden bu kadar sade:

```rust
fn read_and_parse_number(file_path: String) -> Result<i32, AppError> {
    let mut file = File::open(file_path)?;      // io::Error  -> AppError::Io
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;        // io::Error  -> AppError::Io
    let number = contents.trim().parse::<i32>()?;  // ParseIntError -> AppError::Parse
    Ok(number)
}
```

Üç `?`, iki farklı hata tipi, tek dönüş tipi. Ders 2'deki "hepsi aynı hata tipine
çözülmeli" kuralı burada `#[from]` sayesinde bedavaya geliyor.

## `#[error("...")]` içine `{0}` koymayı unutmayın

Varyantın taşıdığı değere `{0}` ile ulaşılır. Koymazsanız asıl sebep kaybolur:

```
#[error("Ayrıştırma hatası:")]        ->  "Ayrıştırma hatası:"
#[error("Ayrıştırma hatası: {0}")]    ->  "Ayrıştırma hatası: invalid digit found in string"
```

`{:?}` (Debug) her zaman varyantı gösterir, ama kullanıcıya gösterilen `{}` (Display)
sizin yazdığınız metindir.

## Çağıran hâlâ `match` yapabiliyor

Asıl kazanç bu: hata bir enum olduğu için çağıran davranış seçebilir.

```rust
match e {
    AppError::Io(_) => std::process::exit(1),
    AppError::Parse(_) => std::process::exit(2),
}
```

Programı üç durumda da çalıştırın:

```
dosya yok      -> Io(Os { code: 2, kind: NotFound, ... })        çıkış kodu 1
dosya bozuk    -> Parse(ParseIntError { kind: InvalidDigit })    çıkış kodu 2
dosya doğru    -> File contains number: 42                       çıkış kodu 0
```

Çıkış kodunu kabuktan `echo $?` ile görebilirsiniz — betikler bu kodu okur.

## Ne zaman `thiserror`

**Kütüphane yazıyorsanız.** Kullanıcınızın hataları ayırt edebilmesi gerekir; enum
verirsiniz, `thiserror` da onu yazmanın angaryasını alır. Uygulama yazıyorsanız
sıradaki nota bakın: `anyhow`.
