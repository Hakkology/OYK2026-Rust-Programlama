// Gun 6 / Ders 1 - Generics ve Monomorphization
// rustc main.rs && ./main
//
// Monomorphization'i GOZLE gormek icin:
//   rustc main.rs && nm -C main | grep strongest
// -> strongest UC KEZ gorunur: i32, f64 ve char icin ayri makine kodu.
// (-O ile derlerseniz hepsi inline olur ve hicbiri gorunmez; o da ayri bir kanit)

use std::fmt::Debug;

// ---------------------------------------------------------------
// 1) PROBLEM: iki fonksiyon, tek fark tip
// ---------------------------------------------------------------
fn strongest_i32(stats: &[i32]) -> i32 {
    let mut en = stats[0];
    for &x in stats {
        if x > en { en = x; }
    }
    en
}

fn strongest_f64(stats: &[f64]) -> f64 {
    let mut en = stats[0];
    for &x in stats {
        if x > en { en = x; }
    }
    en
}

// ---------------------------------------------------------------
// 2) COZUM: generic. Ama BOUND olmadan calismaz.
// ---------------------------------------------------------------
// fn strongest_bozuk<T>(stats: &[T]) -> T {
//     let mut en = stats[0];
//     for &x in stats {
//         if x > en { en = x; }     // E0369: binary operation `>` cannot be
//     }                             //        applied to type `T`
//     en
// }
// Derleyici T'nin ne oldugunu bilmiyor, dolayisiyla ne YAPABILECEGINI de bilmiyor.

// T: PartialOrd = "karsilastirilabilir olacak", T: Copy = "kopyalanabilir olacak"
fn strongest<T: PartialOrd + Copy>(stats: &[T]) -> T {
    let mut en = stats[0];
    for &x in stats {
        if x > en { en = x; }
    }
    en
}

// ---------------------------------------------------------------
// 3) GENERIC STRUCT: envanter yuvasi
// ---------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Potion {
    heal: i32,
}

#[derive(Debug, Clone, Copy)]
struct Sword {
    damage: i32,
}

#[derive(Debug)]
struct Slot<T> {
    label: String,
    item: T,
}

// impl<T> ... : BUTUN T'ler icin
impl<T> Slot<T> {
    fn new(label: &str, item: T) -> Self {
        Slot { label: label.to_string(), item }
    }
    fn item(&self) -> &T {
        &self.item
    }
}

// impl Slot<Potion> : SADECE iksir yuvasinda olan metot. Kosullu impl.
impl Slot<Potion> {
    fn drink(&self) -> String {
        format!("{} icildi, {} can geldi", self.label, self.item.heal)
    }
}

// ikinci kosullu impl: sadece kilic yuvasinda olan metot
impl Slot<Sword> {
    fn swing(&self) -> String {
        format!("{} savruldu, {} hasar", self.label, self.item.damage)
    }
}

// bound'lu impl: sadece Debug olan T'ler bu metodu alir
impl<T: Debug> Slot<T> {
    fn inspect(&self) -> String {
        format!("{:<8} -> {:?}", self.label, self.item)
    }
}

// ---------------------------------------------------------------
// 4) BIRDEN COK GENERIC PARAMETRE: kusanim (silah + zirh)
// ---------------------------------------------------------------
#[derive(Debug)]
struct Loadout<W, A> {
    weapon: W,
    armor: A,
}

impl<W, A> Loadout<W, A> {
    // iki eli degistirir: tipler de yer degistirir
    fn swap_hands(self) -> Loadout<A, W> {
        Loadout { weapon: self.armor, armor: self.weapon }
    }
}

// ---------------------------------------------------------------
// 5) where: bound'lar uzayinca imzayi okunur tutar (davranis AYNI)
// ---------------------------------------------------------------
fn party_report<T>(stats: &[T]) -> String
where
    T: Debug + PartialOrd + Copy,
{
    format!("{} birim, en gucluse {:?}", stats.len(), strongest(stats))
}

// ---------------------------------------------------------------
// 6) CONST GENERICS: kadro BOYUTU da generic olabilir (Rust 1.51+)
// ---------------------------------------------------------------
fn first_member<T: Copy, const N: usize>(party: &[T; N]) -> T {
    party[0]
}

fn party_size<T, const N: usize>(_party: &[T; N]) -> usize {
    N                       // uzunluk DERLEME zamaninda biliniyor
}

fn main() {
    let attack_powers = [12, 18, 55, 3];         // okcu, sovalye, ejderha, sifaci
    let crit_multipliers = [1.5, 2.0, 1.25];

    println!("-- 1) tekrar eden iki fonksiyon --");
    println!("  strongest_i32 : {}", strongest_i32(&attack_powers));
    println!("  strongest_f64 : {}", strongest_f64(&crit_multipliers));

    println!("-- 2) tek generic fonksiyon ikisini de yapiyor --");
    println!("  {} / {}", strongest(&attack_powers), strongest(&crit_multipliers));
    println!("  harflerde bile: {}", strongest(&['e', 'j', 'd', 'e', 'r']));

    println!("-- 3) generic struct --");
    let potion_slot = Slot::new("potion_slot", Potion { heal: 40 });
    let sword_slot = Slot::new("sword_slot", Sword { damage: 25 });
    let note_slot = Slot::new("note", "cursed");
    println!("  {}", potion_slot.inspect());
    println!("  {}", sword_slot.inspect());
    println!("  {}", note_slot.inspect());
    println!("  item(): {:?}", potion_slot.item());

    // sadece Slot<Potion>'da olan metot
    println!("  {}", potion_slot.drink());
    println!("  {}", sword_slot.swing());
    // sword_slot.drink();
    //   E0599: no method named `drink` found for struct `Slot<Sword>`
    //   -> impl Slot<Potion> yazdik, kilic yuvasinda (Slot<Sword>) boyle bir metot YOK

    println!("-- 4) iki tipli generic --");
    let loadout = Loadout { weapon: Sword { damage: 25 }, armor: "chainmail" };
    println!("  {:?}", loadout);
    println!("  {:?}", loadout.swap_hands());

    println!("-- 5) where ile ayni is --");
    println!("  {}", party_report(&attack_powers));

    println!("-- 6) const generics --");
    let trio = [12, 18, 55];
    let quintet = [1.5, 2.0, 1.25, 3.0, 1.1];
    println!("  ilk / kadro: {} {}  |  {} {}",
        first_member(&trio), party_size(&trio),
        first_member(&quintet), party_size(&quintet));

    // MONOMORPHIZATION: derleyici her somut tip icin AYRI fonksiyon uretti.
    // Calisma zamaninda generic diye bir sey yok - maliyet SIFIR.
    // Kanit:  rustc main.rs && nm -C main | grep strongest
}
