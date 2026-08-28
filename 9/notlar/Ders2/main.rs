// Gun 9 / Ders 2 - Unsafe Rust
// rustc main.rs && ./main
//
// Sekiz gundur "derleyici izin vermiyor" dedik. Bugun izin veren kapiyi aciyoruz -
// ve neden cogu zaman ACMAMANIZ gerektigini konusuyoruz.

use std::slice;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

// ---------------------------------------------------------------
// 4) GUVENSIZ ICERIK, GUVENLI ARAYUZ - kanonik ornek
// ---------------------------------------------------------------
// Bir dilimi ikiye bolup IKI MUT referans dondurmek istiyoruz.
// Guvenli Rust'ta yazilamaz:
//
//   fn split_at_mut(v: &mut [i32], orta: usize) -> (&mut [i32], &mut [i32]) {
//       (&mut v[..orta], &mut v[orta..])
//   }
//     E0499: cannot borrow `*v` as mutable more than once at a time
//
// Derleyici HAKSIZ degil, sadece yetersiz: iki dilimin CAKISMADIGINI
// ispat edemiyor. Biz biliyoruz. Bilgiyi unsafe ile ona veriyoruz.
fn split_at_mut(v: &mut [i32], orta: usize) -> (&mut [i32], &mut [i32]) {
    let uzunluk = v.len();
    let bas = v.as_mut_ptr();
    assert!(orta <= uzunluk);                 // SOZUN sarti: once kontrol et

    unsafe {
        // SAFETY: orta <= uzunluk oldugunu yukarida dogruladik.
        // Iki dilim [0, orta) ve [orta, uzunluk) - kesisimleri BOS,
        // dolayisiyla iki &mut ayni bellege bakmiyor.
        (
            slice::from_raw_parts_mut(bas, orta),
            slice::from_raw_parts_mut(bas.add(orta), uzunluk - orta),
        )
    }
}

// ---------------------------------------------------------------
// 6) UNSAFE TRAIT
// ---------------------------------------------------------------
// Ham pointer Send degildir. "Bu pointer'i baska thread'e goturmek guvenli"
// diyorsak bunu BIZ garanti ederiz - derleyici degil.
struct Handle {
    ptr: *const u8,
}

impl Handle {
    // SAFETY sozunu tek yerde tutuyoruz: cagiran unsafe yazmiyor.
    fn first(&self) -> u8 {
        unsafe { *self.ptr }
    }
}

// SAFETY: bu ornek icin ptr sabit, salt okunur bir veriyi gosteriyor ve
// program boyu gecerli. Gercek kodda bu cumleyi ISPATLAMAK zorundasiniz.
unsafe impl Send for Handle {}

// ---------------------------------------------------------------
// 7) FFI - baska dille konusmak
// ---------------------------------------------------------------
extern "C" {
    fn abs(input: i32) -> i32;                // C standart kutuphanesinden
}

// ---------------------------------------------------------------
// 5) GLOBAL DEGISKEN: static mut yerine ne kullanmali
// ---------------------------------------------------------------
static SAYAC: AtomicU32 = AtomicU32::new(0);          // thread-safe sayac
static AYAR: OnceLock<String> = OnceLock::new();      // bir kez yazilir

fn main() {
    println!("-- 1) ham pointer olusturmak GUVENLI --");
    let mut sayi = 42;
    let p1 = &sayi as *const i32;              // *const T
    let p2 = &mut sayi as *mut i32;            // *mut T
    println!("  adres: {:p}", p1);
    // Olusturmak serbest, cunku henuz bir sey OKUMADIK.
    // Referanstan farklari: null olabilir, sarkabilir, ayni anda
    // birden cok mut pointer olabilir, otomatik temizlenmez.

    println!("-- 2) DEREFERENCE etmek unsafe --");
    // println!("{}", *p1);
    //   E0133: dereference of raw pointer is unsafe and requires unsafe function or block
    unsafe {
        // SAFETY: p1 ve p2 az once gecerli bir yerel degiskenden uretildi,
        // hizalanmis ve bu kapsamda yasiyor.
        println!("  *p1 = {}", *p1);
        *p2 += 1;
        println!("  *p2 = {} (degistirdik)", *p2);
    }
    println!("  sayi = {}", sayi);

    println!("-- 3) tehlike burada baslar --");
    let sarkan: *const i32 = {
        let gecici = 7;
        &gecici as *const i32                  // gecici BURADA dusuyor
    };
    println!("  sarkan pointer olusturuldu: {:p}", sarkan);
    println!("  onu okumak TANIMSIZ DAVRANIS olurdu - okumuyoruz.");
    // unsafe { println!("{}", *sarkan); }  <- kesinlikle yapmayin
    let bos: *const i32 = std::ptr::null();
    println!("  null mu: {}", bos.is_null());  // kontrol etmek guvenli

    println!("-- 4) guvensiz icerik, guvenli arayuz --");
    let mut veri = [1, 2, 3, 4, 5, 6];
    let (sol, sag) = split_at_mut(&mut veri, 3);
    sol[0] = 100;
    sag[0] = 200;
    println!("  sol {:?} | sag {:?}", sol, sag);
    println!("  butun: {:?}", veri);
    println!("  Cagiran unsafe YAZMADI. std'nin yaptigi da tam olarak bu.");

    println!("-- 5) global durum --");
    SAYAC.fetch_add(1, Ordering::Relaxed);
    SAYAC.fetch_add(1, Ordering::Relaxed);
    println!("  atomik sayac: {}", SAYAC.load(Ordering::Relaxed));
    AYAR.set(String::from("uretim")).unwrap();
    println!("  OnceLock ayar: {}", AYAR.get().unwrap());
    // `static mut` de var ama ARTIK KULLANMAYIN: ona referans almak
    // Rust 2024'te hata, 2021'de uyari. Yerine Atomic / OnceLock / Mutex.

    println!("-- 6) unsafe impl Send --");
    let veri: &'static [u8] = b"sabit";
    let h = Handle { ptr: veri.as_ptr() };
    // DIKKAT: closure'in TUM struct'i yakalamasi gerekiyor. Sadece h.ptr'yi
    // yakalasaydi (Rust 2021 alan bazli yakalar) unsafe impl Send devreye girmezdi:
    //   E0277: `*const u8` cannot be sent between threads safely
    // Metot cagirmak butun h'yi yakalatiyor.
    let sonuc = std::thread::spawn(move || h.first()).join().unwrap();
    println!("  thread'e gecen pointer'in ilk bayti: {}", sonuc as char);

    println!("-- 7) FFI --");
    // SAFETY: abs C tarafinda tanimli, i32 alip i32 donduruyor.
    let d = unsafe { abs(-13) };
    println!("  C'nin abs(-13) = {}", d);
    println!("  extern fonksiyonlar HER ZAMAN unsafe: derleyici oteki tarafi goremez.");

    println!("-- 8) kural --");
    println!("  unsafe bloklari KUCUK tutun, her birine // SAFETY: yazin,");
    println!("  disariya GUVENLI arayuz verin. unsafe bir kacis degil, bir SOZDUR.");
}
