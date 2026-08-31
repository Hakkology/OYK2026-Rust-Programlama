# Gün 9 · Ders 2 — Unsafe Rust

Sekiz gündür "derleyici izin vermiyor" dedik. Bugün izin veren kapıyı açıyoruz — ve neden
çoğu zaman **açmamanız** gerektiğini konuşuyoruz.

## Bu dersin çerçevesi: kaputu açmak

Bugüne kadar güvenle kullandığınız her şeyin **içi `unsafe`**:

| Kullandığınız | İçinde ne var |
|---|---|
| `Vec<T>` — `push`, indeksleme | ham pointer aritmetiği, elle bellek tahsisi |
| `String` | `Vec<u8>` + UTF-8 sözü |
| `Rc<T>`, `RefCell<T>` | ham pointer, `UnsafeCell` |
| `slice::split_at_mut` | iki `&mut` üretmek — güvenli Rust'ta imkânsız |
| `thread::spawn` | işletim sistemi çağrısı |

Yani `unsafe` "kötü kod" değil; **güvenli soyutlamaların üzerinde durduğu zemin**.
Bugün o zemine inip **std'nin bir fonksiyonunu kendimiz yazacağız**, sonra std'nin
gerçek kaynağını açıp karşılaştıracağız.

## `unsafe` ne DEĞİLDİR

Yaygın üç yanlış anlama:

| Sanılan | Doğrusu |
|---|---|
| `unsafe` ödünç denetleyicisini kapatır | **Kapatmaz.** Ownership, ödünç kuralları, ömürler aynen işler |
| `unsafe` kod otomatik olarak hızlıdır | Hayır; güvenli Rust zaten sıfır maliyetlidir |
| `unsafe` "dikkatli ol" demektir | `unsafe` bir **söz**tür: "bu kodun kurallara uyduğunu ben garanti ediyorum" |

`unsafe` yalnızca **beş** şeye izin verir:

1. Ham pointer dereference etmek
2. `unsafe fn` çağırmak
3. Değiştirilebilir `static`'e erişmek
4. `unsafe trait` implemente etmek
5. `union` alanı okumak

Bunların dışında hiçbir kural gevşemez.

## Ham pointer

```rust
let mut sayi = 42;
let p1 = &sayi as *const i32;      // salt okunur
let p2 = &mut sayi as *mut i32;    // yazılabilir
```

**Oluşturmak güvenlidir** — henüz bir şey okumadık. Referanstan farkları:

- `null` olabilir
- sarkabilir (gösterdiği veri düşmüş olabilir)
- aynı anda birden çok `*mut` olabilir — aliasing kuralı yok
- otomatik temizlenmez, ömür taşımaz

Dereference etmek unsafe'tir:

```rust
println!("{}", *p1);
```

```
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
```

```rust
unsafe {
    // SAFETY: p1/p2 az önce geçerli bir yerel değişkenden üretildi, hizalanmış.
    println!("{}", *p1);
    *p2 += 1;
}
```

```
*p1 = 42
*p2 = 43 (degistirdik)
sayi = 43
```

### Tehlike burada başlar

```rust
let sarkan: *const i32 = {
    let gecici = 7;
    &gecici as *const i32      // gecici BURADA düşüyor
};
```

Bu satır derlenir ve **çalışır** — çünkü pointer'ı okumadık. Okusaydık **tanımsız
davranış** (UB) olurdu: program çökebilir, yanlış değer okuyabilir, ya da aylarca sorunsuz
çalışıp müşteride patlayabilir. UB "hata alırsınız" demek değildir; **derleyicinin
varsayımı bozulur** ve artık hiçbir garanti kalmaz.

Gün 7'de bu kodu güvenli Rust'ta yazmayı denemiştik: `E0597` almıştık. Aynı hata, artık
derleyici sizi durdurmuyor.

## Kanonik örnek: `split_at_mut`

Bir dilimi ikiye bölüp **iki `&mut`** döndürmek istiyoruz. Güvenli Rust'ta yazılamaz:

```rust
fn split_at_mut(v: &mut [i32], orta: usize) -> (&mut [i32], &mut [i32]) {
    (&mut v[..orta], &mut v[orta..])
}
```

```
error[E0499]: cannot borrow `*v` as mutable more than once at a time
```

Derleyici **haksız değil, yetersiz**: iki dilimin çakışmadığını ispat edemiyor. Biz
biliyoruz. Bilgiyi ona `unsafe` ile veriyoruz:

```rust
fn split_at_mut(v: &mut [i32], orta: usize) -> (&mut [i32], &mut [i32]) {
    let uzunluk = v.len();
    let bas = v.as_mut_ptr();
    assert!(orta <= uzunluk);                 // sözün ŞARTI: önce kontrol

    unsafe {
        // SAFETY: orta <= uzunluk doğrulandı. [0,orta) ve [orta,uzunluk)
        // kesişimi boş, dolayısıyla iki &mut aynı belleğe bakmıyor.
        (
            slice::from_raw_parts_mut(bas, orta),
            slice::from_raw_parts_mut(bas.add(orta), uzunluk - orta),
        )
    }
}
```

```
sol [100, 2, 3] | sag [200, 5, 6]
butun: [100, 2, 3, 200, 5, 6]
```

**Dersin özü bu:** fonksiyonun imzası güvenli, içi güvensiz. Çağıran `unsafe` yazmıyor.
`assert!` tesadüf değil — sözün ön koşulunu **çalışma zamanında** garanti ediyor.

### Şimdi std'nin gerçek kaynağını açın

Bu dosya sizin makinenizde duruyor:

```
~/.rustup/toolchains/stable-*/lib/rustlib/src/rust/library/core/src/slice/mod.rs
```

`split_at_mut_unchecked`'in içi:

```rust
pub const unsafe fn split_at_mut_unchecked(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    let len = self.len();
    let ptr = self.as_mut_ptr();
    ...
    // SAFETY: Caller has to check that `0 <= mid <= self.len()`.
    //
    // `[ptr; mid]` and `[mid; len]` are not overlapping, so returning a mutable reference
    // is fine.
    unsafe {
        (
            from_raw_parts_mut(ptr, mid),
            from_raw_parts_mut(ptr.add(mid), unchecked_sub(len, mid)),
        )
    }
}
```

**Bizim yazdığımızla aynı.** `len`, `as_mut_ptr`, `from_raw_parts_mut`, hatta `// SAFETY:`
yorumu bile. Tek fark std'nin sınır kontrolünü ayrı bir fonksiyona (`split_at_mut_checked`)
koyması.

Buradaki ders şu: **kullandığınız kütüphane sizden daha akıllı değil, sadece sözünü
yazılı vermiş.**

std'nin yaptığı tam olarak budur: `Vec`, `String`, `RefCell`, `Arc`… hepsinin içi
`unsafe`'tir. Siz güvenli Rust yazabiliyorsanız, birileri bu sözü sizin yerinize verdiği
içindir.

## `unsafe fn` — sözü **çağıran** tutar

İki yazım var ve farkları önemli:

```rust
// (a) fonksiyonun KENDİSİ unsafe: gövdede unsafe blok yok
unsafe fn swap_raw(a: *mut i32, b: *mut i32) {
    let temp = *a;  *a = *b;  *b = temp;
}

// (b) güvenli imza, içeride küçük unsafe blok  (split_at_mut böyleydi)
fn split_at_mut(...) -> ... { ... unsafe { ... } ... }
```

`unsafe fn` demek: *"ben iç kuralları kontrol etmiyorum, **sen** garanti edeceksin."*
Çağıran `unsafe` yazmak zorunda:

```rust
swap_raw(&mut x, &mut y);
```

```
error[E0133]: call to unsafe function is unsafe and requires unsafe block
```

**Hangisini yazmalı?** Neredeyse her zaman (b). `unsafe fn` bütün gövdeyi işaretler ve
sorumluluğu çağırana atar; (b) sorumluluğu sizde tutar. `unsafe fn` sadece ön koşulu
gerçekten çağıranın bilebileceği durumlarda doğrudur (`get_unchecked` gibi).

Yazdığınızda **`// SAFETY:` yorumu imzanın üstüne** gelir: çağıranın neyi garanti etmesi
gerektiğini orada yazarsınız.

### İki örnek daha

**Sınır kontrolünü atlayan okuma.** std'deki `get_unchecked` ile aynı fikir:

```rust
// SAFETY (çağıranın sözü): index < slice.len() olmalı.
unsafe fn read_unchecked(slice: &[i32], index: usize) -> i32 {
    *slice.as_ptr().add(index)
}

// Aynı işin güvenli sarmalayıcısı: sınır kontrolü unsafe'in DIŞINDA.
fn read_or_zero(slice: &[i32], index: usize) -> i32 {
    if index < slice.len() {
        // SAFETY: index < len olduğunu hemen yukarıda doğruladık.
        unsafe { read_unchecked(slice, index) }
    } else {
        0
    }
}
```

Çalıştırınca: `read_unchecked(1)` 20 verir, `read_or_zero(1)` yine 20 verir,
`read_or_zero(99)` ise 0 verir — sınır dışında ama panik yok. `read_unchecked(&veri, 99)`
çağırsaydık tanımsız davranış olurdu; o satırı yorumda bıraktık.

Kalıp her seferinde aynı: **kontrol dışarıda, `unsafe` içeride, imza temiz.**

**Ham pointer üzerinden yazma.** Bu sefer söz iki parçalı — pointer geçerli olacak *ve*
arkasında en az `len` tane yazılabilir `i32` bulunacak:

```rust
// SAFETY (çağıranın sözü): ptr geçerli, arkasında en az len tane i32 var.
unsafe fn fill(ptr: *mut i32, len: usize, value: i32) {
    for i in 0..len {
        *ptr.add(i) = value;
    }
}

let mut hedef = [0i32; 5];
// SAFETY: hedef 5 elemanlık geçerli bir dizi, uzunluğu doğru veriyoruz.
unsafe { fill(hedef.as_mut_ptr(), hedef.len(), 7) };
```

Sonuç `[7, 7, 7, 7, 7]`. Uzunluğu yanlış verseydiniz — mesela 5 yerine 50 — komşu belleği
ezerdiniz. Derleyici bunu kontrol etmiyor; sözü siz veriyorsunuz. Bu iki fonksiyon
`unsafe fn`'in ne demek olduğunun tam karşılığı: **hata ihtimali imzada değil, çağrıda.**

## `union` — beşinci süper güç

Bütün alanlar **aynı belleği** paylaşır:

```rust
#[repr(C)]
union IntOrFloat { i: u32, f: f32 }

let mut u = IntOrFloat { i: 1_065_353_216 };
unsafe { println!("{}", u.i) };      // 1065353216
unsafe { println!("{}", u.f) };      // 1
```

Aynı 4 baytı iki farklı şekilde okuduk. **Bit deseni değişmedi, yorum değişti** —
1065353216 sayısının bit deseni, `f32` olarak okununca tam olarak `1.0`.

```
i olarak: 1065353216
f olarak: 1
f=2.0 yazdik, i olarak: 1073741824
```

Okumak neden unsafe: **hangi alanın geçerli olduğunu derleyici bilmiyor.** `i` yazıp `f`
okursanız çöp okumuş olabilirsiniz — burada kasten öyle yaptık.

```rust
let u = IntOrFloat { i: 1 };
println!("{}", u.i);         // error[E0133]
```

Nerede kullanılır: C ile konuşurken (C `union`'larının Rust karşılığı) ve donanım
kayıtlarını yorumlarken. Günlük Rust'ta yerine `enum` kullanırsınız — `enum` hangi
varyantın geçerli olduğunu **kendisi tutar**, `union` tutmaz. Fark tam olarak budur.

## `unsafe trait` — kendi sözleşmenizi yazmak

Bir trait, metotlarının uyması gereken ama **derleyicinin doğrulayamadığı** bir kuralı
varsa `unsafe` olur:

```rust
unsafe trait Zeroable {
    // SAFETY sözü: bu tipin tüm bit desenleri geçerli olmalı.
    fn zeroed() -> Self;
    fn is_zeroed(&self) -> bool;
}

// SAFETY: Packet sadece [u8; 4] tutuyor; sıfırlarla dolu hâli geçerli bir paket.
unsafe impl Zeroable for Packet { ... }
```

`unsafe impl` yazmayı unutursanız:

```
error[E0200]: the trait `Zeroable` requires an `unsafe impl` declaration
```

İki tarafı ayırt edin:

| | ne demek |
|---|---|
| `unsafe trait` | "beni implemente eden bir söz vermek zorunda" |
| `unsafe impl` | "o sözü veriyorum" |
| `unsafe fn` | "beni çağıran bir söz vermek zorunda" |
| `unsafe { }` | "bu bloktaki sözü ben veriyorum" |

`Send` ve `Sync` (Gün 8) tam olarak böyle trait'lerdir: derleyici çoğu tip için kendisi
çıkarır, ham pointer içeren tiplerde sözü siz verirsiniz.

## Ham bellek üzerinde güvenli arayüz

`split_at_mut` bir fonksiyondu. Aynı fikri bir **tipe** uygularsak `Vec`'in en küçük hâli
çıkar:

```rust
struct HeapBuffer { ptr: NonNull<i32>, capacity: usize }

impl HeapBuffer {
    fn new(capacity: usize) -> HeapBuffer { ... alloc ... }
    fn set(&mut self, index: usize, value: i32) -> Result<(), String> { ... }
    fn get(&self, index: usize) -> Result<i32, String> { ... }
}

impl Drop for HeapBuffer {
    fn drop(&mut self) { ... dealloc ... }
}
```

```
[0] = Ok(42) | [3] = Ok(7)
[1] = Ok(0)  <- new() sifirlamisti
[9] -> Err("9 sinirin disinda (kapasite 4)")
Cagiran hicbir yerde unsafe yazmadi.
    [drop] 4 elemanlik tampon birakildi
```

Dört şey bir arada:

1. **Sınır kontrolü** `unsafe` bloğun **dışında** — söz orada tutuluyor
2. Hata `Result` ile dönüyor, panik yok (Gün 5)
3. `Drop` belleği bırakıyor — tahsis ettiyseniz bırakmak da sizin işiniz
4. Dışarıya bakan hiçbir imzada `unsafe` yok

`Vec<T>` bunun büyük ve çok daha dikkatli yazılmış hâli.

## Global değişken

`static mut` vardır ama **artık kullanmayın**:

```rust
static mut SAYAC: u32 = 0;
unsafe { SAYAC += 1; println!("{}", SAYAC); }
```

```
warning: creating a shared reference to mutable static
note: `#[warn(static_mut_refs)]` (part of `#[warn(rust_2024_compatibility)]`)
```

Rust 2021'de uyarı, 2024'te hata. Sebep: iki thread aynı anda dokunursa veri yarışı, ve
`&mut` ile `&`'yi aynı anda üretmek çok kolay. Modern karşılıkları:

```rust
static SAYAC: AtomicU32 = AtomicU32::new(0);     // thread-safe sayaç
static AYAR: OnceLock<String> = OnceLock::new(); // bir kez yazılır, çok okunur
```

```
atomik sayac: 2
OnceLock ayar: uretim
```

`AtomicU32` Gün 8'deki `Arc`'ın içinde kullanılan mekanizmanın ta kendisi.

## `unsafe impl` — söz veren siz olursunuz

Ham pointer `Send` değildir (Gün 8). "Bu tipi başka thread'e taşımak güvenli" diyorsanız
bunu **siz** garanti edersiniz:

```rust
struct Handle { ptr: *const u8 }

// SAFETY: ptr sabit, salt okunur ve program boyu geçerli bir veriyi gösteriyor.
unsafe impl Send for Handle {}
```

Küçük bir tuzak: Rust 2021 closure'ları **alan bazlı** yakalar. Closure yalnızca
`h.ptr`'yi yakalarsa `unsafe impl Send for Handle` devreye girmez:

```
error[E0277]: `*const u8` cannot be sent between threads safely
```

Metot çağırmak bütün struct'ı yakalatır ve sorun çözülür.

## FFI — başka dille konuşmak

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

let d = unsafe { abs(-13) };      // 13
```

`extern` fonksiyonlar **her zaman** unsafe'tir: derleyici öteki tarafı göremez. C tarafı
null döndürebilir, belleği serbest bırakabilir, sözleşmeye uymayabilir. Rust'ın C
kütüphanelerini kullanabilmesinin bedeli bu.

## Doğrulama araçları

- **Miri** — yorumlayıcı; UB'yi çalışma zamanında yakalar (`cargo +nightly miri test`).
  Nightly gerektirir, bu makinede kurulu değil; kendi unsafe kodunuzu yazacaksanız ilk
  kuracağınız şey olsun.
- **`cargo clippy`** — `undocumented_unsafe_blocks` gibi lint'ler `// SAFETY:` yazmayı
  zorlar.
- **Sanitizer'lar** — `-Z sanitizer=address` (nightly).

## Kural

1. `unsafe` bloklarını **küçük** tutun — bütün fonksiyonu `unsafe fn` yapmayın
2. Her bloğun üstüne **`// SAFETY:`** yazın ve sözün **neden** tutulduğunu açıklayın
3. Dışarıya **güvenli** arayüz verin; kullanıcı `unsafe` yazmasın
4. Ön koşulları `assert!` ile çalışma zamanında doğrulayın
5. Önce güvenli çözümü arayın: `Rc<RefCell<T>>`, indeks tabanlı yapılar, `split_at_mut`
   gibi std fonksiyonları çoğu ihtiyacı karşılar

> `unsafe` bir kaçış kapısı değil, bir **sözleşmedir**. Derleyicinin ispat edemediğini
> siz ispat edersiniz — ve o ispat yanlışsa kimse sizi uyarmaz.
