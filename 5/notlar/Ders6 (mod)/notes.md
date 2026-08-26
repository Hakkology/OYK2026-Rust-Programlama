# Gün 5 · Ek Not — Modül Kullanımı (ikinci örnek)

Ders 3'teki proje modülleri **klasörlere** yaydı. Bu örnek aynı fikri **tek dosyada**
gösteriyor: `modules.rs` içinde iç içe üç modül var, `main.rs` onları kullanıyor.

```
main.rs        mod modules;  use modules::product::{...};
modules.rs     pub mod product { ... }  pub mod customer { ... }  pub mod order { ... }
```

```bash
rustc main.rs && ./main
```

Çıktı:

```
The intersection is: [Product { id: 2, name: "T-Shirt", price: 20.0, category: Clothing }]
```

## Örnekteki görünürlük kararları

| Satır | Ne diyor |
|---|---|
| `pub mod product` | modül dışarı açık |
| `pub(crate) struct Product` | tip **sadece bu crate içinde** kullanılabilir |
| `id`, `name`, `price` (işaretsiz) | alanlar private — dışarıdan `Product { .. }` kurulamaz |
| `mod category` (işaretsiz) | alt modülün **kendisi** private |
| `pub use category::Category;` | ama içindeki tip **yeniden yayınlanmış** |
| `fn calculate_tax` | private yardımcı |
| `pub fn product_price` | dışarıya açılan hesap |
| `pub(self) fn calculate_discount` | `pub(self)` = hiçbir şey yazmamakla aynı, açık hâli |

En öğretici satır dördüncü ve beşinci: **`category` modülü private, `Category` tipi
public.** Yani dışarıdan şu yazılabilir:

```rust
use modules::product::Category;          // çalışır — re-export
use modules::product::category::Category; // DERLENMEZ — modül private
```

Klasör yapınızı gizleyip yalnızca tipleri sunmanın yolu budur: kullanıcı `category`
diye bir modül olduğunu bilmez, siz onu yarın bölseniz de kodu bozulmaz.

## Kardeş modüller

`order` modülü ikisini birden kullanıyor:

```rust
use super::customer::Customer;
use super::product::Product;
```

`modules.rs` içindeki modüller birbirine `super::` ile bakıyor — çünkü hepsi aynı üst
modülün (yani `modules`) çocuğu. Ders 3'teki projede aynı ilişki `crate::telemetry::...`
ile kuruluyordu; ikisi de doğru, hangisinin kısa olduğuna bakarsınız.

## Uyarılara takılmayın

`cargo`/`rustc` bu dosyada dört "never used" uyarısı verir: `Customer`, `Order`,
`calculate_tax`, `total_bill` hiç çağrılmıyor. Örnek **yapıyı** göstermek için var,
çalışan bir programı değil. Gerçek projede bu uyarılar ciddiye alınır.

## Kapsam dışı kısım

Dosyanın sonundaki şu parça bugünün konusu değil:

```rust
pub trait Intersect<T> { fn intersect(self, other: Vec<T>) -> Vec<T>; }
impl<'a> Intersect<&'a Product> for Vec<&'a Product> { ... }
```

Burada üç yeni şey var: **trait tanımlamak**, **generic** (`<T>`) ve **lifetime** (`'a`).
Bu örnekte sadece "böyle bir şey mümkün" düzeyinde bakın: `Vec<&Product>` gibi hazır bir
tipe kendi metodunuzu ekleyebiliyorsunuz. Nasıl çalıştığı ayrı bir konu.

Çalıştığını görmek isterseniz: `set1.intersect(set2)` iki listede de bulunan ürünü
döndürüyor, çıktıdaki `T-Shirt` o.
