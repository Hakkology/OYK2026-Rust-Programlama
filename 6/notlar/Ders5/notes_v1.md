# Gün 6 · Ders 5 (v1) — Associated Types

> Ders 5'in **ikinci anlatımı**. Konular aynı; dünya farklı: **araçlar ve biletler**.

## Problem

Vapur vapur kartı keser. Metro kentkart okutur. Tramvay kağıt bilet verir. Üçü de "bilet
kes" diyor ama **döndürdükleri tip farklı**. Bunu tek bir trait ile nasıl yazarsınız?

## Çözüm: trait'in içinde tanımlı tip

```rust
trait Vehicle {
    type Ticket;                       // associated type
    fn issue(&self) -> Self::Ticket;
}

impl Vehicle for Ferry {
    type Ticket = BoatPass;            // Ferry için cevap
    fn issue(&self) -> BoatPass { ... }
}

impl Vehicle for Metro {
    type Ticket = SmartCard;           // Metro için cevap
    fn issue(&self) -> SmartCard { ... }
}
```

```
kesildi: 10 gecislik vapur karti
kesildi: kentkart, 5000 krs bakiye
kesildi: 90 dk gecerli kagit bilet
```

Kullanırken tip belli:

```rust
let bilet: BoatPass = ferry.issue();
```

Çağıran hiçbir şey belirtmiyor — `Ferry` deyince `BoatPass` geleceği zaten yazıyor.

## Associated type ile generic parametre farkı

İkisi de "tip bilgisini dışarı taşıma" işini yapar. Ayrım tek soruda biter:

> **Bu tip için doğru cevap tek mi, birden fazla mı?**

### Tek cevap → associated type

Bir tip, bir trait'i **yalnızca bir kez** implemente edebilir:

```rust
impl Vehicle for Ferry { type Ticket = PaperTicket; ... }
```

```
error[E0119]: conflicting implementations of trait `Vehicle` for type `Ferry`
```

Associated type de o tek implementasyona bağlı olduğu için tek cevap verir.

### Birden fazla cevap → generic parametre

Bir bölgeden hem ücret hem mesafe çıkarılabilir — cevap birden fazla:

```rust
trait Estimate<T> {
    fn estimate(&self) -> T;
}

impl Estimate<Fare> for Zone { ... }
impl Estimate<Distance> for Zone { ... }     // AYNI tip, ikinci impl
```

Burada `Estimate<Fare>` ile `Estimate<Distance>` **farklı trait'lerdir**; o yüzden çakışma
yok. Çağıran hangisini istediğini söyler:

```rust
let ucret: Fare = bolge.estimate();
let mesafe: Distance = bolge.estimate();
```

```
3 bolgeden: 15,00 TL ucret, 9000 metre
```

Bedelini de gösteriyor: generic parametrede **çağıranın tipi belirtmesi** gerekir.

```rust
let x = bolge.estimate();
```

```
error[E0283]: type annotations needed
```

İki benzer hata kodunu karıştırmayın:

| kod | ne demek | örnek |
|---|---|---|
| `E0282` | hiçbir bilgi yok, tip çıkarılamıyor | `let v = Vec::new();` |
| `E0283` | birden çok aday var, seçilemiyor | `bolge.estimate()` — iki `impl` de uyuyor |

Associated type'ta böyle bir yük yok.

## std'den iki örnek

`Add` ikisini birden kullanır ve seçim mantıklıdır:

```rust
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

- **`Rhs` generic** — `Fare + Fare` da olabilir, `Fare + u32` da. Sağ taraf birden fazla olabilir.
- **`Output` associated** — belirli bir sol/sağ çifti için sonucun tipi tektir.

```
20,00 TL
22,50 TL
```

`Iterator` yalnızca associated type kullanır: bir iterator'ın ürettiği eleman tipi tektir.

```rust
trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
```

## Bound içinde associated type

Associated type'a koşul koyabilirsiniz:

```rust
fn show_ticket<V>(v: &V)
where
    V: Vehicle,
    V::Ticket: fmt::Display,           // "bilet tipi yazdırılabilir olsun"
```

`V::Ticket` söz dizimi "V'nin trait'inden gelen tip" demektir. Iterator adaptörlerinde çok
görülür (`I::Item: Display` gibi).

## Associated type ve `dyn`

Ders 2'de `Box<dyn Vehicle>` yazmıştık — orada `Vehicle`'ın associated type'ı yoktu.
Burada denerseniz:

```rust
let v: Box<dyn Vehicle> = Box::new(Ferry);
```

```
error[E0191]: the value of the associated type `Ticket` must be specified
```

Sebep basit: `dyn` derken somut tipi unutuyorsunuz. Ama `issue()`'nun ne döndürdüğü
`Ticket`'a bağlı ve `Ticket` somut tipe göre değişiyor. Derleyici çağıranın elinde ne
kalacağını bilemez, o yüzden **açık açık yazmanızı** ister:

```rust
let kentkartlilar: Vec<Box<dyn Vehicle<Ticket = SmartCard>>> =
    vec![Box::new(Metro), Box::new(Funicular)];
```

```
kentkart, 5000 krs bakiye
kentkart, 1500 krs bakiye
```

Liste heterojen ama **bileti ortak**. `Ferry` bu listeye giremez, çünkü onun `Ticket`'ı
`BoatPass`.

## Karar tablosu

| Soru | Cevap |
|---|---|
| Bir tip için cevap tek mi? | associated type |
| Aynı tip için birden çok olabilir mi? | generic parametre |
| Çağıran tipi seçsin mi istiyorsunuz? | generic parametre |
| Çağıran hiç düşünmesin mi istiyorsunuz? | associated type |

Pratik kural: **önce associated type deneyin.** Çakışma çıkarsa generic parametreye geçin.
