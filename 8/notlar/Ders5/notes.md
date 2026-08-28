# Gün 8 · Ders 5 — Tek Yönlü Bağlı Liste

Mutfağın sipariş rayı: yeni fiş **en öne** asılır, şef en öndekini alır. Yani LIFO.

Bu ders yeni bir konu değil; **kampın toplamı**. Tek bir veri yapısında şunların hepsi
buluşuyor:

| parça | nereden |
|---|---|
| `Box` | Gün 7 Ders 1 — özyinelemeli tip |
| `Option` | Gün 4 — "sonrası var mı?" |
| generic `<T>` | Gün 6 Ders 1 |
| `'a` | Gün 7 Ders 3 — kopyalamadan gezmek |
| `type Item` | Gün 6 Ders 5 — `Iterator` |
| `Drop` | Gün 7 Ders 1 |

## Tip tanımı

```rust
type Link<T> = Option<Box<Node<T>>>;      // okunaklılık için takma ad

struct Node<T> { elem: T, next: Link<T> }

pub struct TicketRail<T> { head: Link<T>, len: usize }
```

Üç karar var, üçü de bilinçli:

- **`Box`**: özyinelemeli tipin boyutu hesaplanabilsin (Gün 7'de `E0072` almıştık)
- **`Option`**: "son düğüm" ayrı bir tip değil, `None`
- **takma ad**: `Option<Box<Node<T>>>` üç yerde geçince okunmuyor

## `push` — ve `take()` neden şart

```rust
fn push(&mut self, elem: T) {
    let yeni = Box::new(Node { elem, next: self.head.take() });
    self.head = Some(yeni);
    self.len += 1;
}
```

`self.head`'i doğrudan taşımayı deneyin:

```rust
let eski = self.head;
```

```
error[E0507]: cannot move out of `self.head` which is behind a mutable reference
```

Elimizde `&mut self` var, **sahiplik yok**. `take()` bu düğümü çözer: yerine `None`
bırakır, eskisini bize verir. Liste bir an için "başsız" kalır ama **geçerli** kalır —
Rust'ın hiçbir zaman yarım bir duruma izin vermemesi bu.

## `pop` — `Option` kombinatörleriyle

```rust
fn pop(&mut self) -> Option<T> {
    self.head.take().map(|dugum| {
        self.head = dugum.next;
        self.len -= 1;
        dugum.elem
    })
}
```

`map` içinde `dugum` **sahiplenilmiş** durumda, o yüzden hem `next`'ini hem `elem`'ini
alabiliyoruz. Boş listede `take()` `None` döndürür, `map` hiç çalışmaz — `if` yazmaya
gerek kalmıyor.

## `peek` — `as_ref()`

```rust
fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|dugum| &dugum.elem)
}
```

`as_ref()`, `Option<Box<Node<T>>>`'ü `Option<&Box<Node<T>>>` yapar: içeriği **taşımadan**
ödünç alır. Olmasaydı `self.head`'i taşımaya çalışırdık ve yine `E0507`.

`peek_mut` aynısının `as_mut()` sürümü; dönen `&mut T` ile baştaki fişi yerinde
değiştirebiliyoruz.

## `Iterator` — kopyalamadan gezmek

```rust
struct Iter<'a, T> { simdiki: Option<&'a Node<T>> }

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.simdiki.map(|dugum| {
            self.simdiki = dugum.next.as_deref();
            &dugum.elem
        })
    }
}
```

İki gün önceki iki konu burada birleşiyor: **`'a`** dönen referansın listeye bağlı
olduğunu söylüyor, **`type Item`** bir iterator'ın tek bir tip ürettiğini.

`as_deref()`: `Option<Box<Node<T>>>` → `Option<&Node<T>>`. `Box`'ın `Deref`'i sayesinde
(Gün 7 Ders 1) tek çağrıda hem ödünç alıyor hem kutuyu açıyor.

Karşılığı: Gün 7'nin bütün kombinatörleri bu listede çalışır.

```
toplam 3030 | en buyuk Some(3000) | uzunluk 3
15'ten buyukler: [3000, 20]
```

`TicketRail`'in kendisi de `Iterator` — o sürüm **tüketir** (`next` = `pop`).

## `Drop` — sessiz bir tuzak

Varsayılan `drop` özyinelemeli çalışır: `head` düşerken `next` düşer, o düşerken bir
sonraki… Uzun listede **yığın taşar.** Ölçtük — 200.000 düğüm:

```
olusturuldu, simdi dusuyor
thread 'main' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

İteratif sürüm sorunu bitirir:

```rust
impl<T> Drop for TicketRail<T> {
    fn drop(&mut self) {
        let mut simdiki = self.head.take();
        while let Some(mut dugum) = simdiki {
            simdiki = dugum.next.take();     // bağlantıyı kes, sonra düğüm düşsün
        }
    }
}
```

```
200000 dugum olusturuldu
temizlendi - ozyinelemeli drop olsaydi yigin tasardi
```

> Bu, "Rust güvenli, o hâlde her şey halloldu" düşüncesine iyi bir panzehir. Yığın taşması
> bellek güvenliği ihlali **değildir**; program kontrollü biçimde ölür. Ama yine de sizin
> çözmeniz gereken bir tasarım sorunudur.

## Neden gerçek projede `Vec` kullanırsınız

Bağlı liste ders kitaplarının klasiğidir, pratikte nadiren doğru seçimdir:

| | bağlı liste | `Vec<T>` |
|---|---|---|
| bellek | her düğüm ayrı tahsis | tek blok |
| önbellek | her adımda pointer takibi | ardışık, cache dostu |
| başa ekleme | O(1) | O(n) |
| sona ekleme | O(n) (kuyruk tutulmazsa) | amortize O(1) |
| indeksleme | O(n) | O(1) |

Modern donanımda **önbellek satırı** her şeyi belirler. `Vec` ardışık olduğu için
işlemci sonraki elemanı zaten getirmiştir; bağlı listede her adım bir bellek sıçramasıdır.

Bağlı listeyi öğrenmenizin sebebi onu kullanmanız değil: `Box`, `Option`, `take`,
lifetime ve `Drop`'un tek bir yerde nasıl birleştiğini görmeniz.
