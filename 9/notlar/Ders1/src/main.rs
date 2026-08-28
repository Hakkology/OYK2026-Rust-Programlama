// Ikili (binary): kutuphaneyi DISARIDAN kullanir - tipki bir musteri gibi.
use ders1::Bill;

fn main() {
    let mut bill = Bill::new();
    bill.add("mercimek corbasi", 12_000);
    bill.add("kuru fasulye", 18_500);
    bill.add("ayran", 4_000);

    println!("{} kalem", bill.len());
    println!("toplam    : {:.2} TL", bill.total() as f64 / 100.0);
    println!("4 kisiye  : {:.2} TL", bill.split(4) as f64 / 100.0);

    // Bu satir yalnizca dev profilinde calisir (release'te debug_assertions kapali)
    if cfg!(debug_assertions) {
        println!("[dev profili] debug_assertions ACIK");
    } else {
        println!("[release profili] debug_assertions KAPALI");
    }
}
