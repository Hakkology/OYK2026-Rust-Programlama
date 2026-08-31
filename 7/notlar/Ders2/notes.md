# Gün 7 · Ders 2 — Lifetime: Neden Var, Nasıl Okunur

Aynı büro. Bugün tanık ifadeleri üzerinde çalışıyoruz: uzun metinleri **kopyalamadan**,
dilimleyerek.

## Tek cümlelik mesaj

> **Lifetime bir açıklamadır, emir değil.**
> Ömrü siz uzatmıyorsunuz; derleyiciye "bu referanslar şu kadar yaşar" diye anlatıyorsunuz.

En yaygın yanlış anlama `'a`'nın bir şeyi "uzattığını" sanmaktır. Uzatmıyor; sadece
ilişkiyi yazıyor.

## Somut ömür (concrete lifetime)

Her **değerin** bir ömrü vardır: doğduğu satırda başlar, düştüğü ya da **taşındığı**
satırda biter. Gün 2'de ownership'i böyle anlatmıştık; "lifetime" o sürenin adıdır.

Ömrü bitiren **üç** olay vardır:

```rust
// (a) kapsam bitti
{ let temp = String::from("gecici tutanak"); }

// (b) başka bir binding'e TAŞINDI
let original = String::from("ilk tutanak");
let moved = original;
println!("{}", original);        // E0382: borrow of moved value

// (c) fonksiyona DEĞERLE geçildi
let report = String::from("gunluk rapor");
file_away(report);
println!("{}", report);          // E0382: ömrü çağrı satırında bitti
```

Gün 2'de bunu "ownership" diye öğrendik; "lifetime" o sürenin adı. Dikkat: (b) ve (c)'de
değer hâlâ bellekte — ama **o binding'in** ömrü bitti.

Değer taşımak serbesttir:

```rust
let outer;
{
    let inner = String::from("otoparkta bir golge vardi");
    outer = inner.len();          // uzunluk kopyalandı
}
println!("{}", outer);            // 25
```

Referans taşımak değildir:

```rust
let outer_ref;
{
    let inner = String::from("otoparkta bir golge vardi");
    outer_ref = &inner;
}
println!("{}", outer_ref);
```

```
error[E0597]: `inner` does not live long enough
```

`inner` blok bitince düştü; `outer_ref` onu göstermeye devam edemez. Borrow checker'ın
tek işi bu: **referans, gösterdiği veriden uzun yaşamasın.**

## Sarkan referans — `E0106`

```rust
fn latest_note_broken() -> &str {
    let note = String::from("gece bekcisi 23:40 dedi");
    &note                       // note fonksiyon bitince düşüyor
}
```

```
error[E0106]: missing lifetime specifier
```

Hatanın söylediği şey: "bu referans **nereden** geliyor?" Fonksiyonun referans girdisi
yok, dolayısıyla dönüşü bağlayacak bir ömür de yok. Çözüm `'a` eklemek değil — **sahipliği
döndürmek**:

```rust
fn latest_note() -> String {
    String::from("gece bekcisi 23:40 dedi")
}
```

## İki girdi: hangisi dönüyor?

```rust
fn longer_statement(a: &str, b: &str) -> &str      // E0106
```

Derleyici dönenin `a`'dan mı `b`'den mi geldiğini bilmiyor. `'a` yazınca söylüyoruz:

```rust
fn longer_statement<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Burada `'a`, iki girdinin **kısa** olanına eşitlenir; dönen referans da o kadar yaşar.
Kısıt canlı olarak şöyle görünür:

```rust
let long_lived = String::from("tanik A: araba maviydi");
let winner;
{
    let short_lived = String::from("tanik B: kirmizi");
    winner = longer_statement(&long_lived, &short_lived);
    println!("{}", winner);        // blok içinde serbest
}
println!("{}", winner);            // E0597: `short_lived` does not live long enough
```

`long_lived` hâlâ yaşıyor olsa bile fark etmez: `'a` kısa olana bağlandı.

### Her parametreye `'a` yazmak zorunda değilsiniz

Dönüş tek bir girdiye bağlıysa yalnızca onu işaretlersiniz:

```rust
fn preferred<'a>(primary: &'a str, _fallback: &str) -> &'a str {
    primary
}
```

`_fallback`'in ömrü dönüşü ilgilendirmiyor, o yüzden `'a` almadı. İmza aynı zamanda
**belgedir**: okuyan, dönenin `primary`'den geldiğini görür.

## Elision — neden çoğu zaman `'a` yazmıyoruz

Derleyici üç kuralı **sırayla** uygular. Çözülürse siz `'a` yazmazsınız; çözülmezse
hata alırsınız. Üçünün de örneği `main.rs`'te, hem elision'lı hem açık hâliyle:

### Kural 1 — referans olan her parametre kendi ömrünü alır

```rust
fn same_length(a: &str, b: &str) -> bool                          // yazdığınız
fn same_length<'a, 'b>(a: &'a str, b: &'b str) -> bool            // derleyicinin gördüğü
```

İki ayrı ömür atandı. Fonksiyon referans **döndürmediği** için iş burada biter — bağlanacak
bir çıkış yok.

### Kural 2 — tek girdi ömrü varsa çıkışa o atanır

```rust
fn first_word(s: &str) -> &str                                    // yazdığınız
fn first_word<'a>(s: &'a str) -> &'a str                          // derleyicinin gördüğü
```


fn longest_int<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if *x >= *y {
        x
    } else {
        y
    }
}

let x = 10;
    let result: &i32;

    {
        let y = 20;
        result = longest_int(&x, &y);
    } // `y` burada kapsam dışı kalır ve ölür.

    println!("result: {result}"); // HATA: `y` does not live long enough

Tek girdi olduğu için belirsizlik yok: dönen referans `s`'ten gelir, başka ihtimal yok.

### Kural 3 — `&self` varsa çıkışa `self`'in ömrü atanır

```rust
impl Casebook {
    fn title(&self) -> &str                                       // yazdığınız
    fn title<'a>(&'a self) -> &'a str                             // derleyicinin gördüğü
}
```

Bu kural metotları neredeyse tamamen `'a`'dan kurtarır. İki referans girdi olsa bile
çalışır:

```rust
fn find(&self, keyword: &str) -> Option<&String>
```

Dönen referans **`self`'e** bağlıdır, `keyword`'e değil. Kanıtı çıktıda:

```
kural 3 | arama           : Some("23:40 tanik beyani")
kural 3 | defter yasiyor  : 2 kayit
```

`keyword` kısa yaşayan bir `String`'di ve bloktan çıkınca düştü; defter yerinde duruyor.

### Kurallar yetmezse

```rust
fn longer_statement(a: &str, b: &str) -> &str      // E0106
```

Kural 1 iki ayrı ömür verdi, kural 2 işlemedi (tek girdi değil), kural 3 işlemedi
(`&self` yok). Geriye elle yazmak kalıyor.

Kendinize sorun: `first_word` neden çalışıyor da `longer_statement` çalışmıyor? Cevap
2. kural — birinde tek girdi var, diğerinde iki.

## NLL — ömür son **kullanımda** biter

```rust
let mut leads = vec![String::from("otopark"), String::from("plaka")];
let peek = &leads[0];
println!("{}", peek);            // peek'in son kullanımı
leads.push(String::from("bekci"));   // artık yazma ödüncü alınabiliyor
```

Ödünç, kapsamın sonuna kadar değil **son kullanıma** kadar sürer. `peek`'i `push`'tan
sonra kullansaydık ömürler çakışırdı:

```
error[E0502]: cannot borrow `leads` as mutable because it is also borrowed as immutable
```

## Sık gelen yanlış anlamalar

| Sanılan | Doğrusu |
|---|---|
| `'a` ömrü uzatır | Sadece ilişkiyi anlatır |
| Lifetime çalışma zamanında vardır | Tamamen derleme zamanı; kod üretilmez |
| Her referansa `'a` yazmak gerekir | Elision çoğunu halleder |
| `'a` bir süre birimidir | Bir **isimdir**; "şu referansla aynı ömür" demek |

## Hata mesajı çözücü

Bugünün lab'ında bu kodlarla karşılaşacaksınız. Hepsi bu makinede derlenip doğrulandı;
mesajı okuyunca hangi soruyu sorduğunu bilin:

| kod | mesaj | ne diyor | tipik çözüm |
|---|---|---|---|
| `E0106` | missing lifetime specifier | "bu referans **nereden** geliyor?" | Girdi varsa `'a` ekleyin; yoksa sahiplik döndürün (`String`) |
| `E0597` | `x` does not live long enough | Referans, gösterdiği veriden **uzun yaşıyor** | Veriyi daha dışarıda tanımlayın ya da klonlayın |
| `E0515` | cannot return value referencing local variable | Yerelin referansını **döndürüyorsunuz** | Sahiplenen tipi döndürün |
| `E0716` | temporary value dropped while borrowed | Geçici değer satır sonunda düştü | Geçiciyi bir `let` ile isimlendirin |
| `E0505` | cannot move out of `x` because it is borrowed | Ödünç dururken **taşımaya** çalıştınız | Ödüncün son kullanımını taşımadan öne alın |
| `E0502` | cannot borrow as mutable because also borrowed as immutable | Okuma ve yazma ödüncü **çakışıyor** | Okumanın son kullanımını yazmadan öne alın (NLL) |
| `E0499` | cannot borrow `x` as mutable more than once | İki `&mut` aynı anda | Kapsamları ayırın |
| `E0507` | cannot move out of borrowed content | `&`'nin arkasından değer taşıma | `clone()`, `take()` ya da referans döndürün |
| `E0382` | use of moved value | Değer taşındı, sonra kullanıldı | Klonlayın ya da referans geçin |

### Her birinin en kısa hâli

Dokuzu da derlenip doğrulandı; hangisini alırsanız karşılığı burada:

```rust
// E0106 — donen referans nereden geliyor?
fn ilk() -> &str { "a" }

// E0597 — s blok bitince dustu, r hala gosteriyor
let r; { let s = String::from("a"); r = &s; } println!("{}", r);

// E0515 — yerelin referansi disari cikamaz
fn ad() -> &'static str { let s = String::from("a"); &s }

// E0716 — gecici deger satir sonunda dustu
let r; { r = &String::from("a"); } println!("{}", r);

// E0505 — odunc dururken tasima
let v = vec![1]; let r = &v; let w = v; println!("{:?}{:?}", r, w);

// E0502 — okuma odunci dururken yazma
let mut v = vec![1]; let r = &v[0]; v.push(2); println!("{}", r);

// E0499 — iki &mut ayni anda
let mut v = vec![1]; let a = &mut v; let b = &mut v;

// E0507 — & arkasindan deger tasima
let v = vec![String::from("a")]; let s: String = v[0];

// E0382 — tasindi, sonra kullanildi
let s = String::from("a"); let t = s; println!("{}", s);
```

**Okuma sırası:** önce `-->` satırındaki **konuma** bakın, sonra `note:` ile başlayan
açıklamayı okuyun, en sonda `help:` çoğu zaman doğrudan çözümü verir. Rust'ın hata
mesajları ders anlatır; kapatmayın, okuyun.

> Bu tablodaki hataların **hiçbiri** bir tehlike değil — hepsi derleyicinin sizi bir
> tehlikeden koruduğu andır. Aynı kod C'de derlenir ve müşteride çöker.
