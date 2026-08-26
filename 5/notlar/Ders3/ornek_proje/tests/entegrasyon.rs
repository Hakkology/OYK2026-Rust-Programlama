// tests/ dizini SADECE kutuphane crate'ini gorur ve sadece PUBLIC API'ye erisir.
// Bu yuzden lib.rs + main.rs ayrimi onemli.
use ornek_proje::{araligda, parse};

#[test]
fn gecerli_satir_ayristirilir() {
    let r = parse("sicaklik=-63.2").expect("gecerli satir");
    assert!((r.deger() - (-63.2)).abs() < 0.0001);   // f64 karsilastirmasi epsilon ile
}

#[test]
fn bozuk_satirlar_hata_verir() {
    assert!(parse("nem=40").is_err());
    assert!(parse("sicaklik=abc").is_err());
    assert!(parse("sicaklik=999").is_err());
}

#[test]
fn re_export_calisiyor() {
    assert!(araligda(-63.2));
}
