# Gün 3 · Ders 1 — Fonksiyonlarda Sahiplik

## Sorun

Bir değeri fonksiyona parametre olarak verdiğinizde değer **taşınır**. Çağıran onu kaybeder:

```rust
let s = String::from("merhaba");
yut(s);
println!("{}", s);   // E0382: borrow of moved value
```

Copy tipler (sayılar, `bool`, `char`, `&T`) taşınmaz, kopyalanır. Sorun yalnızca
heap tutan tiplerde: `String`, `Vec`, `Box`.

## Çalışmayan çözümler

**1. Geri döndür.** Tek fonksiyonda işe yarar, zincir uzayınca dayanılmaz olur:

```rust
let s = al_ve_geri_ver(s);
let s = tekrar_al(s);
let s = bir_daha(s);
```

Her satırda değeri elden ele taşıyorsunuz; asıl iş kayboluyor.

**2. Tuple ile hem sonucu hem değeri döndür.** Çirkin ve bulaşıcı:

```rust
fn uzunluk_ve_geri_ver(s: String) -> (String, usize) {
    let u = s.len();
    (s, u)
}
```

Fonksiyonun gerçek sonucu `usize`; `String` sadece iade edilmek için dönüş tipinde duruyor.

## Gerçek çözüm: ödünç almak

```rust
fn uzunluk_odunc(s: &String) -> usize {
    s.len()
}
```

`&s` bir **referans** oluşturur: sahiplik geçmez, fonksiyon değere bakar ve biter.
Değer hâlâ çağıranındır.

Referansın kendisi ucuzdur — bir adres kadar, 8 bayt. Veriyi kopyalamaz, sadece gösterir.

## `&` ve `*`

- `&x` → referans oluşturur, sahiplik geçmez
- `*r` → referansın gösterdiği değere iner
- `println!` ve karşılaştırmada otomatik çözülür: `r == &5` yerine `*r == 5` da yazılabilir
- Atama ve aritmetikte elle gerekir: `*n *= 2` — `*` olmazsa referansı çarpmaya çalışırsınız

```rust
let x = 5;
let r = &x;
println!("{}", *r + 1);   // 6
```

## İmza bir sözleşmedir

```
fn f(s: String)       alır, geri vermez        — tüketir
fn f(s: &String)      okur                     — ödünç
fn f(s: &mut String)  okur ve değiştirir       — dışlayıcı ödünç
fn f() -> String      üretir, size verir       — sahiplik döner
```

Asıl mesaj: **imzaya bakarak, içine bakmadan ne olacağını bilirsiniz.** Diğer dillerde
bu bilgi imzada yok, dokümantasyonda (varsa) yazar.

Çağrı yerinde de `&mut` yazmak zorundasınız:

```rust
sirala(&mut v);
```

Kodu okuyan kişi `v`'nin bu satırda değişeceğini görür. Sessizce değiştirilen argüman yok.

## `mut` parametresi ≠ `&mut` parametresi

```rust
fn yerel_degistir(mut s: String)   // sahipliği alır, kendi kopyasını değiştirir
fn ekle(s: &mut String)            // sahiplik almaz, çağıranın verisini değiştirir
```

`mut s: String` çağırana hiçbir şey yapmaz — değeri zaten aldı, üstüne "değiştirebilirim"
diyor. `&mut String` ise çağıranın elindeki veriyi değiştirir.

İkisi sık karıştırılıyor; fark tamamen imzada görünüyor.

## Dönüş değeri de taşınır

```rust
fn uret() -> String {
    let s = String::from("uretildi");
    s
}
```

Fonksiyon içinde doğan değer, dönerken çağırana geçer. Kopya yok, taşıma var.

Parametreden gelen bir referansı geri döndürmek de serbesttir:

```rust
fn ilk(v: &Vec<i32>) -> &i32 {
    &v[0]
}
```

Dönen referans çağıranın verisini gösterir, fonksiyonun kendi verisini değil.

## Değer ne zaman düşer

Sahip kimse düşüren de o. Fonksiyon değeri **aldıysa**, değer fonksiyon içinde,
çağrı dönmeden düşer:

```rust
yut(s);          // s'in verisi burada bırakıldı
```

Ödünç alan fonksiyon düşürmez; değer çağıranda kalmaya devam eder.

`drop(x)` değeri erken düşürür. Bu bir dil özelliği değil, sadece sahipliği alıp
hiçbir şey yapmayan bir fonksiyon:

```rust
let gecici = String::from("geçici veri");
drop(gecici);
// println!("{}", gecici);   // E0382 — sahipliği drop aldı
```

`drop` **metodunu** elle çağıramazsınız: kapsam sonunda derleyici zaten çağıracak,
değer iki kez düşmüş olurdu.

```rust
let v = vec![1, 2, 3];
v.drop();     // E0040: explicit use of destructor method
```

Küçük ayrıntı: bu hatayı almak için tipin kendi `Drop` implementasyonu olmalı.
`Vec` uygular, `String` uygulamaz — `String`'de aynı satır `E0599` (böyle bir metot yok)
verir. İkisinde de sonuç aynı: elle çağıramıyorsunuz, `drop(x)` fonksiyonunu kullanıyorsunuz.

## Parametre tipi seçimi

Ödünç alırken **dar tipi değil, geniş tipi** isteyin:

| Yazmayın | Yazın |
|---|---|
| `&String` | `&str` |
| `&Vec<T>` | `&[T]` |

```rust
fn uzunluk_str(s: &str) -> usize { s.len() }

uzunluk_str(&sahipli_string);   // String geçer
uzunluk_str("sabit metin");     // sabit metin de geçer
```

```rust
fn topla(v: &[i32]) -> i32 { ... }

topla(&vektor);       // Vec geçer
topla(&dizi);         // dizi geçer
topla(&vektor[1..3]); // dilim de geçer
```

`&String` yazarsanız çağıranı gereksiz yere `String` üretmeye zorlarsınız; `&Vec<T>`
yazarsanız elinde dizi olan kimse fonksiyonunuzu çağıramaz. İkisi de tek bir şey
yapıyor: gereğinden fazlasını istemek.

Dönüş tipinde tersi geçerli: ürettiğiniz değeri `String` / `Vec<T>` olarak, yani
sahipliğiyle döndürürsünüz.
