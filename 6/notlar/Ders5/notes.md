# Gün 6 · Ders 5 — Associated Types

Ders 3'te `impl Add for Hp` yazarken `type Output = Hp;` satırını "şimdilik böyle" diye
geçmiştik. O satırın adı **associated type**.

## Problem

Silahları doldurmak isteyelim:

- `Bow` doldurulunca **ok** gelir
- `Musket` doldurulunca **mermi** gelir
- `Staff` doldurulunca **mana yükü** gelir

Yani dönüş tipi, trait'i implemente eden tipe göre **değişiyor**. Generic bir dönüş tipi
yazamayız, çünkü çağıran seçmiyor — silahın cephanesi zaten bellidir.

## Çözüm: trait'in içinde tanımlı tip

```rust
trait Weapon {
    type Ammo;                            // "bir cephane tipi olacak"
    fn reload(&self) -> Self::Ammo;
}

impl Weapon for Bow {
    type Ammo = Arrow;                    // Bow için cevap: Arrow
    fn reload(&self) -> Arrow { Arrow { count: 30 } }
}

impl Weapon for Musket {
    type Ammo = Bullet;                   // Musket için cevap: Bullet
    fn reload(&self) -> Bullet { Bullet { count: 6 } }
}
```

Kullanırken tip belli:

```rust
let ammo: Arrow = bow.reload();           // derleyici Bow -> Arrow olduğunu biliyor
```

## Associated type ile generic parametre farkı

İkisi de "tip bilgisini dışarı taşıma" işini yapar. Ayrım tek soruda biter:

> **Bu tip için doğru cevap tek mi, birden fazla mı?**

### Tek cevap → associated type

```rust
impl Weapon for Bow { type Ammo = Arrow; ... }
impl Weapon for Bow { type Ammo = Bullet; ... }   // İKİNCİSİ YASAK
```

```
error[E0119]: conflicting implementations of trait `Weapon` for type `Bow`
```

Bir tip, bir trait'i **yalnızca bir kez** implemente edebilir. Associated type de o tek
implementasyona bağlı olduğu için tek cevap verir.

### Birden fazla cevap → generic parametre

Demirden hem kılıç hem kalkan yapılabilir — cevap birden fazla:

```rust
trait Craft<T> { fn craft(&self) -> T; }

impl Craft<Sword> for Iron  { ... }     // aynı tip
impl Craft<Shield> for Iron { ... }     // ikinci impl — serbest
```

Burada `Craft<Sword>` ile `Craft<Shield>` **farklı trait'lerdir**; o yüzden çakışma
yok. Çağıran hangisini istediğini söyler:

```rust
let sword: Sword   = iron.craft();
let shield: Shield = iron.craft();
let x              = iron.craft();   // E0283: type annotations needed
```

Son satır bedelini gösteriyor: generic parametrede **çağıranın tipi belirtmesi** gerekir.

İki benzer hata kodunu karıştırmayın:

| Kod | Ne demek | Örnek |
|---|---|---|
| `E0282` | hiçbir bilgi yok, tip çıkarılamıyor | `let v = Vec::new();` |
| `E0283` | birden çok aday var, seçilemiyor | `iron.craft()` — iki `impl` de uyuyor |
Associated type'ta böyle bir yük yok.

## std'den iki örnek

```rust
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

`Add` ikisini birden kullanır ve seçim mantıklıdır:

- **`Rhs` generic** — `Arrow + Arrow` da olabilir, `Arrow + u32` da. Sağ taraf birden
  fazla olabilir.
- **`Output` associated** — belirli bir sol/sağ çifti için sonucun tipi tektir.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

`Iterator` yalnızca associated type kullanır: bir iterator'ın ürettiği eleman tipi tektir.
`Vec<i32>` üzerinde gezen bir iterator hem `i32` hem `String` üretemez.

## Bound içinde associated type

Associated type'a koşul koyabilirsiniz:

```rust
fn show_ammo<W>(w: &W)
where
    W: Weapon,
    W::Ammo: fmt::Display,           // "cephane tipi yazdırılabilir olsun"
```

`W::Ammo` sözdizimi "W'nin trait'inden gelen tip" demektir. Bu, generic kodda
associated type'lara koşul koymanın yoludur ve iterator adaptörlerinde çok görülür
(`I::Item: Display` gibi).

## Associated type ve `dyn`

Ders 2'de `Box<dyn Unit>` yazmıştık. `Weapon` ile aynısını denerseniz:

```rust
let w: Box<dyn Weapon> = Box::new(Bow);
//  error[E0191]: the value of the associated type `Ammo` must be specified
```

Sebep basit: `dyn` derken somut tipi unutuyorsunuz. Ama `reload()`'un ne döndürdüğü
`Ammo`'ya bağlı ve `Ammo` somut tipe göre değişiyor. Derleyici çağıranın elinde ne
kalacağını bilemez, o yüzden **açık açık yazmanızı** ister:

```rust
let w: Box<dyn Weapon<Ammo = Arrow>> = Box::new(Bow);
let arsenal: Vec<Box<dyn Weapon<Ammo = Arrow>>> = vec![Box::new(Bow), Box::new(Crossbow)];
```

Artık liste heterojen ama **cephanesi ortak**: içindeki her silah `Arrow` döndürür.
`Musket` bu listeye giremez, çünkü `Ammo = Bullet`.

Buradan çıkan kural: associated type'lı bir trait'i `dyn` yapacaksanız associated
type'ı sabitlemek zorundasınız. Generic parametreli trait'te (`Craft<T>`) durum aynı —
`dyn Craft<Sword>` yazarsınız, `T`'yi belirtmeden `dyn` olmaz.

## Karar tablosu

| Soru | Cevap |
|---|---|
| Bir tip için cevap tek mi? | associated type |
| Aynı tip için birden çok olabilir mi? | generic parametre |
| Çağıran tipi seçsin mi istiyorsunuz? | generic parametre |
| Çağıran hiç düşünmesin mi istiyorsunuz? | associated type |

Pratik kural: **önce associated type deneyin.** Çakışma çıkarsa (aynı tipe iki impl
gerekiyorsa) generic parametreye geçin. Associated type kullanıcıya daha az yük bindirir.
