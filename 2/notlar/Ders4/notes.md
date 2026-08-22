# Gün 2 · Ders 4 — Borrowing Kuralları

## Tek kural

> Aynı anda **ya sınırsız sayıda okuyucu YA DA tek yazıcı.**

Aynı anda ikisi birden olmaz. Bütün `E0499` / `E0502` hataları bunun türevi.
Buna "aliasing XOR mutability" deniyor.

## Neden bu kural?

Veri yarışı üç koşul ister:
1. İki veya daha fazla erişim aynı veriye
2. En az biri **yazma**
3. Erişimler eşzamanlı ve senkronize değil

Rust 2. ve 3. koşulun aynı anda sağlanmasını **derleme zamanında** imkânsız kılıyor.

## NLL (Non-Lexical Lifetimes)

En çok şaşırtan kısım bu: ödünç, **kapsam sonunda değil son kullanımında** biter (2018).

```rust
// (a) derlenir              // (b) derlenmez
let r = &v[0];               let r = &v[0];
println!("{}", r);           v.push(9);
v.push(9);                   println!("{}", r);
```

Tek fark satır sırası.

## `&mut` Copy değildir

```rust
let birinci = &mut z;
let ikinci = birinci;    // taşındı, birinci artık yok
```

`&T` Copy'dir, `&mut T` değildir. Sebebi tek kural: iki `&mut` aynı anda var olamaz,
dolayısıyla kopyalanamaz. İlk bakışta kafa karıştırıyor ama "tek yazıcı" kuralının
doğrudan sonucu.

## Sarkan referans (dangling pointer) imkânsız

`fn f() -> &String` → `E0106: missing lifetime specifier`

Yerel bir değere referans döndüremezsiniz; sahipliği döndürürsünüz.

## Özet — üç hata kodu

- `E0382` — taşınmış değer kullanıldı
- `E0499` — aynı anda iki `&mut`
- `E0502` — `&` ile `&mut` bir arada

Üçü de aynı tek kuralın türevi. Ezberlenmesi gereken hata kodu değil, kaynağı.
