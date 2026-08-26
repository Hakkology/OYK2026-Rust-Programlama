// ENTEGRASYON TESTI: tests/ dizini sadece KUTUPHANE crate'ini gorur
// ve sadece PUBLIC API'ye erisir. main.rs'e gomulu kod buradan test edilemez.
use ders3::{in_range, parse, summary, table};

#[test]
fn public_api_is_flat() {
    // ic yapi ders3::telemetry::parser::parse ama biz kisa adi kullaniyoruz
    assert!(parse("sicaklik=-63.2").is_ok());
    assert!(in_range(-63.2));
}

#[test]
fn report_pipeline_works() {
    let readings = vec![parse("sicaklik=-60").unwrap(), parse("sicaklik=-40").unwrap()];
    assert!(summary(&readings).contains("2 olcum"));
    assert_eq!(table(&readings).lines().count(), 4);
}

#[test]
fn private_items_are_not_reachable() {
    // ders3::calibrate(1.0);              // derlenmez: disari acilmadi
    // ders3::telemetry::validation::LOWER // derlenmez: telemetry private
    assert!(parse("nem=40").is_err());
}
