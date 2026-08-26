# Gün 5 · Ek Not — `anyhow`

`thiserror` kütüphaneler içindi: çağıran hatayı ayırt edebilsin diye enum üretiyordu.
`anyhow` ise **uygulamalar** için: hangi hata olduğu değil, **ne olduğu ve nerede
olduğu** önemlidir.

## Kurulum

```bash
cargo add anyhow
```

## Tek tip, tek satır

```rust
use anyhow::{Context, Result};

fn read_and_parse_number(file_path: String) -> Result<i32> { ... }
```

`anyhow::Result<i32>` aslında `Result<i32, anyhow::Error>` demek. Hata tipi tek:
`anyhow::Error` her hatayı kabul eder. Kendi enum'unuzu yazmıyorsunuz, `From`
implementasyonu yazmıyorsunuz — `?` her tür hatayı içine alıyor.

## `?`'in yapamadığını `context` yapar

Ders 2'de şunu söylemiştik: `?` hatayı yukarı taşır ama **bağlam eklemez**. "Dosya
bulunamadı" der, hangi dosyayı hangi iş için açtığınızı söylemez. `anyhow` bunu çözer:

```rust
File::open(file_path).context("Failed to read file contents")?;

contents.trim().parse::<i32>()
    .with_context(|| format!("Failed to parse integer from contents: {}", contents.trim()))?;
```

- `context("...")` — sabit metin
- `with_context(|| ...)` — **tembel**: metin ancak hata olursa üretilir. `format!` gibi
  tahsis yapan bir şey kullanıyorsanız bunu seçin.

## Çıktı: hata zinciri

`{:?}` ile bastığınızda sebep zinciri görünür:

```
dosya yok:
Cannot process: Failed to read file contents

Caused by:
    No such file or directory (os error 2)

dosya bozuk:
Cannot process: Failed to parse integer from contents: abc

Caused by:
    invalid digit found in string
```

Üstteki satır **sizin** cümleniz, alttaki işletim sisteminin/std'nin cümlesi. Kullanıcı
üsttekini okur, siz alttakini. Elle yazılan hata tiplerinde bu zinciri kurmak için
`source()` implementasyonu gerekirdi.

## `thiserror` mı `anyhow` mı

| | Ne zaman | Neden |
|---|---|---|
| `thiserror` | **kütüphane** | çağıran `match` ile davranış seçebilsin |
| `anyhow` | **uygulama** | tipin önemi yok, raporlanacak; bağlam önemli |

Kural cümlesi: **kütüphanede `thiserror`, uygulamada `anyhow`.**

İkisi birlikte de kullanılır: kütüphane katmanı `thiserror` ile kendi enum'unu verir,
uygulama katmanı onu `anyhow` ile yakalayıp bağlam ekleyerek raporlar.

## `main` da `anyhow::Result` döndürebilir

```rust
fn main() -> anyhow::Result<()> {
    let n = read_and_parse_number(String::from("number.txt"))?;
    println!("{}", n);
    Ok(())
}
```

Hata olursa zincir ekrana basılır ve çıkış kodu 1 olur — küçük komut satırı
programlarında en pratik kullanım budur.
