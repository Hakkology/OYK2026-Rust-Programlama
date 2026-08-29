# Gün 8 · Ders 1 — Thread'ler ve Sahiplik

Dünya: 2087, Neo-İzmir. Bir soygun ekibi Ariva Kulesi'ne giriyor — hacker güvenliği kırıyor, sürücü motoru çalıştırıyor, kasacı kapıyı açıyor. Hepsi **aynı anda**.

> Bu dersin ilk 15 dakikası sunumdur: `8/slides/Eszamanlilik-Thread-Ownership.pptx`
> (eşzamanlılık nedir, thread nedir, Rust nasıl uyguluyor, ownership ne oluyor).
> Aşağısı sunumdan sonraki kod seansı.

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
  [merkez] 1. kamera devre disi
    [hacker] 1. guvenlik katmani kirildi
    [hacker] 2. guvenlik katmani kirildi
  [merkez] 2. kamera devre disi
```

## `join()` bir `Result` döndürür

Thread panikleyebilir. Panik o thread'i öldürür ama **programı öldürmez**; `join()` size
bunu `Err` olarak bildirir:

```rust
let riskli = thread::spawn(|| panic!("uye yakalandi"));

match riskli.join() {
    Ok(_)  => println!("gorev tamam"),
    Err(_) => println!("thread PANIKLEDI -> join Err dondu, main devam ediyor"),
}
```

```
thread PANIKLEDI -> join Err dondu, main devam ediyor
```

`unwrap()` yazsaydık main de panikleyecekti. Örneklerde kısalık için `unwrap()`
kullanıyoruz; **gerçek kodda `match`**. Gün 5'teki kural burada da geçerli: `unwrap`
"burada hata olamaz" demektir, ve thread'lerde bunu söyleyemezsiniz.

## `move` neden zorunlu

```rust
let ekipman = vec![String::from("EMP granati")];
thread::spawn(|| println!("{:?}", ekipman));
```

```
error[E0373]: closure may outlive the current function, but it borrows `ekipman`
```

Derleyici thread'in ne kadar yaşayacağını **bilmiyor**. `ekipman` `main`'de düşebilir,
thread hâlâ çalışıyor olabilir → sarkan referans.

> **Bu Gün 2'nin doğrudan sonucu.** C'de bu kod derlenir ve rastgele çöker; Rust'ta
> derlenmez. Aynı hata sınıfını Gün 2'de `&` ile, Gün 7'de `'a` ile gördük; bugün
> thread'lerle görüyoruz. Mekanizma hep aynı.

Çözüm `move`:

```rust
let h = thread::spawn(move || { ...ekipman... });
println!("{:?}", ekipman);        // E0382: thread'e taşındı
```

## `thread::scope` — taşımadan ödünç almak

`move` her şeyi taşımak zorunda bırakıyor. Bazen istemeyiz: veri `main`'de kalsın, iki
thread onu **okusun**. `scope` bunu mümkün kılar, çünkü scope içindeki thread'ler scope
bitmeden **önce** biter — dolayısıyla sarkma ihtimali yok.

```rust
let kasa: Vec<u32> = (1..=100).collect();
let (sol, sag) = kasa.split_at(50);
let toplam = thread::scope(|s| {
    let a = s.spawn(|| sol.iter().sum::<u32>());     // ödünç alıyor, taşımıyor
    let b = s.spawn(|| sag.iter().sum::<u32>());
    a.join().unwrap() + b.join().unwrap()
});
println!("{}", kasa.len());                          // hâlâ kullanılabilir
```

```
paralel sayim: 5050 kredi
kasa hala elimizde: 100 deste
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
