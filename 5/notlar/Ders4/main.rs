// Gun 5 / Ders 4 - Declarative Makrolar
// rustc main.rs && ./main
// testler icin:  rustc --test main.rs -o test4 && ./test4

// --- 1) en basit makro: desen -> uretilecek kod ---
macro_rules! selam {
    () => {
        println!("merhaba");
    };
    // ikinci kol: bir ifade alir
    ($ad:expr) => {
        println!("merhaba {}", $ad);
    };
}

// --- 2) yakalama tipleri: ty ve ident ---
macro_rules! tip_takma_ad {
    ($t:ty => $ad:ident) => {
        type $ad = $t;
    };
}
tip_takma_ad!(u32 => Sayac);

// --- 3) HIJYEN: makro icindeki isim disariyi kirletmez ---
macro_rules! artir {
    ($x:ident) => {
        $x += 1;                 // ismi DISARIDAN aldik, o yuzden calisir
    };
}

macro_rules! kirletmez {
    () => {
        let x = 9999;            // bu x, disaridaki x DEGILDIR
        let _ = x;
    };
}

// --- 4) PARANTEZ TUZAGI: C'de var, Rust'ta expr ile YOK ---
// expr yakalamasi metin degil, AYRISTIRILMIS TEK BIR IFADE dugumu yakalar.
// Yerine konurken butunlugu korunur.
macro_rules! kare_expr {
    ($x:expr) => { $x * $x };            // (2+3) * (2+3) = 25
}
// tt token seviyesinde yakalar - iste tuzak burada geri geliyor
macro_rules! kare_tt {
    ( $($x:tt)* ) => { $($x)* * $($x)* }; // 2 + 3 * 2 + 3 = 11
}

// --- 5) TEKRAR: kendi vec! makromuz ---
macro_rules! avec {
    // sifir veya daha fazla ifade, sondaki virgul de kabul
    ( $( $eleman:expr ),* $(,)? ) => {{
        // bos cagrilirsa hic push uretilmez, mut gereksiz kalir - uyariyi bastiriyoruz
        #[allow(unused_mut)]
        let mut v = Vec::new();
        $( v.push($eleman); )*           // her eleman icin bir push satiri uretilir
        v
    }};
    // vec![deger; adet] bicimi
    ( $eleman:expr ; $adet:expr ) => {{
        let mut v = Vec::new();
        v.resize($adet, $eleman);
        v
    }};
}

// --- 6) tekrar eden impl'leri makroyla uretmek ---
trait EnBuyuk {
    fn en_buyuk() -> Self;
}

macro_rules! max_uygula {
    ( $( $t:ty ),+ $(,)? ) => {
        $(
            impl EnBuyuk for $t {
                fn en_buyuk() -> Self { <$t>::MAX }
            }
        )+
    };
}
max_uygula!(u8, u16, u32, i8, i16, i32);

// --- 7) stringify!: ismi METIN olarak kullanmak ---
macro_rules! yazdir_ve_hesapla {
    ($ifade:expr) => {
        println!("{:>18} = {}", stringify!($ifade), $ifade);
    };
}

fn main() {
    selam!();
    selam!("Mars");
    selam!["kose parantez de olur"];      // ( ) [ ] { } ucu de aynidir

    let sayac: Sayac = 42;                // makronun urettigi tip takma adi
    println!("Sayac = {}", sayac);

    // hijyen
    let mut x = 42;
    artir!(x);
    assert_eq!(x, 43);
    kirletmez!();
    println!("hijyen: disaridaki x = {} (makro icindeki 9999 degil)", x);

    // parantez tuzagi: ayni ifade, iki farkli yakalama
    println!("kare_expr!(2 + 3) = {}   <- expr butunlugu korur", kare_expr!(2 + 3));
    println!("kare_tt!(2 + 3)   = {}   <- tt token kopyalar, C'deki tuzak", kare_tt!(2 + 3));

    // kendi vec makromuz
    let bos: Vec<u32> = avec![];
    let sayilar = avec![1, 2, 3];
    let sondaki_virgul = avec![1, 2, 3,];
    let tekrarli = avec![7; 4];
    println!("{:?} {:?} {:?} {:?}", bos, sayilar, sondaki_virgul, tekrarli);

    // makroyla uretilen impl'ler
    println!("u8::en_buyuk  = {}", <u8 as EnBuyuk>::en_buyuk());
    println!("i32::en_buyuk = {}", <i32 as EnBuyuk>::en_buyuk());

    // stringify
    yazdir_ve_hesapla!(2 + 3 * 4);
    yazdir_ve_hesapla!(sayilar.len());

    // NE ZAMAN MAKRO: degisken sayida arguman, tekrar eden impl, isimleri metne cevirme.
    // Bunlarin disinda FONKSIYON yazin - makro hata mesajlarini ve IDE destegini bozar.
    // Uretilen kodu gormek icin: cargo install cargo-expand && cargo expand
}

#[cfg(test)]
mod tests {
    // makrolar metinsel kapsamda oldugu icin use gerekmiyor

    #[test]
    fn bos_vec() {
        let v: Vec<u32> = avec![];
        assert!(v.is_empty());
    }

    #[test]
    fn elemanli_vec() {
        let v: Vec<u32> = avec![42, 43];
        assert_eq!(v.len(), 2);
        assert_eq!(v[1], 43);
    }

    #[test]
    fn tekrarli_vec() {
        let v: Vec<u32> = avec![7; 3];
        assert_eq!(v, vec![7, 7, 7]);
    }

    #[test]
    fn parantez_tuzagi() {
        assert_eq!(kare_expr!(2 + 3), 25);   // expr: ifade butun kalir
        assert_eq!(kare_tt!(2 + 3), 11);     // tt: token kopyalanir, tuzak geri gelir
    }
}
