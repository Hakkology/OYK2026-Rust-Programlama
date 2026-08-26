//! Prosedurel makro ornegi: #[derive(Etiket)]
//!
//! Prosedurel makro, DERLEME ZAMANINDA CALISAN normal Rust kodudur.
//! Girdisi bir token akisi, ciktisi da bir token akisidir.
//! Gercek projelerde ayristirma icin `syn`, kod uretmek icin `quote` kullanilir;
//! burada bagimlilik olmasin diye elle, kaba bir ayristirma yapiyoruz.

use proc_macro::TokenStream;

#[proc_macro_derive(Etiket)]
pub fn etiket_turet(girdi: TokenStream) -> TokenStream {
    // girdi: "struct Rover { ad: String, mesafe_km: f64 }" gibi bir token akisi
    let kaynak = girdi.to_string();

    // --- 1) tip adini bul ---
    let ad = kaynak
        .split_whitespace()
        .skip_while(|t| *t != "struct" && *t != "enum")
        .nth(1)
        .unwrap_or("Bilinmeyen")
        .trim_end_matches('{')
        .trim()
        .to_string();

    // --- 2) alan sayisini KABACA say (sadece ustteki seviyedeki virguller) ---
    // Bu naif sayim, syn crate'inin neden var oldugunu anlatan en iyi ornek:
    // Vec<(u8, u8)> gibi bir alan bu sayimi bozar.
    let govde = kaynak.find('{').map(|i| &kaynak[i + 1..]).unwrap_or("");
    let govde = govde.trim_end_matches('}');
    let alan_sayisi = if govde.trim().is_empty() {
        0
    } else {
        govde.matches(':').count()
    };

    // --- 3) uretilecek kodu metin olarak kur ve token akisina cevir ---
    let uretilen = format!(
        r#"
        impl {ad} {{
            /// Bu metodu {ad} icin derleyici degil, MAKRO yazdi.
            pub fn tip_adi() -> &'static str {{ "{ad}" }}
            pub fn alan_sayisi() -> usize {{ {alan_sayisi} }}
            pub fn etiket(&self) -> String {{
                format!("[{ad} / {alan_sayisi} alan]")
            }}
        }}
        "#
    );

    uretilen.parse().expect("uretilen kod gecerli Rust olmali")
}
