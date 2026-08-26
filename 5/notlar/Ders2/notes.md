# Gün 5 · Ders 2 — Hata Yayma: `?` ve `From`

## Sorun: elden ele taşıma

Her fonksiyonda `match` yazarsanız asıl iş hata kontrolünün altında kaybolur:

```rust
let sayi = match metin.parse::<i32>() {
    Ok(n) => n,
    Err(e) => return Err(e),
};
```

Üç satır, hiçbiri işin kendisi değil. `?` bunu tek karaktere indiriyor:

```rust
let sayi: i32 = metin.parse()?;
```

## `?` tam olarak ne yapıyor

Derleyicinin ürettiği açılım kabaca şu:

```rust
match ifade {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```

Yani **iki iş** birden:

1. **Erken dönüş** — hata varsa fonksiyondan hemen çıkar
2. **Tip dönüşümü** — `From::from` ile hatayı fonksiyonun hata tipine çevirir

İkinci madde çoğu kişinin gözünden kaçar ve `?`'in asıl gücü odur.

`?` bir **postfix** operatördür; ifadenin ortasına da girebilir:

```rust
Ok(Reading::new(parse_field(a)?, parse_field(b)?))
```

## `?` `Option` üzerinde de çalışır

`None` ise erken döner:

```rust
fn kullanici_adi(eposta: &str) -> Option<&str> {
    let at = eposta.find('@')?;
    eposta.get(0..at)
}
```

Ama **`Result` ile `Option` arasında geçiş yapmaz.** En çok takılınan yer burasıdır:

- `Option` → `Result`: `.ok_or(hata)?`
- `Result` → `Option`: `.ok()?`

## `From` — `?`'in arkasındaki mekanizma

`parse()` size `ParseFloatError` verir, fonksiyonunuz `TelemetryError` döndürüyor.
İkisi farklı tip; `?` normalde derlenmezdi. Çözüm, dönüşümü bir kez tanımlamak:

```rust
impl From<std::num::ParseFloatError> for TelemetryError {
    fn from(e: std::num::ParseFloatError) -> Self {
        TelemetryError::NotANumber(e.to_string())
    }
}
```

Bunu yazdığınız andan itibaren `?` her yerde otomatik çevirir. Tek satırda toplanan
bu kural, dosyanın geri kalanını temizler.

## Farklı hata tipleri bir araya nasıl gelir

Bir fonksiyonda birden çok `?` varsa **hepsi aynı hata tipine** çözülmek zorundadır.
İki yol var:

**1. `map_err` ile çağrı yerinde çevirmek** — imzalara dokunmaz, her çağrıda tekrar yazarsınız:

```rust
let t = sensor_read(id).map_err(TelemetryError::Sensor)?;
```

**2. `From` yazıp `?`'e bırakmak** — bir kez yazılır, her yerde çalışır. Tercih edilen budur.

## Üç strateji — hangi projede hangisi

| Yaklaşım | Ne zaman |
|---|---|
| Kendi enum'unuz + `From` | kütüphane yazıyorsanız; çağıran hataları ayırt etsin |
| `Box<dyn std::error::Error>` | uygulama, prototip; tip önemli değil, "bir şey ters gitti" yeter |
| `thiserror` / `anyhow` crate'leri | gerçek projede ikisinin de otomatiği |

`Box<dyn Error>` "herhangi bir hata" demektir: farklı tipteki hatalar aynı kutuya girer.
Bedeli, çağıranın artık `match` ile ayırt edememesi.

## `main` da `Result` döndürebilir

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let d = parse_temperature("sicaklik=-63.2")?;
    println!("{}", d);
    Ok(())
}
```

`Err` dönerse hata `Debug` biçiminde yazılır ve **çıkış kodu 1** olur. Küçük komut satırı
programlarında çok pratiktir; kabuk betikleri bu çıkış kodunu okur.

## `?`'in yapamadığı: bağlam eklemek

`?` hatayı olduğu gibi yukarı taşır. "Hangi satırda oldu, hangi dosyayı okurken oldu"
bilgisi eklenmez — siz eklemezseniz kaybolur:

```rust
// bağlamı hatanın kendisine koyun
Err(TelemetryError::AtLine { line_no, source: e })
```

`anyhow` crate'i bunu `.context("ayar dosyası okunamadı")` ile yapar ve hata zinciri
üretir:

```
Error: ayar dosyası okunamadı
Caused by: No such file or directory (os error 2)
```

## `thiserror` ve `anyhow` (crate gerektirir)

- **Kütüphane** → `thiserror`: `#[from]` sizin yerinize `From`'u, `#[error("...")]`
  `Display`'i yazar. Bugün elle yazdığınız otuz satır beş satıra iner.
- **Uygulama** → `anyhow`: tek bir `anyhow::Error` tipi, `.context()` ile bağlam.

```rust
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("veritabanı hatası: {0}")]
    Db(#[from] DbError),
    #[error("dosya hatası: {0}")]
    Io(#[from] std::io::Error),
}
```

Kural: **kütüphanede `thiserror`, uygulamada `anyhow`.** Kütüphane kullanıcısı hataları
ayırt edebilmeli; uygulama sadece raporlayıp çıkacak.
