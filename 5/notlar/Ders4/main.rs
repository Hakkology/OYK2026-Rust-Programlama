// Gun 5 / Ders 4 - Declarative Makrolar
// rustc main.rs && ./main
// testler icin:  rustc --test main.rs -o test4 && ./test4

// --- 1) en basit makro: desen -> uretilecek kod ---
macro_rules! hello {
    () => {
        println!("merhaba");
    };
    // ikinci kol: bir ifade alir
    ($ad:expr) => {
        println!("merhaba {}", $ad);
    };
}

// --- 2) yakalama tipleri: ty ve ident ---
macro_rules! type_alias {
    ($t:ty => $ad:ident) => {
        type $ad = $t;
    };
}
type_alias!(u32 => Counter);

// --- 3) HIJYEN: makro icindeki isim disariyi kirletmez ---
macro_rules! increment {
    ($x:ident) => {
        $x += 1;                 // ismi DISARIDAN aldik, o yuzden calisir
    };
}

macro_rules! no_pollution {
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
trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! impl_max {
    ( $( $t:ty ),+ $(,)? ) => {
        $(
            impl MaxValue for $t {
                fn max_value() -> Self { <$t>::MAX }
            }
        )+
    };
}
impl_max!(u8, u16, u32, i8, i16, i32);

// --- 7) TT MUNCHER: ozyinelemeli makro, token'lari tek tek yer ---
macro_rules! token_say {
    () => { 0 };                                             // taban durum
    ($ilk:tt $($geri:tt)*) => { 1 + token_say!($($geri)*) };  // birini ye, kalani devret
}

// --- 8) stringify!: ismi METIN olarak kullanmak ---
macro_rules! print_and_eval {
    ($ifade:expr) => {
        println!("{:>18} = {}", stringify!($ifade), $ifade);
    };
}

fn main() {
    hello!();
    hello!("Mars");
    hello!["kose parantez de olur"];      // ( ) [ ] { } ucu de aynidir

    let sayac: Counter = 42;                // makronun urettigi tip takma adi
    println!("Counter = {}", sayac);

    // hijyen
    let mut x = 42;
    increment!(x);
    assert_eq!(x, 43);
    no_pollution!();
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
    println!("u8::max_value  = {}", <u8 as MaxValue>::max_value());
    println!("i32::max_value = {}", <i32 as MaxValue>::max_value());

    // TT muncher - tt en ilkel yapitasi: tek token ya da parantezli grup
    println!("token_say!()      = {}", token_say!());
    println!("token_say!(a b c) = {}", token_say!(a b c));
    println!("token_say!(1 + 2) = {}   (uc token: 1, +, 2)", token_say!(1 + 2));

    // stringify
    print_and_eval!(2 + 3 * 4);
    print_and_eval!(sayilar.len());

    // NE ZAMAN MAKRO: degisken sayida arguman, tekrar eden impl, isimleri metne cevirme.
    // Bunlarin disinda FONKSIYON yazin - makro hata mesajlarini ve IDE destegini bozar.
    // Uretilen kodu gormek icin: cargo install cargo-expand && cargo expand
}

#[cfg(test)]
mod tests {
    // makrolar metinsel kapsamda oldugu icin use gerekmiyor

    #[test]
    fn empty_vec() {
        let v: Vec<u32> = avec![];
        assert!(v.is_empty());
    }

    #[test]
    fn vec_with_elements() {
        let v: Vec<u32> = avec![42, 43];
        assert_eq!(v.len(), 2);
        assert_eq!(v[1], 43);
    }

    #[test]
    fn repeated_vec() {
        let v: Vec<u32> = avec![7; 3];
        assert_eq!(v, vec![7, 7, 7]);
    }

    #[test]
    fn paren_trap() {
        assert_eq!(kare_expr!(2 + 3), 25);   // expr: ifade butun kalir
        assert_eq!(kare_tt!(2 + 3), 11);     // tt: token kopyalanir, tuzak geri gelir
    }
}
