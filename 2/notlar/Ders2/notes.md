# Gün 2 · Ders 2 — Ownership

## Üç kural

1. Her değerin bir **sahibi** vardır
2. Aynı anda **tek** sahip olabilir
3. Sahip kapsam dışına çıkınca değer **düşer**

## Move sığ kopyadır

```rust
let s1 = String::from("merhaba");
let s2 = s1;
```

Stack'teki üçlü (ptr/len/cap) `s2`'ye kopyalanır, **heap verisi kopyalanmaz**.
Sonra `s1` geçersiz işaretlenir.

Neden geçersiz? İkisi de aynı heap bloğunu gösterseydi, ikisi de düşerken
**double free** olurdu. Rust bunu kopyalayarak değil, birini iptal ederek çözüyor.

`as_ptr()` çıktısı bunu doğruluyor: `s2` aynı adresi gösteriyor.

## Copy vs Clone

| | Copy | Clone |
|---|---|---|
| Ne zaman | otomatik, örtük | `.clone()` yazınca |
| Maliyet | bedava (stack) | pahalı (heap kopyası) |
| Hangi tipler | tamamı stack'te olanlar | isteyen herkes |

Copy olanlar: tüm sayılar, `bool`, `char`, `&T`, ve **tüm alanları Copy olan** tuple/array.
Copy olmayanlar: `String`, `Vec`, `Box` — heap tutan hiçbir şey.

Önemli nokta: **`clone()` yazmak zorunda olmanız bir bug değil, bir fatura.** Rust size
"bu işlem pahalı, emin misin" diye soruyor. C++'ta kopya constructor'ı sessizce çalışır.

## Kısmi move

```rust
let k = (String::from("a"), String::from("b"));
let ilk = k.0;          // sadece k.0 taşındı
println!("{}", k.1);    // çalışır
println!("{:?}", k);    // E0382 — bütünü artık yok
```

Neden `k.1` çalışıyor da `k` çalışmıyor? Taşınan sadece `k.0`; `k.1` hâlâ yerinde
ve tek başına kullanılabilir. Ama tuple'ın bütünü artık geçerli bir değer değil.

## Drop ve move ilişkisi

`tasi_ve_dusur(i1)` çağrısından **sonra** değil, fonksiyon içinde — dönmeden önce — düşer.
Çıktıdaki sıra bunu gösteriyor: sahip kimse, düşüren de o.

`drop(x)` erken düşürür. Bu bir dil özelliği değil, sadece sahipliği alıp hiçbir şey
yapmayan bir fonksiyon: `fn drop<T>(_: T) {}`.
