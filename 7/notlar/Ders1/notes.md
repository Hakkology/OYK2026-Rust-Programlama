# Gün 7 · Ders 1 — Akıllı İşaretçiler: `Box`, `Rc`, `RefCell`

Dünya: dedektiflik bürosu. Ders boyunca **dört tip** dolaşıyor, hepsi bu:

| tip | ne gösteriyor |
|---|---|
| `Lead` | ipucu zinciri — `Box` |
| `MyBox<T>` | kendi kutumuz — `Deref` |
| `CaseFile` | dosya — `Drop`, `Rc`, `RefCell` |
| `Case` / `Detective` | dava ve ekibi — `Weak` |

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
struct Lead { text: String, next: Option<Lead> }
```

```
error[E0072]: recursive type `Lead` has infinite size
```

Derleyici boyutu hesaplayamıyor: `Lead` = `String` + `Lead` = `String` + `String` + `Lead`…
`Box` araya girince zincir kırılır, çünkü `Box` her zaman **tek bir pointer**:

```rust
struct Lead {
    text: String,
    next: Option<Box<Lead>>,
}
```

```
otoparktaki bilet -> plaka kaydi -> gece bekcisinin ifadesi
```

### 2. Büyük veriyi taşımadan aktarmak

```
[u8; 4096]          4096 bayt
Box<[u8; 4096]>        8 bayt   <- icindeki ne olursa olsun bir pointer
Option<Box<Lead>>      8 bayt   <- Option bedava (Gun 4: niche)
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
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;                 // Gün 6: associated type
    fn deref(&self) -> &T { &self.0 }
}
```

`Deref` `*` operatörünü tanımlar, ama asıl önemli yan etkisi **deref coercion**:
derleyici referans beklenen yerde bu zinciri kendiliğinden takip eder.

```
&MyBox<String>  ->  &String  ->  &str
```

```rust
fn duyur(metin: &str) -> String { ... }

duyur(&kutu);                            // &MyBox<String> geçti
duyur(&String::from("dosya 48 acildi")); // &String geçti
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
    iki dosya acik
    [drop] 47-B dosyasi kapandi
    [drop] 47-A dosyasi kapandi
```

Son açılan önce kapanıyor. Erken düşürmek isterseniz `drop(x)` yazarsınız — `x.drop()`
**değil**, o E0040 verir (Gün 2'de görmüştük).

## `Rc<T>` — paylaşılan sahiplik

Gün 2'de "her verinin **tek** sahibi var" demiştik. Bazen yetmiyor: aynı dosyaya iki
dedektif de bakıyor. `Rc` = *reference counted*.

```rust
let dosya = Rc::new(CaseFile::new("KRG-12"));
let alvarez = Rc::clone(&dosya);         // veri kopyalanmıyor, sayaç artıyor
```

```
sayac: 1
sayac: 2   <- Alvarez de bakiyor
sayac: 3   <- gece vardiyasi da acti
sayac: 2   <- vardiya bitti
```

Veri, sayaç **sıfıra inince** düşer — `CaseFile`'a `Drop` yazdığımız için bunu gözle
görüyoruz:

```
Alvarez birakti, sayac: 1
    [drop] KRG-12 dosyasi kapandi
```

Dört kural:

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
    fn not_ekle(&self, not: &str) {           // &self, &mut self DEĞİL
        self.notes.borrow_mut().push(not.to_string());
    }
}
```

`dosya` `mut` değil ama içindeki notlar değişiyor. "İçsel mutasyon" budur.

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
ikinci borrow_mut REDDEDILDI (already borrowed)
```

Bu yüzden `RefCell` **son çaredir**. Derleme zamanında çözebiliyorsanız orada çözün.

### `Cell<T>` — `RefCell`'in ucuz kardeşi

`RefCell` çalışma zamanında ödünç **sayar**; bu bir bayrak tutmak ve panic riski demek.
`Cell` saymaz — çünkü referans vermez, değeri **kopyalar**:

```rust
let ziyaret = Cell::new(0u32);
ziyaret.set(ziyaret.get() + 1);
```

```
sayac 2 | Cell deger kopyalar, RefCell referans verir
```

| | ne verir | panic riski | ne zaman |
|---|---|---|---|
| `Cell<T>` | değerin kopyası (`get`/`set`) | yok | küçük `Copy` tipler: sayaç, bayrak |
| `RefCell<T>` | referans (`borrow`/`borrow_mut`) | var | `Vec`, `String`, büyük struct'lar |

## `Rc<RefCell<T>>`

Paylaşılan **ve** değiştirilebilir. `Rc<CaseFile>` tam olarak bu: `Rc` ile iki dedektif
aynı dosyayı tutuyor, `RefCell` ile ikisi de not ekleyebiliyor.

```
KRG-12 dosyasinda 2 not var
```

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
struct Case      { team: RefCell<Vec<Rc<Detective>>> }   // aşağı: sahiplik
struct Detective { case: RefCell<Weak<Case>>         }   // yukarı: sahiplik yok
```

`Weak` sahiplenmediği için hedefi düşmüş olabilir; bu yüzden `upgrade()` `Option` döner.

```
dava sayaci: strong 1 / weak 1
Alvarez'in dosyasi: LMN-8
    [drop] LMN-8 dosyasi kapandi
    [drop] Alvarez evine gitti
```

### Sızıntıyı görelim

`main.rs`'te aynı yapının `Weak` yerine `Rc` kullanan bir ikizi var. Tek fark geri
bağlantının tipi:

```rust
struct Detective      { case: RefCell<Weak<Case>>            }   // sahiplik YOK
struct LeakyDetective { case: RefCell<Option<Rc<LeakyCase>>> }   // sahiplik VAR
```

`Weak` sürümü:

```
    [drop] LMN-8 dosyasi kapandi
    [drop] Alvarez evine gitti
```

`Rc` sürümü:

```
  sayaclar: dava 2 / dedektif 2
  blok bitti - HIC DROP SATIRI YOK. Ikisi birbirini tutuyor: bellek sizdi.
```

İkisi birbirini tuttuğu için sayaçlar 2'de takılı kaldı, hiçbir zaman sıfıra inmedi.
`Drop` çalışmadı, bellek serbest kalmadı. **Tek satırlık fark, kalıcı sızıntı.**

> Sızıntı Rust'ta **güvenlidir**: veri okunmaz, program çökmez, sadece bellek boşalmaz.
> Rust bellek güvenliğini garanti eder, sızıntıyı etmez. `Box::leak` bile güvenli bir
> fonksiyondur. Yani bu, derleyicinin değil **sizin** çözeceğiniz bir tasarım sorunudur.

## Özet tablo

| | sahiplik | değiştirilebilir | kontrol |
|---|---|---|---|
| `&T` | yok | hayır | derleme |
| `&mut T` | yok | evet | derleme |
| `Box<T>` | tek | evet (`mut` ise) | derleme |
| `Rc<T>` | paylaşılan | hayır | derleme |
| `Cell<T>` | tek | evet (kopyalayarak) | — |
| `RefCell<T>` | tek | evet | **çalışma** |
| `Rc<RefCell<T>>` | paylaşılan | evet | **çalışma** |

## Karar sırası

Ezberlemeyin, **sırayla sorun**:

1. **Referans (`&T`) yeter mi?** → Yeterse başka hiçbir şey kullanmayın. Çoğu kod burada biter.
2. **Heap'te olması ya da boyutunun bilinmemesi mi gerekiyor?** → `Box<T>`
3. **Gerçekten birden çok sahip mi var?** → `Rc<T>`. "Birden çok yerden okunuyor" değil,
   "hangisinin önce biteceğini bilmiyorum" demek.
4. **Paylaşılan veriyi değiştirmem gerekiyor mu?** → `Rc<RefCell<T>>` — ve önce
   "veriyi yeniden tasarlasam kurtulur muyum?" diye sorun.
5. **Geri/çapraz bağlantı var mı?** → O kenar `Weak`.
6. **Thread'e mi geçecek?** → Gün 8.

`Rc<RefCell<T>>` bir çözümdür ama aynı zamanda bir **kokudur**: veri modelinizin ağaç
değil graf olduğunu söyler. Bazen doğrudur (GUI, oyun sahnesi), bazen tasarım hatasıdır.

## `DerefMut`

`Deref` okuma tarafını hallediyor; yazma tarafı için ikizi var:

```rust
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}
```

`&mut Box<String>` → `&mut String` → `&mut str` zinciri bu sayede çalışır. Kural aynı:
`Deref` yalnızca akıllı işaretçiler içindir.
