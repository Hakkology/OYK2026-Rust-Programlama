// Gun 1 / Lab - Desenler, sicaklik tablosu, carpim tablosu
// rustc lab-1.rs && ./lab-1
//
// Her gorevde TODO'lari doldurun; ustundeki ORNEK nasil calistigini gosteriyor.

// Iskelet kod: TODO'lar doldurulana kadar kullanilmayan degisken/import uyarilari normal.
#![allow(unused)]

fn main() {
    lab_1_yildizlar();
    lab_2_sicaklik_tablosu();
    lab_3_carpim_tablosu();
}

// ---------------------------------------------------------------------------
// LAB 1 - Yildiz desenleri
// ---------------------------------------------------------------------------
fn lab_1_yildizlar() {
    println!("-- lab 1 --");

    // ORNEK: sol ucgen (n = 5)
    // *
    // **
    // ***
    // ****
    // *****
    for i in 1..=5 {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
    println!();

    // TODO 1a: ters ucgen
    // *****
    // ****
    // ***
    // **
    // *

    // TODO 1b: piramit
    //     *
    //    ***
    //   *****
    //  *******
    // *********
    // ipucu: once bosluk sonra yildiz; bosluk 5-i, yildiz 2*i-1

    // TODO 1c: ici bos kare (n = 5)
    // *****
    // *   *
    // *   *
    // *   *
    // *****
    // ipucu: ilk/son satir ya da ilk/son sutun ise yildiz, degilse bosluk
}

// ---------------------------------------------------------------------------
// LAB 2 - Sicaklik donusum tablosu
// C -> F : c * 9.0 / 5.0 + 32.0
// F -> C : (f - 32.0) * 5.0 / 9.0
// 9/5 tamsayi bolmesi 1 verir, derlenir ama sessiz yanlis
// ---------------------------------------------------------------------------
fn lab_2_sicaklik_tablosu() {
    println!("-- lab 2 --");

    println!("{:>7.1} (32.0)", c_to_f(0.0));
    println!("{:>7.1} (212.0)", c_to_f(100.0));
    println!("{:>7.1} (-40.0)", c_to_f(-40.0));
    println!("{:>7.1} (0.0)", f_to_c(32.0));
    println!("{:>7.1} (100.0)", f_to_c(212.0));

    // TODO 2a: c_to_f ve f_to_c fonksiyonlarini doldurun

    // TODO 2b: -40'tan 100'e 20'ser artan tablo
    //     C       F
    //   -40   -40.0
    //   -20    -4.0
    //     0    32.0
    // ipucu: for c in (-40..=100).step_by(20) { let c = c as f64; ... }

    // TODO 2c: donma (0 C) ve kaynama (100 C) satirlarini "<--" ile isaretleyin
}

fn c_to_f(c: f64) -> f64 {
    c // TODO
}

fn f_to_c(f: f64) -> f64 {
    f // TODO
}

// ---------------------------------------------------------------------------
// LAB 3 - Carpim tablosu
// {:>4} sagdan 4 karakter genislik
// ---------------------------------------------------------------------------
fn lab_3_carpim_tablosu() {
    println!("-- lab 3 --");

    // ORNEK: tek satir
    for j in 1..=10 {
        print!("{:>4}", 3 * j);
    }
    println!();
    println!();

    // TODO 3a: 1'den 10'a tam carpim tablosu (10 satir x 10 sutun)

    // TODO 3b: ust satira ve sol sutuna baslik ekleyin
    //        1   2   3   4 ...
    //   1    1   2   3   4
    //   2    2   4   6   8

    // TODO 3c: sadece kosegenin altini yazdirin (j <= i olanlar)
    //   1    1
    //   2    2   4
    //   3    3   6   9
}
