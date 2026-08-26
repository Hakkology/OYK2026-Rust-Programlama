# Gün 5 · Ders 5 — Prosedürel Makrolar

## İki makro ailesi

| | `macro_rules!` (declarative) | Prosedürel |
|---|---|---|
| Nasıl çalışır | desen eşleştirir | **Rust kodu çalıştırır** |
| Girdi/çıktı | token deseni → kod şablonu | `TokenStream` → `TokenStream` |
| Nerede yaşar | herhangi bir dosyada | **kendi crate'inde** |
| Gücü | şablon kadar | ayrıştırma, sayma, koşul, dosya okuma… her şey |

Prosedürel makro sihirli bir sözdizimi değil: **derleme zamanında çalışan sıradan bir
Rust fonksiyonudur.** Derleyici ona kodunuzun token'larını verir, o da yerine geçecek
token'ları döndürür.

`#[derive(Debug)]` yazdığınızda olan tam olarak budur — Gün 4'te "derleyici sizin
yerinize yazıyor" demiştik; yazan şey bir prosedürel makro.

## Üç tür

```rust
#[derive(Serialize)]        // 1. derive makro     — tipe kod EKLER
#[route(GET, "/index")]     // 2. attribute makro  — öğeyi YENİDEN YAZABİLİR
sql!(SELECT * FROM users)   // 3. fonksiyon benzeri — kendi sözdizimini kurar
```

Önemli fark: **derive makro var olan kodu değiştiremez**, sadece yanına yeni kod ekler.
Attribute makro ise işaretlediği öğeyi tamamen yeniden yazabilir — web çatılarının
`#[tokio::main]`, `#[get("/")]` gibi şeyleri böyle çalışır.

## Neden ayrı crate?

```toml
[lib]
proc-macro = true
```

Bu satır olmadan prosedürel makro yazılamaz. Sebep sıralamadır: makronun kendisi,
onu kullanan kod derlenmeden **önce** derlenip çalıştırılabilir olmalıdır. Derleyici
bunu ancak ayrı bir derleme birimiyle yapabilir. Bu yüzden `serde` yanında
`serde_derive`, `thiserror` yanında `thiserror-impl` vardır.

Bir `proc-macro` crate'i başka bir şey dışa açamaz — sadece makro barındırır.

## Çalışan örnek

`proc_ornek/` klasöründe iki crate var:

```
proc_ornek/
  Cargo.toml            workspace
  label_derive/         proc-macro = true   -> makronun kendisi
  consumer/             makroyu kullanan program
```

```bash
cd proc_ornek && cargo run
```

`consumer` tarafında yazılan tek şey şu:

```rust
#[derive(Label)]
struct Rover { name: String, distance_km: f64 }
```

Karşılığında `Rover::type_name()`, `Rover::field_count()` ve `r.label()` metotları
oluşuyor — hiçbirini biz yazmadık.

Makronun içi (`label_derive/src/lib.rs`) üç adımdan ibaret:

1. Gelen token akışını metne çevir
2. Tip adını ve alan sayısını bul
3. Üretilecek kodu metin olarak kur, `parse()` ile token akışına çevir

## `syn` ve `quote` neden var

Örnekteki ayrıştırma kasten **naif**: alan sayısını iki nokta üst üste sayarak buluyor.
`Vec<(u8, u8)>` gibi bir alan bu sayımı bozar. Gerçek dünyada kimse böyle yazmaz:

| Crate | İşi |
|---|---|
| `syn` | token akışını gerçek bir sözdizim ağacına (AST) ayrıştırır |
| `quote` | `quote! { ... }` ile kod şablonu yazdırır, `#degisken` ile araya değer koyar |
| `proc-macro2` | ikisinin ortak token tipi |

Bu üçlüyle yazıldığında makro şuna benzer:

```rust
#[proc_macro_derive(Label)]
pub fn derive_label(girdi: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(girdi as DeriveInput);
    let ad = &ast.ident;
    quote! {
        impl #ad {
            pub fn type_name() -> &'static str { stringify!(#ad) }
        }
    }.into()
}
```

Metin birleştirme yok, tip güvenli ağaç var. Hata mesajları da kullanıcının kaynak
koduna doğru konumlanır.

## Nereye kadar gidebilir

Prosedürel makro Rust'ın sözdizimini genişletir. Örneğin Python'daki liste üreteci
Rust'a taşınabilir:

```rust
comp![x * y for x in 0..3 if x > 0 for y in 0..3 if y % 2 == 0]
```

Bu, derleme zamanında şuna dönüşür:

```rust
core::iter::IntoIterator::into_iter(0..3)
    .filter_map(move |x| {
        (true && (x > 0)).then(|| {
            core::iter::IntoIterator::into_iter(0..3)
                .filter_map(move |y| (true && (y % 2 == 0)).then(|| x * y))
        })
    })
    .flatten()
```

Yani çalışma zamanında hiçbir maliyet yok; üretilen şey sıradan bir iterator zinciri.
Makro `for`/`if` sözdizimini `syn` ile ayrıştırıp `quote` ile bu zinciri kuruyor,
iç içe döngüleri en içten dışa doğru katlıyor.

## Ne zaman prosedürel makro yazmalı

**Yazın:**
- Aynı `impl` onlarca tip için tekrarlanıyorsa (`serde`, `thiserror` tam olarak bu)
- Bir çatı, kullanıcı kodunu sarmalamak zorundaysa (`#[tokio::main]`)
- Derleme zamanında doğrulanacak bir DSL varsa (SQL sorgusunu derlerken denetlemek)

**Yazmayın:**
- `macro_rules!` yetiyorsa
- Fonksiyon yetiyorsa

Bedeli gerçektir: ayrı crate, ciddi derleme süresi, zor hata mesajları, zor bakım.
Kullanmak bedavaya yakın, **yazmak** pahalıdır.

## Genişlemeyi görmek

```
cargo install cargo-expand
cargo expand
```

`#[derive(Debug)]` dâhil bütün makroların ürettiği gerçek kodu gösterir. Bir kere
bakmak, makroların "sihir" olmadığını anlatan en hızlı yoldur.
