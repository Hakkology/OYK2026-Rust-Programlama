// tests/ altindaki her dosya AYRI bir crate olarak derlenir ve kutuphaneyi
// DISARIDAN kullanir. Yani buradan yalnizca `pub` olanlara erisebilirsiniz.
use ders1::Bill;

#[test]
fn disaridan_kullanim() {
    let mut bill = Bill::new();
    bill.add("kahve", 8_500);
    bill.add("tatli", 15_000);
    assert_eq!(bill.total(), 23_500);
    assert_eq!(bill.split(2), 11_750);
}

#[test]
fn varsayilan_bos() {
    // #[derive(Default)] sayesinde
    assert_eq!(Bill::default(), Bill::new());
}
