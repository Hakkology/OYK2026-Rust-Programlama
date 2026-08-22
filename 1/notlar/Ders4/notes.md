# Gün 1 · Ders 4 — Değişkenler ve Veri Tipleri

## Varsayılan değişmezlik

`let` değişmez, `mut` istisna. Diğer dillerin çoğunda tersi.
Bu bir kısıtlama değil, bir **belge**: `mut` yoksa o değer değişmeyecek demektir.
Kod okurken bilgi kazandırıyor.

## Shadowing

`mut` ile karıştırılıyor. Fark:

| | mut | shadowing |
|---|---|---|
| Ne yapar | aynı değişkeni değiştirir | yeni değişken yaratır |
| Tip | sabit | **değişebilir** |

Asıl kullanım yeri girdi ayrıştırma:
```rust
let yas = "30";
let yas: u32 = yas.parse().unwrap();
```
`yas_metin` / `yas_sayi` diye iki isim uydurmaktan iyi.

## `usize`

İndeks ve uzunluk **her zaman** `usize`. Makinenin adres genişliği.

## `char` 4 bayt, `len()` bayt sayar

`char` bir Unicode kod noktası, daima 4 bayt.
Ama `String` içinde kapladığı yer değişken (UTF-8).
`"İstanbul".len()` → 9, `.chars().count()` → 8.

Türkçe bir metinle deneyince fark hemen görünüyor.

`'i'.to_uppercase()` → `"I"`, Türkçe'de `İ` olmalı. Rust'ın eksiği değil; Unicode'da
büyük harfe çevirme dile bağlı ve `std` dil bilmiyor.

## Dizi uzunluğu tipin parçası

`[i32; 5]` ile `[i32; 6]` **farklı tipler**. C'den gelenler için en şaşırtıcı nokta bu.
Uzunluğu değişebilen liste `Vec<T>`.

Sabit indeks sınır dışıysa derleyici yakalıyor. Değişken indekste `get()` → `Option`.

## `const` ve `static`

- `const` → değer kullanıldığı her yere gömülür, tip **zorunlu**
- `static` → bellekte tek adres, program boyu yaşar
- İkisi de derleme zamanında bilinmeli; fonksiyon çağrısı olamaz
