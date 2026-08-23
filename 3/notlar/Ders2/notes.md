# Gün 3 · Ders 2 — Borrowing Kuralları

## Tek kural

> Aynı anda **ya sınırsız sayıda okuyucu YA DA tek yazıcı.**

İkisi bir arada olmaz. Buna "aliasing XOR mutability" deniyor.
Gördüğünüz bütün ödünç hatası bunun bir türevi.

```rust
let r1 = &s;
let r2 = &s;
let r3 = &s;        // istediğiniz kadar okuyucu, sorun yok

let w = &mut s;     // tek yazıcı, yanında başka hiçbir ödünç olamaz
```

## Neden bu kural?

Veri yarışı üç koşul ister:

1. İki veya daha fazla erişim aynı veriye
2. En az biri **yazma**
3. Erişimler eşzamanlı ve senkronize değil

Rust 2. ve 3. koşulun aynı anda sağlanmasını **derleme zamanında** imkânsız kılıyor.
Tek iş parçacıklı kodda da aynı kural geçerli, çünkü aynı sorun orada da var:
elinizde bir referans dururken veri yer değiştirirse referans sarkta kalır.

## Ödünç almak için değişken `mut` olmalı

```rust
let v = vec![1, 2, 3];
v.push(4);          // E0596: cannot borrow as mutable
```

`push` imzasında `&mut self` var. `v` değişmez tanımlandığı için `&mut v` alınamıyor.
Çözüm `let mut v`.

## NLL — ödünç son kullanımında biter

Ödünç, **kapsam sonunda değil son kullanımında** biter (Non-Lexical Lifetimes, 2018).

```rust
// (a) derlenir              // (b) derlenmez — E0502
let r = &v[0];               let r = &v[0];
println!("{}", r);           v.push(9);
v.push(9);                   println!("{}", r);
```

Tek fark satır sırası. `(a)`'da `r`'nin işi `push`'tan önce bitiyor, ödünç kapanıyor.
`(b)`'de `push` anında hâlâ yaşayan bir okuyucu var.

Bu yüzden "hata alınca ödüncü daha erken bitir" çoğu durumda geçerli bir çözüm.

## Neden `push` bir okuyucuyu bozar?

`push` kapasite dolmuşsa heap'te **yeni bir yer alır ve veriyi taşır**. Eski adresi
gösteren referans o anda geçersiz olur. Kural keyfi değil, bunu engelliyor.
C++'ta aynı duruma **iterator invalidation** denir ve çalışma zamanında sessizce çöker.

Allocator bazen bloğu yerinde büyütür ve adres aynı kalır — yani her `push` taşımaz.
Ama **garantisi yok**, derleyicinin taşıyabileceğini varsayması yeterli.

## Döngü içinde değiştirme

En sık görülen hâli:

```rust
for x in &v {
    v.push(*x);     // E0502 — for zaten v'yi ödünç almış durumda
}
```

`for x in &v` döngü boyunca süren bir okuma ödüncü açar. Çözüm: değişikliği ayrı bir
`Vec`'te toplayıp döngüden sonra uygulamak.

## Bir elemanı ödünç almak tümünü kilitler

```rust
let e = &mut d[0];
*e = 10;
println!("{:?}", d);   // e hâlâ yaşıyorsa E0502
```

Derleyici "sadece 0. eleman" diye ayırt etmez; ödünç tüm koleksiyonu kapsar.
Aynı anda iki elemanı ayrı ayrı `&mut` almak gerekiyorsa `split_at_mut` gibi
std fonksiyonları diziyi ikiye bölerek bunu güvenli hâle getirir:

```rust
let (sol, sag) = d.split_at_mut(2);
sol[0] = 100;
sag[0] = 200;   // iki ayrı &mut, çakışmadıkları kanıtlı
```

## `&mut` Copy değildir

```rust
let birinci = &mut z;
let ikinci = birinci;    // taşındı, birinci artık yok
```

`&T` Copy'dir, `&mut T` değildir. Sebebi tek kural: iki `&mut` aynı anda var olamaz,
dolayısıyla kopyalanamaz.

Buna rağmen bir `&mut`'u fonksiyona birden fazla kez verebilirsiniz:

```rust
ekle(&mut s);
ekle(&mut s);
```

Burada her seferinde **yeniden ödünç** (reborrow) alınıyor: fonksiyon dönünce ödünç
kapanıyor, sıradaki çağrı temiz sayfayla başlıyor.

## Sarkan referans (dangling pointer) imkânsız

```rust
fn sarkan() -> &String {
    let s = String::from("yerel");
    &s              // E0106: missing lifetime specifier
}
```

`s` fonksiyon bitince düşer; döndürülen referans ölü belleği gösterirdi.
Rust bunu derlemez. Çözüm referansı değil **sahipliği** döndürmek: `-> String`.

Parametreden gelen bir referansı döndürmek serbesttir, çünkü veri çağıranda yaşıyor.

## Özet — hata kodları

| Kod | Ne oldu |
|---|---|
| `E0382` | taşınmış değer kullanıldı |
| `E0499` | aynı anda iki `&mut` |
| `E0502` | `&` ile `&mut` bir arada |
| `E0596` | `mut` olmayan değişkenden `&mut` alınmaya çalışıldı |
| `E0106` | referans döndürülüyor ama verinin kime ait olduğu belli değil |

Ezberlenmesi gereken hata kodu değil, kaynağı: her biri "ya okuyucular ya tek yazıcı"
kuralının farklı bir yüzü.
