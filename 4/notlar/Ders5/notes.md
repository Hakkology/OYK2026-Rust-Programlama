# Gün 4 · Ders 5 — Döngüler ve `iter` Üçlüsü

## `for` her zaman bir iterator üzerinde döner

C tarzı `for (i = 0; i < n; i++)` yok. `for` bir **iterator** ister; aralık da
(`0..5`), koleksiyon da iterator verebilir.

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

```rust
for i in 0..3      { }   // aralık
for x in &v        { }   // vektör
for (k, d) in &map { }   // harita
```

## Üç biçim, tek fark: sahiplik

```rust
for x in &v      { }   // x: &T      — okur
for x in &mut v  { }   // x: &mut T  — değiştirir
for x in v       { }   // x: T       — TÜKETİR
```

Bunlar üç metodun kısayolu:

| Yazım | Metot | Verir | Döngüden sonra koleksiyon |
|---|---|---|---|
| `for x in &v` | `v.iter()` | `&T` | elinizde |
| `for x in &mut v` | `v.iter_mut()` | `&mut T` | elinizde, değişmiş |
| `for x in v` | `v.into_iter()` | `T` | **yok, taşındı** |

Üçüncüsü en sık yapılan hata:

```rust
for x in v { }
println!("{:?}", v);   // E0382: v taşındı
```

Düzeltmesi tek karakter: `&v`.

## `&T` ile çalışmak

`for x in &v` size `&T` verir, `T` değil:

```rust
for x in &v {
    if *x > 10 { }      // aritmetik ve karşılaştırmada * gerekebilir
    toplam += x;        // burada gerekmiyor, otomatik çözülüyor
}
```

`for x in &mut v` ise değiştirmek için `*` **şart**:

```rust
for x in &mut v {
    *x *= 2;
}
```

## `enumerate` — indeks lazımsa

```rust
for (i, sehir) in sehirler.iter().enumerate() {
    println!("{}. {}", i + 1, sehir);
}
```

`enumerate()` her elemanı `(indeks, eleman)` çiftine çevirir. İndeks **0'dan** başlar;
1'den saymak istiyorsanız yazdırırken `i + 1` yaparsınız.

Alternatifi şu ve daha kötüsü:

```rust
for i in 0..sehirler.len() {
    println!("{}", sehirler[i]);
}
```

Bu her adımda sınır kontrolü yapar, `i`'yi elle yönetmenizi ister ve bir yerde
`len()` ile karıştırınca panik üretir. `enumerate` bunların hiçbirini yaşatmaz.

`iter_mut` ile de çalışır:

```rust
for (i, x) in v.iter_mut().enumerate() {
    *x += i as i32;
}
```

## Metinde `enumerate`

```rust
for (i, k) in "gül".chars().enumerate()    { }   // 0,1,2      — kaçıncı harf
for (i, k) in "gül".char_indices()         { }   // 0,1,3      — kaçıncı BAYT
```

İkisi Türkçe metinde farklı sonuç verir: `enumerate` harfleri sayar, `char_indices`
bayt konumunu söyler. Dilim alacaksanız `char_indices`, "kaçıncı harf" diyecekseniz
`enumerate`.

## Ters çevirmek

```rust
for x in v.iter().rev()          { }
for (i, x) in v.iter().enumerate().rev() { }   // indeksler de tersten gelir
```

## Kendi tiplerinizde gezinme

Kural sayılarda neyse struct ve enum'larda da odur; sadece artık `*` ile uğraşmak
yerine alanlara ve varyantlara bakarsınız.

```rust
// okumak
for g in &gezegenler {
    println!("{} — {} uydu", g.name, g.moons);
}

// değiştirmek
for g in &mut gezegenler {
    g.moons += 1;            // alanı doğrudan değiştiriyoruz, * gerekmiyor
}

// tüketmek — alanları dışarı taşımak istiyorsanız
for g in gezegenler {
    adlar.push(g.name);       // String'i devraldık, kopya yok
}
```

Üçüncüsü `String` alanı çekip almak için tek yol: ödünçten `String` **taşıyamazsınız**
(`E0507`), o yüzden ya `clone()` yaparsınız ya listeyi tüketirsiniz.

Enum listelerinde de aynı: döngü ile `match` yan yana çalışır.

```rust
for s in &sekiller {
    match s {
        Shape::Circle { r } => ...,
        ...
    }
}
```

## Diğer koleksiyonlarda

```rust
for (anahtar, deger) in &harita     { }   // &K, &V
for deger in harita.values_mut()    { *deger += 1; }
for k in kume.iter()                { }
```

Kural her yerde aynı: `&` okur, `&mut` değiştirir, çıplak isim tüketir.
