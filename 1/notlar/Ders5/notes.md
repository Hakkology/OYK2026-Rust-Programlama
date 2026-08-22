# Gün 1 · Ders 5 — Fonksiyonlar ve Kontrol Akışı

## Neredeyse her şey bir ifade

Rust'ın en temel yapısal farkı bu. Blok, `if`, `match`, `loop` — hepsi **değer üretir**.

```rust
let sonuc = { let a = 10; let b = 20; a + b };
```

Kural: bloğun son satırında **noktalı virgül yoksa** o bloğun değeridir.
Noktalı virgül koyarsanız değer `()` olur.

`if` ve `match` için iki kural daha:
- Kolların tipi **aynı** olmalı → yoksa E0308
- Koşul **mutlaka** `bool` → C'deki "0 = false" kısayolu yok

## Sadece `loop` değer döndürür

`while` ve `for` döndüremez, `loop` döndürür:

```rust
let ilk = loop {
    sayac += 1;
    if sayac * sayac > 50 { break sayac; }
};
```

Sebep: derleyici `while`ın en az bir kez çalışacağını garanti edemez, dolayısıyla
değerin var olacağını bilemez. `loop` ancak `break` ile çıkar; değer garanti altında.

## C tarzı `for` yok

`for (i = 0; i < n; i++)` yazamazsınız. Her `for` bir **iterator** üzerinde döner.

- `1..5` → üst sınır hariç
- `1..=5` → üst sınır dahil

## Etiketli break

```rust
'dis: for i in 1..=5 {
    for j in 1..=5 {
        if i * j == 12 { break 'dis; }
    }
}
```

## `match` aralıklarla

```rust
match sayi {
    0 => "sifir",
    1..=5 => "kucuk",
    _ => "buyuk",
}
```

## Fonksiyon imzası sözleşmedir

Parametre tipleri **asla** çıkarılmaz, yazmak zorunlu. Dönüş tipi de öyle.
Gerekçe: imzaya bakınca içine bakmadan ne yaptığını anlayabilmelisiniz.

## Tamsayı bölmesi

`c * 9 / 5 + 32` tamsayı ile yazılırsa `9 / 5 = 1` olur. **Derlenir ama sessizce yanlış çalışır.**
`f64` ile yazarsanız zaten `9.0 / 5.0` yazmak gerekir.

Derleyicinin yakalayamadığı hatalardan biri; sıfıra bölme gibi.
