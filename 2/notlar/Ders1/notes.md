# Gün 2 · Ders 1 — Stack ve Heap

Ownership'e geçmeden önce "veri nerede duruyor" sorusuna net cevap verebilmek gerekiyor.

```
        STACK                          HEAP
   ┌──────────────┐              ┌──────────────────┐
   │ ptr  ────────┼─────────────►│ m e r h a b a    │
   │ len   7      │              └──────────────────┘
   │ cap   7      │
   └──────────────┘
   let s = String::from("merhaba");
```

`String` **stack'te 24 bayt** tutar (3 × 8), veri heap'te. `Vec` de aynı üçlüyü tutar.
`size_of_val` ile bakınca içerik ne olursa olsun 24 çıkıyor.

## Kilit noktalar

- Stack: boyut derleme zamanında belli, fonksiyonla gelir gider, bedava
- Heap: boyut çalışma zamanında belli, tahsis maliyetli, birinin bırakması gerek
- Dizi stack'te çünkü uzunluğu tipin parçası. `Vec` heap'te çünkü değil.

## Kapasite büyümesi

`main.rs`'teki döngü kapasitenin 0 → 8 → 16 → 32 gittiğini gösteriyor.
Ardından adres örneği: `push_str` sonrası `as_ptr()` çıktısı **değişiyor**.
(Allocator bazen yerinde büyütür ve adres aynı kalır — garantisi yok. Asıl mesele bu:
değişebiliyor olması, referans tutmayı yasaklamaya yetiyor.)

> `let ilk = &v[0]; v.push(4);` neden derlenmiyor? Çünkü `push` vektörü
> heap'te başka bir yere taşıyabilir ve `ilk` sarkta kalırdı.

Bu bağlantı kurulduğu anda borrow checker "keyfi kural" olmaktan çıkıyor.
C++'ta buna **iterator invalidation** denir ve sessizce çöker.

## RAII ve Drop

- `Drop` trait'i — kapsam bitince derleyici `drop` çağrısını **kendisi** koyar
- Siz `drop()` metodunu elle çağıramazsınız (E0040); `std::mem::drop(x)` kullanılır
- Drop sırası **ters**: son tanımlanan önce düşer (stack mantığı)
- C++'taki RAII ile aynı fikir; fark: Rust'ta derleyici bunu zorunlu tutuyor
