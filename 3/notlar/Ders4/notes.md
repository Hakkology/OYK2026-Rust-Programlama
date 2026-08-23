# Gün 3 · Ders 4 — HashMap

## Ne işe yarar

`Vec` elemanlara **konumla** erişir: `v[3]`. `HashMap<K, V>` **anahtarla** erişir:
`map["ankara"]`. Arama, ekleme ve silme ortalama O(1) — listede arama O(n)'di.

Bedeli: sıra yoktur ve anahtarın hash'lenebilmesi gerekir.

Hangi koleksiyon ne zaman:

| | Ne zaman | Maliyet |
|---|---|---|
| `Vec` | sıra önemli, konumla erişim | O(1) indeks, O(n) arama |
| `HashMap` | sıra önemsiz, hızlı arama | ortalama O(1) |
| `BTreeMap` | sıralı gezme, aralık sorgusu | O(log n) |
| `HashSet` | tekrarsız üyelik | ortalama O(1) |

`std`'nin ön yüklü (prelude) tipi değildir, elle almak gerekir:

```rust
use std::collections::HashMap;
```

## Oluşturma

```rust
let mut plaka: HashMap<String, u32> = HashMap::new();

let sabit = HashMap::from([
    ("Ankara", 6),
    ("İstanbul", 34),
]);

let ciftler = vec![("Ankara", 6), ("İzmir", 35)];
let toplanan: HashMap<&str, u32> = ciftler.into_iter().collect();
```

Tip belirtmek çoğu zaman şart, çünkü `collect()` neye toplayacağını kendi bilemez.

## Temel metotlar

| Metot | Ne yapar |
|---|---|
| `insert(k, v)` | ekler; anahtar **varsa üzerine yazar** ve eski değeri `Option<V>` olarak döndürür |
| `get(&k)` | `Option<&V>` — yoksa `None` |
| `get_mut(&k)` | `Option<&mut V>` — değeri yerinde değiştirmek için |
| `map[&k]` | doğrudan değer, ama anahtar **yoksa panikler** |
| `contains_key(&k)` | var mı |
| `remove(&k)` | siler ve `Option<V>` döndürür |
| `len()` / `is_empty()` / `clear()` | boyut / boş mu / boşalt |
| `keys()` / `values()` / `values_mut()` | anahtarlar / değerler / değiştirilebilir değerler |
| `entry(k)` | "varsa bul, yoksa yarat" — aşağıda |

`get` neden `Option` döndürüyor? Çünkü anahtarın olmaması normal bir durum, hata değil.
Rust bunu tipin içine yazıp ele almanızı zorunlu kılıyor:

```rust
match plaka.get("Ankara") {
    Some(k) => println!("{}", k),
    None    => println!("kayıt yok"),
}

let k = plaka.get("Yok").copied().unwrap_or(0);   // varsayılan ver
```

## `entry` — en çok kullanacağınız metot

"Varsa üzerinde çalış, yoksa oluştur" kalıbı:

```rust
let sayac = harita.entry(kelime).or_insert(0);
*sayac += 1;
```

`or_insert` bir `&mut V` döndürür; `*` ile hedefe inip değiştirirsiniz.
Bunu `contains_key` + `get` + `insert` üçlüsüyle de yazabilirsiniz; o zaman aynı
anahtarın hash'i **üç kez** hesaplanır. `entry` bir kez hesaplar.

Yakınları: `or_insert_with(|| pahali_hesap())` (değer sadece gerekirse üretilir),
`and_modify(|v| *v += 1).or_insert(1)`.

## Gezinme — sıra yoktur

```rust
for (anahtar, deger) in &harita { }
for anahtar in harita.keys() { }
for deger in harita.values_mut() { *deger += 1; }
```

Çıktı sırası **her çalıştırmada değişebilir**. Programı iki kez çalıştırıp görün.

Sebebi bilinçli bir güvenlik tercihi: Rust hash fonksiyonunu her program başlangıcında
rastgele tohumluyor. Tohum sabit olsaydı, saldırgan hepsi aynı kovaya düşen anahtarlar
üretip aramayı O(1)'den O(n)'e düşürebilirdi (**HashDoS**). 2011'de PHP, Python ve
Java bu şekilde vurulmuştu.

Sıralı çıktı istiyorsanız iki yol var:

```rust
let mut anahtarlar: Vec<_> = harita.keys().collect();
anahtarlar.sort();
```

ya da baştan `BTreeMap` kullanmak — o anahtarı sıralı tutar, karşılığında
erişim O(1) değil O(log n) olur.

## Sahiplik

- `insert(k, v)` anahtarı ve değeri **taşır**. `String` verdiyseniz artık haritanındır.
- Copy tipler (sayılar, `&str`) kopyalanır.
- `get(&k)` **ödünç** verir; dönen `&V` yaşadığı sürece haritayı değiştiremezsiniz —
  Ders 2'deki tek kural burada da geçerli.
- Değeri haritadan çıkarmak isterseniz `remove` sahipliği geri verir.

`&str` anahtar kullanmak yaygındır ve ucuzdur, ama anahtarın işaret ettiği metin
haritadan **daha uzun yaşamalı**. Metin haritanın içinde üretiliyorsa `String` anahtar
kullanın.

## Anahtar ne olabilir?

Anahtar tipi `Eq + Hash` uygulamalı: tüm tamsayılar, `bool`, `char`, `String`, `&str`
ve bunlardan oluşan tuple'lar. `f64` **anahtar olamaz** — `NaN != NaN` olduğu için
eşitlik tam tanımlı değil.

## Kapanış: `HashSet`

`HashSet<T>` değersiz `HashMap`'tir: sadece "bu eleman var mı" sorusuna cevap verir.

```rust
use std::collections::HashSet;

let mut k = HashSet::new();
k.insert("elma");
k.insert("elma");        // ikinci ekleme false döner, küme tekrar tutmaz
k.contains("elma");      // true
```

Tekrarları ayıklamak, "gördüm mü" kontrolü ve kesişim/birleşim işleri için kullanılır.
