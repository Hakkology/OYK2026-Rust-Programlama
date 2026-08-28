# Gün 8 · Ders 1 — Thread'ler ve Sahiplik

Dünya: gece servisi mutfağı. Şefler aynı anda çalışıyor, siparişler birikiyor, kiler ortak.

Bugünün konusu **eşzamanlılık**, ama aslında dün öğrendiğimiz iki şeyin uygulaması:
`move` closure (Ders 4) ve paylaşılan sahiplik (Ders 1). Rust'ın eşzamanlılıkta güvenli
olmasının sebebi ayrı bir mekanizma değil — **ownership'in ta kendisi.**

## İlk thread

```rust
let handle = thread::spawn(|| {
    ...
    "servis bitti"                   // thread bir DEĞER döndürebilir
});

let sonuc = handle.join().unwrap();  // bitmesini bekle + değeri al
```

- `spawn` bir `JoinHandle` döndürür
- `join()` beklemek demektir; thread'in döndürdüğü değeri verir
- `join()` çağırmazsanız `main` bitince thread yarıda kesilebilir

Çıktının sırası **garanti değildir** — iki thread bağımsız ilerler:

```
  [salon] 1. masa siparis verdi
    [sef] 1. tabak hazir
    [sef] 2. tabak hazir
  [salon] 2. masa siparis verdi
```

## `move` neden zorunlu

```rust
let siparis = vec![String::from("mercimek corbasi")];
thread::spawn(|| println!("{:?}", siparis));
```

```
error[E0373]: closure may outlive the current function, but it borrows `siparis`
```

Derleyici thread'in ne kadar yaşayacağını **bilmiyor**. `siparis` `main`'de düşebilir,
thread hâlâ çalışıyor olabilir → sarkan referans.

> **Bu Gün 2'nin doğrudan sonucu.** C'de bu kod derlenir ve rastgele çöker; Rust'ta
> derlenmez. Aynı hata sınıfını Gün 2'de `&` ile, Gün 7'de `'a` ile gördük; bugün
> thread'lerle görüyoruz. Mekanizma hep aynı.

Çözüm `move`:

```rust
let h = thread::spawn(move || { ...siparis... });
println!("{:?}", siparis);        // E0382: thread'e taşındı
```

## `thread::scope` — taşımadan ödünç almak

`move` her şeyi taşımak zorunda bırakıyor. Bazen istemeyiz: veri `main`'de kalsın, iki
thread onu **okusun**. `scope` bunu mümkün kılar, çünkü scope içindeki thread'ler scope
bitmeden **önce** biter — dolayısıyla sarkma ihtimali yok.

```rust
let stok: Vec<u32> = (1..=100).collect();
let (sol, sag) = stok.split_at(50);
let toplam = thread::scope(|s| {
    let a = s.spawn(|| sol.iter().sum::<u32>());     // ödünç alıyor, taşımıyor
    let b = s.spawn(|| sag.iter().sum::<u32>());
    a.join().unwrap() + b.join().unwrap()
});
println!("{}", stok.len());                          // hâlâ kullanılabilir
```

```
paralel toplam: 5050
stok hala kullanilabilir: 100 kalem
```

Rust 1.63'te stabilize oldu; öncesinde `crossbeam` crate'i gerekiyordu.

## Paralellik her zaman kazandırmaz

Thread açmanın **sabit** bir maliyeti var. Kazanıp kazanmadığını iş miktarı belirler:

```
     1000 eleman | tek      2.6µs | cift     95.7µs | TEK thread kazandi
  2000000 eleman | tek      5.1ms | cift      2.8ms | iki thread kazandi
```

Küçük işte thread açmanın maliyeti işin kendisinden büyük. **Önce ölçün, sonra
paralelleştirin.** (`rustc -O` ile tekrar ölçün; iyimserleştirici tabloyu değiştirir.)

## Bugünün haritası

| Ders | Soru |
|---|---|
| 1 | Thread nasıl açılır, veri nasıl taşınır |
| 2 | Aynı veriyi **birden çok thread değiştirecekse** ne yapılır |
| 3 | Thread'ler birbirine nasıl **haber** verir |
| 4 | 10.000 bağlantı için 10.000 thread açamayınca ne yapılır |
| 5 | Öğrendiğimiz her şeyle bir veri yapısı: bağlı liste |
