# Gün 2 · Ders 3 — Fonksiyonlarda Sahiplik

## Dersin akışı

1. Parametreye geçen değer taşınıyor → çağıran kaybediyor
2. Çözüm denemesi 1: geri döndür → tek fonksiyonda tamam, zincirde dayanılmaz
3. Çözüm denemesi 2: tuple ile hem sonucu hem değeri döndür → çirkin
4. **Gerçek çözüm: ödünç al**

## `&` ve `*`

- `&x` → referans oluşturur, sahiplik geçmez
- `*r` → referansın gösterdiği değere iner
- `println!` ve karşılaştırmada otomatik çözülür, atama/aritmetikte elle gerekir
- `*n *= 2` → hedefi değiştirmek için `*` şart

## İmza bir sözleşmedir

```
fn f(s: String)       alır, geri vermez        — tüketir
fn f(s: &String)      okur                     — ödünç
fn f(s: &mut String)  okur ve değiştirir       — dışlayıcı ödünç
fn f() -> String      üretir, size verir       — sahiplik döner
```

Asıl mesaj: **imzaya bakarak içine bakmadan ne olacağını bilirsiniz.** Diğer dillerde
bu bilgi imzada yok, dokümantasyonda (varsa) yazar.

## `mut` parametresi ≠ `&mut` parametresi

```rust
fn yerel_degistir(mut s: String)   // sahipliği alır, yerel kopyası değiştirilebilir
fn ekle(s: &mut String)            // sahiplik almaz, çağıranın verisini değiştirir
```

İkisi sık karıştırılıyor; fark tamamen imzada görünüyor.

## `&String` mi `&str` mi?

Kural: **parametrede `&str`, `&String`'e tercih edilir.**
Sebebi: `&str` hem `String`'den hem sabit metinden gelebilir, `&String` sadece `String`'den.
