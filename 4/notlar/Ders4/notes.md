# Gün 4 · Ders 4 — Pattern Matching

## Exhaustiveness — derleyicinin en görünür süper gücü

`match` **bütün ihtimalleri** ele almak zorundadır. Biri eksikse kod derlenmez:

```
E0004: non-exhaustive patterns: `Isik::Sari` not covered
```

Asıl kazanç burada değil, ileride: enum'a yeni bir varyant eklediğinizde onu ele
almayan **tüm** `match`'ler derlenmez. Derleyici size yapılacaklar listesi çıkarır.
Unutmanız mümkün değildir.

Bir enum'a varyant ekleyip projeyi derlemek, "nereleri güncellemem gerek" sorusunun
en hızlı cevabıdır.

**Uyarı:** `_ => ...` yazarsanız bu korumayı kendi elinizle kapatırsınız. Yeni varyant
sessizce `_` dalına düşer. Sayılarda `_` şart, ama enum'larda genelde kötü fikirdir.

## Desen çeşitleri

| Desen | Örnek |
|---|---|
| değer | `1 => ...` |
| aralık | `1..=5 => ...` |
| çoklu | `'a' \| 'e' \| 'ı' => ...` |
| joker | `_ => ...` |
| koşullu (guard) | `n if n % 15 == 0 => ...` |
| bağlama (`@`) | `n @ 90..=100 => ...` (hem eşleşir hem değeri tutar) |
| tuple | `(0, y) => ...` |
| struct | `Kare { satir: 1, sutun } => ...` |
| enum | `Sekil::Cember { r } => ...` |

## Destructuring — deseni parçalayarak okumak

Bir enum varyantının içindeki veriye ancak desenle ulaşırsınız:

```rust
match sekil {
    Sekil::Cember { r } => 3.14 * r * r,
    Sekil::Ucgen(a, b, c) => ...,
    Sekil::Nokta => 0.0,
}
```

`r`, `a`, `b`, `c` burada tanımlanıyor. Yani `match` hem "hangisi" sorusunu cevaplıyor
hem de içindekini çıkarıyor. Bu ikisinin tek adımda olması dilin en rahat yanlarından biri.

Struct'ta da çalışır, hatta bazı alanları sabitleyip bazılarını yakalayabilirsiniz:

```rust
match kare {
    Kare { satir: 1, sutun } => println!("ilk sırada, {}. sütun", sutun),
    Kare { satir, .. }       => println!("{}. sıra", satir),
}
```

`..` "kalan alanlar umurumda değil" demek.

## Kısayollar — hangisi ne zaman

| Yapı | Ne zaman |
|---|---|
| `match` | birden çok dal var, exhaustiveness istiyorsunuz |
| `if let` | sadece tek bir dalla ilgileniyorsunuz |
| `let else` | eşleşmezse erken çık, gerisi düz aksın |
| `while let` | eşleştiği sürece dön |
| `matches!` | sadece "eşliyor mu", `bool` döner |

```rust
if let Some(ad) = kullanici { println!("{}", ad); }

let Some(ilk) = olcumler.first() else {
    println!("ölçüm yok");
    return;
};
// buradan sonra ilk düz bir &f64, girinti yok

while let Some(ust) = yigin.pop() { }

if matches!(isik, Isik::Kirmizi) { }
```

`let else`'in sağ tarafı **çıkmak zorundadır** (`return`, `break`, `continue`, `panic!`).
Faydası: "başarısızsa çık" durumunu tek satıra indirip asıl akışı girintiden kurtarır.

## Desende sahiplik

Gün 3'ün kuralları burada da aynen geçerli:

```rust
match &sahipli { Some(s) => ... }    // ödünç — sahipli hâlâ bizde
match sahipli  { Some(s) => ... }    // TAŞINDI — sahipli artık yok
```

`&` ile eşleştirdiğinizde bağlanan değişkenler de referans olur (`s: &String`).
Buna **match ergonomics** deniyor; eskiden `Some(ref s)` yazmak gerekiyordu, eski
kodlarda görürseniz şaşırmayın.

## `match` bir ifadedir

Değer üretir, dolayısıyla `let`'in sağına yazılabilir. Gün 1'deki `if` kuralının
aynısı geçerli: **tüm kolların tipi aynı olmalı.**

```rust
let mesaj = match puan {
    90..=100 => "mükemmel",
    50..=89  => "geçer",
    _        => "kaldı",
};
```
