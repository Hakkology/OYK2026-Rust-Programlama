# Gün 3 · Ders 3 — Vec Metotları ve Metin Tipleri

## Bölüm 1 — `Vec` metotları (hızlı geçiş)

`Vec<T>` büyüyebilen liste. Stack'te üç kelime tutar (ptr / len / cap), veri heap'te.
Elemanların hepsi **aynı tipte**.

```rust
let mut v = vec![3, 1, 2];
```

| Metot | Ne yapar |
|---|---|
| `Vec::new()` | boş vektör, kapasite 0 |
| `vec![1, 2, 3]` / `vec![0; 5]` | dolu vektör / 5 tane 0 |
| `Vec::with_capacity(n)` | baştan yer ayırır, gereksiz taşımayı önler |
| `push(x)` | sona ekler |
| `pop()` | sondan alır, `Option<T>` döner (boşsa `None`) |
| `insert(i, x)` | araya ekler, sonrakileri kaydırır — O(n) |
| `remove(i)` | çıkarır ve döndürür, sonrakileri kaydırır — O(n) |
| `swap_remove(i)` | sondakiyle yer değiştirip çıkarır — O(1), sıra bozulur |
| `len()` | eleman sayısı |
| `is_empty()` | boş mu — `len() == 0` yerine bunu yazın |
| `clear()` | hepsini siler, kapasite kalır |
| `truncate(n)` | ilk n eleman kalır |
| `contains(&x)` | içinde var mı |
| `get(i)` | `Option<&T>` — sınır dışında `None` |
| `v[i]` | doğrudan erişim — sınır dışında **panikler** |
| `first()` / `last()` | `Option<&T>` |
| `sort()` | küçükten büyüğe sıralar |
| `sort_by(\|a, b\| b.cmp(a))` | kendi ölçütünüzle sıralar |
| `reverse()` | ters çevirir |
| `swap(i, j)` | iki elemanı takas eder |
| `retain(\|x\| kosul)` | koşulu sağlamayanları atar |
| `dedup()` | **yan yana** tekrarları siler (önce `sort()` gerekir) |
| `extend(digeri)` | başka bir koleksiyonu sona ekler |
| `capacity()` / `reserve(n)` | mevcut kapasite / yer ayır |

### Gezinme

```rust
for x in &v      { }   // okur
for x in &mut v  { }   // değiştirir  (*x = ...)
for x in v       { }   // TÜKETİR — v bundan sonra yok
```

Fark tamamen sahiplikte. Üçünün karşılığı olan `iter()` / `iter_mut()` /
`into_iter()` metotları Ders 5'in konusu.

### Maliyetler

| İşlem | Maliyet |
|---|---|
| `v[i]` — konumla erişim | O(1) |
| `contains` — arama | O(n) |
| `push` — sona ekleme | amortize O(1) |
| `insert` / `remove` — araya | O(n) |
| `swap_remove` | O(1), sıra bozulur |

Kapasite dolunca `Vec` **yeni bir blok alır, veriyi taşır, eskisini bırakır** ve
kapasiteyi ikiye katlar. Tek bir `push` bazen O(n) olur ama n push'un ortalaması
O(1)'dir — buna amortize O(1) denir. Kaç eleman geleceğini biliyorsanız
`with_capacity(n)` bu taşımaların hepsini önler.

### `Vec`'ten taşıma yasak — E0507

```rust
let v = vec![String::from("a")];
let ilk = v[0];      // E0507: cannot move out of index
```

Taşımaya izin verilseydi `v[0]` boş kalırdı ama `v.len()` hâlâ 1 derdi — tutarsız
durum. Üç çözüm: `&v[0]` (ödünç al), `v[0].clone()` (kopyala), `v.remove(0)`
(gerçekten çıkar). Sayılarda bu sorun yok, çünkü onlar Copy.

### Dilim — `&[T]`

`&v[1..4]` bir **dilimdir**: sahiplik taşımaz, sadece pencere açar.

```rust
&v[..]      // tamamı
&v[2..]     // 2'den sona
&v[..2]     // baştan 2'ye (2 hariç)
&v[1..=3]   // 1'den 3'e (3 dahil)
```

Dilim de bir ödünçtür; yaşadığı sürece `Vec`'i değiştiremezsiniz.

Sıralama sayılarda doğrudan çalışır. `f64`'te `sort()` yoktur (`NaN` yüzünden tam sıra
tanımlı değil), `sort_by(|a, b| a.partial_cmp(b).unwrap())` kullanılır.

## Bölüm 2 — `String` ve `&str`

Rust'ta iki metin tipi var, çünkü iki farklı soruya cevap veriyorlar:

| | `String` | `&str` |
|---|---|---|
| Ne | veriyi **sahiplenir** | başkasının verisine **bakar** |
| Nerede | heap, büyüyebilir | ödünç pencere |
| Stack'te | 24 bayt (ptr + len + cap) | 16 bayt (ptr + len) |
| Değiştirilebilir mi | evet (`mut` ile) | hayır |

Sabit metin (`"merhaba"`) çalıştırılabilir dosyanın içine gömülüdür ve tipi `&str`'dir.
Program boyunca yaşar, kimse onu düşürmez.

### Dönüşümler

```rust
let a: &str   = "merhaba";
let b: String = a.to_string();      // veya String::from(a)
let c: &str   = &b;                 // veya b.as_str() / &b[..]
```

`String` üretmenin beş yolu aynı kapıya çıkar:

```rust
String::new()          // boş
String::from("abc")
"abc".to_string()
"abc".to_owned()
format!("{}-{}", "abc", 1)
```

Büyütmek için `push` (tek karakter), `push_str` (metin), `+=` (metin).

`&String` verilen yerde `&str` beklenen bir fonksiyon çalışır (deref coercion).
Tersi olmaz: `&str`'yi `String` isteyen yere veremezsiniz, önce `to_string()` gerekir.

### UTF-8 gerçeği

`String` bir **bayt dizisidir**, harf dizisi değil. Bir harf 1-4 bayt tutar.

```rust
let t = String::from("şğü");
t.len()             // 6  — bayt sayısı
t.chars().count()   // 3  — karakter sayısı
```

Bunun üç sonucu var:

**1. İndeksleme yok.** `t[0]` derlenmez (E0277). "Bir bayt bir harf" olmadığı için
Rust bu işlemi yasaklamış — sessizce yarım harf döndürmektense hiç izin vermiyor.

**2. Dilim bayt cinsindendir ve harf sınırında olmalı.** `&t[0..2]` → `"ş"` (iki bayt).
`&t[0..1]` yarım harf demektir ve **çalışma zamanında panikler**. Emin değilseniz
`t.get(0..1)` kullanın, `Option` döner.

**3. Karakter üzerinde gezmek için `chars()` gerekir:**

```rust
for k in t.chars() { }                 // harfler
for (i, k) in t.char_indices() { }     // bayt konumu + harf
for b in t.bytes() { }                 // ham baytlar
```

### Sık kullanılan metotlar

| Metot | Ne yapar |
|---|---|
| `push_str("...")` / `push('a')` | sona metin / tek karakter ekler |
| `insert_str(i, "...")` | bayt konumuna ekler |
| `pop()` | son karakteri alır, `Option<char>` |
| `len()` / `is_empty()` | bayt sayısı / boş mu |
| `clear()` | boşaltır |
| `trim()` / `trim_start()` / `trim_end()` | baştaki-sondaki boşlukları atar |
| `to_uppercase()` / `to_lowercase()` | yeni `String` üretir, aslını bozmaz |
| `contains("x")` | içeriyor mu |
| `starts_with` / `ends_with` | başlıyor mu / bitiyor mu |
| `find("x")` | ilk konum, `Option<usize>` (bayt konumu) |
| `replace("a", "b")` | yeni `String` döner |
| `split(' ')` | ayırır — **tembel**, `collect()` etmeden liste olmaz |
| `split_whitespace()` | ardışık boşlukları tek sayar |
| `lines()` | satırlara böler |
| `repeat(n)` | n kez tekrarlar |
| `parse::<i32>()` | sayıya çevirir, `Result` döner |
| `chars().rev()` | tersten gezer |

### `split(' ')` ile `split_whitespace()` farkı

```rust
"a  b".split(' ')            // ["a", "", "b"]  — arada boş parça
"a  b".split_whitespace()    // ["a", "b"]      — yığılmayı ve sekmeyi halleder
```

Kelime ayırırken neredeyse her zaman istediğiniz `split_whitespace()`.

### n. karakteri almak O(n)

```rust
s.chars().nth(1)     // baştan taramak zorunda
```

`Vec`'te `v[1]` doğrudan adres hesabıyken metinde böyle bir kısayol yok; harflerin
boyutu değişken olduğu için baştan saymak gerekiyor.

### Metnin bir parçasını döndürmek

```rust
fn ilk_kelime(s: &str) -> &str {
    match s.find(' ') {
        Some(k) => &s[..k],
        None => s,
    }
}
```

Dönen değer yeni bir metin değil, girdinin **bir dilimi**. Kopya yok; fonksiyon
sadece "şurasından şuraya bak" diyor.

### Birleştirme ve sahiplik

```rust
let s3 = s1 + &s2;      // s1 TAŞINIR, s2 ödünç verilir
```

`+` operatörü sol tarafın sahipliğini alır — `s1` bundan sonra kullanılamaz. Sebebi
sol taraftaki tamponu yeniden kullanıp gereksiz kopya yapmamak.

Karıştırmamak için pratikte `format!` tercih edilir; hiçbirini tüketmez:

```rust
let s3 = format!("{} {}", s1, s2);
```

Liste birleştirmede `join`: `["a", "b"].join("-")` → `"a-b"`.

### Fonksiyon imzasında hangisi?

- Parametre **okuyorsa** → `&str`. Hem `String`'i hem sabit metni kabul eder.
- Parametre **değiştirecekse** → `&mut String`.
- Fonksiyon yeni metin **üretiyorsa** → `String` döndürür.

### Türkçe tuzakları

- `"İstanbul".len()` → 9, `.chars().count()` → 8
- `'i'.to_uppercase()` → `"I"`, Türkçe'de `İ` olmalı
- `'I'.to_lowercase()` → `"i"`, Türkçe'de `ı` olmalı
- `"İ".to_lowercase()` → **iki kod noktası** (`i` + birleşen nokta). Yani küçük harfe
  çevrilmiş metin orijinalden **uzun** olabilir; uzunluk karşılaştırmasına güvenmeyin.
- Unicode'da büyük/küçük harf dönüşümü dile bağlı, `std` dil bilmiyor. Doğrusu için
  dil bilen (ICU tabanlı) bir crate gerekir.
- Türkçe metni bayt bayt kesmeye çalışan her kod er geç panikler; `chars()` kullanın.

Bu sadece bizim derdimiz değil: "Turkish-I bug" diye aranır. 2000'lerde .NET ve
Java'da `"INDEX".toLowerCase()` Türkçe yerelde `"ındex"` verdiği için veritabanı
sorguları bozuluyordu.
