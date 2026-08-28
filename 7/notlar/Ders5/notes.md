# Gün 7 · Ders 5 — Closure'larla Çalışmak

Büro her dosya için farklı süzme kuralları kuruyor: kuralları saklıyor, zincirliyor,
çalışma zamanında seçiyor. Ders 4'te closure'ın ne olduğunu gördük; bugün onu **taşıyoruz**.

## Closure döndürmek

### `impl Fn` — tek somut tip

```rust
fn min_weight_rule(threshold: u8) -> impl Fn(&Lead) -> bool {
    move |l: &Lead| l.weight >= threshold
}
```

`move` şart: `threshold` fonksiyon bitince düşecek, closure onu sahiplenmeli.

### `Box<dyn Fn>` — çalışma zamanında seçim

```rust
fn rule_for(mode: &str) -> Box<dyn Fn(&Lead) -> bool> {
    match mode {
        "strict" => Box::new(|l: &Lead| l.weight >= 8),
        "loose"  => Box::new(|l: &Lead| l.weight >= 3),
        _        => Box::new(|_l: &Lead| true),
    }
}
```

```
strict  -> 2 ipucu
loose   -> 3 ipucu
off     -> 4 ipucu
```

Aynısını `impl Fn` ile yazamayız — **çevre yakalayan** iki closure iki ayrı tiptir:

```rust
fn rule_for_broken(mode: &str, threshold: u8) -> impl Fn(&Lead) -> bool {
    if mode == "strict" { move |l: &Lead| l.weight >= threshold }
    else { move |l: &Lead| l.weight >= threshold / 2 }
}
```

```
error[E0308]: `if` and `else` have incompatible types
              expected closure, found a different closure
```

Gün 6'da `Dragon`/`Archer` ile aynı duvara çarpmıştık; çözüm de aynı: `Box<dyn>`.

> İnce ayrıntı: **hiçbir şey yakalamayan** iki closure ile aynı kod **derlenir**. Sebebi
> Ders 4'ün son bölümü: yakalamayan closure `fn(&Lead) -> bool` pointer'ına dönüşür ve
> iki dal ortak bir tipte buluşur. Yakalamaya başladığı anda bu yol kapanır.

## Closure'ı struct içinde saklamak

### Generic alan — tek kural, sıfır maliyet

```rust
struct Screen<F> where F: Fn(&Lead) -> bool {
    name: String,
    rule: F,
}
```

`(self.rule)(l)` yazılışına dikkat: parantezler olmadan derleyici `rule` adında bir
**metot** arar.

### `Box<dyn Fn>` alan — farklı kurallar bir arada

```rust
struct RuleBook {
    rules: Vec<(String, Box<dyn Fn(&Lead) -> bool>)>,
}
```

Her closure ayrı tip olduğu için `Vec` ancak böyle kurulabilir — Gün 6'daki
`Vec<Box<dyn Unit>>` ile birebir aynı gerekçe.

```rust
book.add("agirlik >= 3", Box::new(|l| l.weight >= 3));
book.add("muhbir belli", Box::new(|l| l.informant != "bilinmiyor"));
```

```
2 kuraldan gecenler: ["otoparktaki bilet", "plaka kaydi"]
```

Karar aynı: **tek tip yeterliyse generic, koleksiyon/çalışma zamanı seçimi gerekiyorsa
`Box<dyn>`.**

## Kombinatörler

Gün 4'te iterator'ları görmüştük; closure'ları oraya takıyoruz.

```rust
let notes: Vec<String> = leads
    .iter()
    .filter(|l| l.weight >= 5)
    .map(|l| format!("{} ({})", l.note, l.weight))
    .collect();
```

```
filter + map : ["otoparktaki bilet (8)", "plaka kaydi (9)"]
sum          : 22
max_by_key   : Some("plaka kaydi")
sort_by_key  : [9, 8, 3, 2]
any / all    : true / true
```

### Adaptör / tüketici — dersin can alıcı ayrımı

| | alır | döndürür |
|---|---|---|
| **adaptör** (`map`, `filter`, `take`) | iterator | iterator — **tembel**, hiçbir şey yapmaz |
| **tüketici** (`collect`, `sum`, `count`, `fold`) | iterator | değer — **zinciri başlatır** |

Kanıtı `main.rs`'te canlı: `map`'in içine `println!` koyduk.

```
zincir kuruldu - yukarida hic satir yok, degil mi?
    >> otoparktaki bilet isleniyor
    >> isimsiz telefon isleniyor
    >> plaka kaydi isleniyor
    >> dedikodu isleniyor
sum() cagrildi -> toplam 22
```

Zincir kurulurken **tek satır** çalışmadı; `sum()` çağrılınca hepsi birden aktı.

Tüketmezseniz derleyici uyarır:

```
warning: unused `Map` that must be used
note: iterators are lazy and do nothing unless consumed
```

Bu tembelliğin bedeli yok, faydası büyük: ara adımlarda geçici `Vec`'ler oluşmaz,
derleyici zinciri tek bir döngüye açar (loop fusion). Gün 1'de "zero-cost abstraction"
dedik, Gün 6'da monomorphization ile ödedik; bu ikinci kanıt.

`collect` hangi koleksiyona toplayacağını **tipten** anlar; belirsiz kalırsa tipi
yazarsınız (`collect::<Vec<_>>()`).

### `fold` — en genel tüketici

`sum`, `product`, `count`, `max`… hepsi `fold`'un özel hâli:

```rust
leads.iter().fold(0, |acc, l| acc + l.weight as u32)   // = .map(...).sum()
```

```
fold 22 == sum 22
```

`fold` başlangıç değeri alır. `reduce` almaz — bu yüzden boş iterator ihtimaline karşı
`Option` döndürür:

```rust
.reduce(|a, b| if a.len() >= b.len() { a } else { b })   // Some("otoparktaki bilet")
```

## `Option` üzerinde kombinatörler

`Option` da bu dünyanın parçası — Gün 4'te `match` ile açıyorduk, artık zincirleyebiliriz:

| metot | ne yapar |
|---|---|
| `map` | `Some`'un **içini** dönüştürür, `None`'a dokunmaz |
| `and_then` | `Option` döndüren bir işlemi zincirler (iç içe `Option` oluşmaz) |
| `filter` | `Some`'u koşula sokar, geçmezse `None` |
| `unwrap_or_else` | closure **yalnızca** `None` ise çalışır |
| `ok_or` | `Option` → `Result` (Gün 5) |

```
map          : Some(9)
unwrap_or_else: 9
and_then     : Some("plaka")
filter       : None
ok_or        : Ok("plaka kaydi")
```

`unwrap_or_else`'in değerli yanı: pahalı bir varsayılan hesaplaması ancak gerekirse
çalışır. `unwrap_or(pahali_hesap())` yazarsanız hesap **her zaman** yapılır.

## Kombinatör mü, `?` mi

Aynı iş iki türlü yazılabilir:

```rust
fn weight_sum_chained(a: &str, b: &str) -> Option<u32> {
    a.parse::<u32>().ok().and_then(|x| b.parse::<u32>().ok().map(|y| x + y))
}

fn weight_sum_question(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x: u32 = a.parse()?;
    let y: u32 = b.parse()?;
    Ok(x + y)
}
```

```
kombinator: Some(17)    / None
? ile     : Ok(17)      / Err(...)
```

Gün 5'te `?`'in `From` üzerinden çalıştığını görmüştük; ikisi aynı işi yapıyor.
**Seçim ölçütü:** zincir kısa ve tek hatlıysa kombinatör; adım sayısı artıyor ya da
araya `if`/`match` giriyorsa `?` daha okunur. Kombinatör zincirini üç adımdan uzun
tutmayın.
