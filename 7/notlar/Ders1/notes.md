# Gün 7 · Ders 1 — Akıllı İşaretçiler: `Box`, `Rc`, `RefCell`

Dünya: gece vardiyası dedektiflik bürosu. Dosyalar, ipucu zincirleri, aynı dosyaya bakan
birden çok dedektif.

## Akıllı işaretçi nedir

Bir adres tutar — tıpkı `&` gibi. Farkı: **veriye sahiptir ve ek davranış taşır.**
`String` ve `Vec<T>` de birer akıllı işaretçidir; ikisi de heap'te veri tutar, düştüğünde
belleği bırakır. Bugün üçünü ekliyoruz:

| | ne katıyor |
|---|---|
| `Box<T>` | veriyi heap'e koyar, sahipliği tek elde tutar |
| `Rc<T>` | aynı veriye **birden çok sahip** olmasını sağlar |
| `RefCell<T>` | ödünç kontrolünü **çalışma zamanına** taşır |

## `Box<T>` — üç sebep

### 1. Özyinelemeli tip

Her ipucu bir sonrakine götürüyor. Doğrudan yazarsanız:

```rust
struct Lead { note: String, next: Option<Lead> }
```

```
error[E0072]: recursive type `Lead` has infinite size
```

Derleyici boyutu hesaplayamıyor: `Lead` = `String` + `Lead` = `String` + `String` + `Lead`…
`Box` araya girince zincir kırılır, çünkü `Box` her zaman **tek bir pointer**:

```rust
struct Lead {
    note: String,
    next: Option<Box<Lead>>,
}
```

```
otoparktaki bilet -> plaka kaydi -> gece bekcisinin ifadesi
```

### 2. Büyük veriyi taşımadan aktarmak

```
[u8; 4096]         4096 bayt
Box<[u8; 4096]>       8 bayt
Option<Box<Lead>>     8 bayt
```

Diziyi bir fonksiyona geçerken 4 KB kopyalanır; `Box` ile 8 bayt taşınır.

Üçüncü satır Gün 4'teki **niche optimization**: `Box` asla null olamaz, o yüzden "null"
değeri `None`'ı temsil etmek için kullanılır. `Option<Box<T>>` sarmalayıcısı bedavadır.

### 3. `dyn Trait` taşıyıcısı

Gün 6'da `Vec<Box<dyn Unit>>` yazmıştık. Sebebi aynı: `dyn Unit`'in boyutu bilinmiyor,
`Box` bilinen boyutta bir pointer veriyor.

## `Deref` — Gün 3'te bıraktığımız borç

`Box`'ın sihri yok; `Deref` implemente eden bir tip. Kendimiz de yazabiliriz:

```rust
struct CaseBox<T>(T);

impl<T> Deref for CaseBox<T> {
    type Target = T;                 // Gün 6: associated type
    fn deref(&self) -> &T { &self.0 }
}
```

`Deref` `*` operatörünü tanımlar, ama asıl önemli yan etkisi **deref coercion**:
derleyici referans beklenen yerde bu zinciri kendiliğinden takip eder.

```
&CaseBox<String>  ->  &String  ->  &str
```

```rust
fn announce(text: &str) -> String { ... }

announce(&boxed);     // &CaseBox<String> geçti
announce(&owned);     // &String geçti
```

Gün 3 Ders 3'te "parametrede `&str` al, `&String` alma" demiştik ve nedenini
bugüne bırakmıştık. **Cevap:** `String`'in `Deref<Target = str>` implementasyonu var.
Aynı sebeple `&Vec<T>` otomatik `&[T]` olur.

std'deki örnekler: `String -> str`, `Vec<T> -> [T]`, `Box<T> -> T`, `Rc<T> -> T`.

> Uyarı: `Deref` yalnızca **akıllı işaretçi** tipleri içindir. Kalıtım taklidi yapmak için
> kullanmayın; metotlar sihirli biçimde görünür, okuyan nereden geldiklerini anlayamaz.

## `Drop` — Gün 2'deki RAII

Kapsam bitince derleyici `drop`'u kendisi çağırır, sıra **terstir**:

```
    iki takip suruyor
    [drop] Liman deposu takibi sonlandirildi
    [drop] Kordon Kafe takibi sonlandirildi
```

Erken düşürmek isterseniz `drop(x)` yazarsınız — `x.drop()` **değil**, o E0040 verir
(Gün 2'de görmüştük).

## `Rc<T>` — paylaşılan sahiplik

Gün 2'de "her verinin **tek** sahibi var" demiştik. Bazen yetmiyor: aynı dosyaya iki
dedektif de bakıyor. `Rc` = *reference counted*.

```rust
let file = Rc::new(CaseFile::new("47-B"));
let for_alvarez = Rc::clone(&file);      // veri kopyalanmıyor, sayaç artıyor
```

```
sayac: 1
sayac: 2
sayac (blok icinde): 3
sayac (blok bitti): 2
```

Veri, sayaç sıfıra inince düşer. Dört kural:

- `Rc::clone(&x)` **ucuzdur** — veriyi kopyalamaz, sayacı artırır
- `x.clone()` yerine `Rc::clone(&x)` yazın; okuyan ucuz olduğunu görsün (konvansiyon)
- **`Rc` değiştirilemez.** Birden çok sahip varken biri değiştirse diğerleri şaşırır:

```
error[E0596]: cannot borrow data in an `Rc` as mutable
```

- **Tek iş parçacığı içindir.** Çok iş parçacıklı hâli `Arc`.

## `RefCell<T>` — içsel mutasyon

Ödünç kuralı aynı — ya çok okuyucu ya tek yazıcı — ama kontrol **çalışma zamanında**
yapılır:

```rust
struct CaseFile {
    code: String,
    notes: RefCell<Vec<String>>,
}

impl CaseFile {
    fn add_note(&self, note: &str) {          // &self, &mut self DEĞİL
        self.notes.borrow_mut().push(note.to_string());
    }
}
```

`file` `mut` değil ama içindeki notlar değişiyor. "İçsel mutasyon" budur.

| | ihlal ne zaman yakalanır | müşteriye gider mi |
|---|---|---|
| `&mut T` | derleme (`E0499`) | hayır |
| `RefCell<T>` | çalışma | **evet** |

İhlal edilirse program **panikler**:

```
thread 'main' panicked at ...:
RefCell already borrowed
```

`try_borrow_mut()` panic yerine `Result` döndürür:

```
ikinci borrow_mut REDDEDILDI - already borrowed
```

Bu yüzden `RefCell` **son çaredir**. Derleme zamanında çözebiliyorsanız orada çözün.

## `Rc<RefCell<T>>`

Paylaşılan **ve** değiştirilebilir. Bugünkü `CaseFile` tam olarak bu: `Rc` ile iki
dedektif aynı dosyayı tutuyor, `RefCell` ile ikisi de not ekleyebiliyor.

> `Rc<RefCell<T>>` görünce önce "başka yolu var mı?" diye sorun. Çoğu zaman veriyi
> yeniden tasarlamak daha iyi bir çözümdür.

## `Weak` — döngüsel referans

İki `Rc` birbirini tutarsa sayaç hiç sıfıra inmez → **bellek sızıntısı**. Rust bellek
güvenliğini garanti eder ama sızıntıyı engellemez; sızıntı "güvenli"dir: veri okunmaz,
sadece bellek boşalmaz.

**Kural:**

- aşağı doğru (birim → dedektif) = `Rc`, sahiplik var
- yukarı doğru (dedektif → birim) = `Weak`, sahiplik yok

```rust
struct Department { detectives: RefCell<Vec<Rc<Detective>>> }
struct Detective  { department: RefCell<Weak<Department>>  }
```

`Weak` sahiplenmediği için hedefi düşmüş olabilir; bu yüzden `upgrade()` `Option` döner.

```
birim sayaci  : strong 1 / weak 1
Alvarez'in birimi: Cinayet Masasi
program bitiyor, drop ciktilari:
    [drop] Cinayet Masasi birimi kapandi
    [drop] Alvarez evine gitti
```

Son iki satır kanıt: `Weak` yerine `Rc` koysaydınız bu `drop` çıktıları **hiç
görünmezdi** — ikisi birbirini tutup sızardı.

## Özet tablo

| | sahiplik | değiştirilebilir | kontrol |
|---|---|---|---|
| `&T` | yok | hayır | derleme |
| `&mut T` | yok | evet | derleme |
| `Box<T>` | tek | evet (`mut` ise) | derleme |
| `Rc<T>` | paylaşılan | hayır | derleme |
| `RefCell<T>` | tek | evet | **çalışma** |
| `Rc<RefCell<T>>` | paylaşılan | evet | **çalışma** |
