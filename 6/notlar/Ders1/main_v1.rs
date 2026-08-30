// Gun 6 / Ders 1 (v1) - Generics ve Monomorphization
// rustc main_v1.rs && ./main_v1
//
// Ayni ders, farkli dunya: sehir ulasim agi. Vapur, metro, tramvay.
//
// Monomorphization'i GOZLE gormek icin:
//   rustc main_v1.rs && nm -C main_v1 | grep longest

use std::fmt::Debug;

// ---------------------------------------------------------------
// 1) PROBLEM: iki fonksiyon, tek fark tip
// ---------------------------------------------------------------
fn longest_i32(sureler: &[i32]) -> i32 {
    let mut en = sureler[0];
    for &x in sureler {
        if x > en { en = x; }
    }
    en
}

fn longest_f64(sureler: &[f64]) -> f64 {
    let mut en = sureler[0];
    for &x in sureler {
        if x > en { en = x; }
    }
    en
}
// Govdeleri harfi harfine ayni. Ucuncu bir tip gelince ucuncu kopya mi?

// ---------------------------------------------------------------
// 2) COZUM: generic - ama BOUND olmadan calismaz
// ---------------------------------------------------------------
// fn longest_bozuk<T>(sureler: &[T]) -> T {
//     let mut en = sureler[0];
//     for &x in sureler {
//         if x > en { en = x; }     // E0369: binary operation `>` cannot be
//     }                             //        applied to type `T`
//     en
// }
// T: PartialOrd = "karsilastirilabilir olacak", T: Copy = "kopyalanabilir olacak"
fn longest<T: PartialOrd + Copy>(sureler: &[T]) -> T {
    let mut en = sureler[0];
    for &x in sureler {
        if x > en { en = x; }
    }
    en
}

// ---------------------------------------------------------------
// 3) GENERIC STRUCT: peron - icine hangi araci koydugunuz sizin seciminiz
// ---------------------------------------------------------------
#[derive(Debug)]
struct Ferry {
    deck_count: u8,
}

#[derive(Debug)]
struct Metro {
    car_count: u8,
}

#[derive(Debug)]
struct Platform<T> {
    label: String,
    vehicle: T,
}

// impl<T> ... : BUTUN T'ler icin
impl<T> Platform<T> {
    fn new(label: &str, vehicle: T) -> Self {
        Platform { label: label.to_string(), vehicle }
    }
    fn vehicle(&self) -> &T {
        &self.vehicle
    }
}

// impl Platform<Ferry> : SADECE vapur peronunda olan metot. Kosullu impl.
impl Platform<Ferry> {
    fn lower_ramp(&self) -> String {
        format!("{}: rampa indi, {} guverte", self.label, self.vehicle.deck_count)
    }
}

// ikinci kosullu impl: sadece metro peronunda
impl Platform<Metro> {
    fn open_doors(&self) -> String {
        format!("{}: {} vagonun kapilari acildi", self.label, self.vehicle.car_count)
    }
}

// bound'lu impl: sadece Debug olan T'ler bu metodu alir
impl<T: Debug> Platform<T> {
    fn inspect(&self) -> String {
        format!("{:<10} -> {:?}", self.label, self.vehicle)
    }
}

// ---------------------------------------------------------------
// 4) BIRDEN COK GENERIC PARAMETRE: aktarma (iki ayri hat)
// ---------------------------------------------------------------
#[derive(Debug)]
struct Transfer<A, B> {
    first: A,
    second: B,
}

impl<A, B> Transfer<A, B> {
    // Aktarma yonunu ters cevir: tipler de yer degistiriyor
    fn reverse(self) -> Transfer<B, A> {
        Transfer { first: self.second, second: self.first }
    }
}

// ---------------------------------------------------------------
// 5) where: bound'lar uzayinca imzayi okunur tutar (davranis AYNI)
// ---------------------------------------------------------------
fn report<T>(items: &[T]) -> String
where
    T: Debug + Clone,
{
    format!("{} kayit: {:?}", items.len(), items.to_vec())
}

// ---------------------------------------------------------------
// 6) CONST GENERICS: filo BOYUTU da generic olabilir (Rust 1.51+)
// ---------------------------------------------------------------
struct Fleet<const N: usize> {
    plates: [u32; N],
}

impl<const N: usize> Fleet<N> {
    fn size(&self) -> usize {
        N
    }
    fn total(&self) -> u32 {
        self.plates.iter().sum()
    }
}

// ---------------------------------------------------------------
// 7) BOUND'SUZ GENERIC ne ise yarar
// ---------------------------------------------------------------
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    println!("-- 1) tekrar eden iki fonksiyon --");
    let dakikalar = [12, 7, 25, 18];
    let ucretler = [7.5, 12.0, 9.25];
    println!("  longest_i32 : {} dk", longest_i32(&dakikalar));
    println!("  longest_f64 : {} TL", longest_f64(&ucretler));

    println!("-- 2) tek generic fonksiyon ikisini de yapiyor --");
    println!("  {} dk / {} TL", longest(&dakikalar), longest(&ucretler));
    println!("  hat harflerinde bile: {}", longest(&['A', 'F', 'C']));

    println!("-- 3) generic struct --");
    let ferry_platform = Platform::new("Konak", Ferry { deck_count: 2 });
    let metro_platform = Platform::new("Halkapinar", Metro { car_count: 5 });
    let note_platform = Platform::new("Duyuru", "bakim var");
    println!("  {}", ferry_platform.inspect());
    println!("  {}", metro_platform.inspect());
    println!("  {}", note_platform.inspect());
    println!("  vehicle(): {:?}", ferry_platform.vehicle());

    // sadece Platform<Ferry>'de olan metot
    println!("  {}", ferry_platform.lower_ramp());
    println!("  {}", metro_platform.open_doors());
    // metro_platform.lower_ramp();
    //   E0599: no method named `lower_ramp` found for struct `Platform<Metro>`
    //   -> impl Platform<Ferry> yazdik, metro peronunda boyle bir metot YOK

    println!("-- 4) iki tipli generic --");
    let aktarma = Transfer { first: Ferry { deck_count: 2 }, second: Metro { car_count: 5 } };
    println!("  {:?}", aktarma);
    let ters = aktarma.reverse();
    println!("  ters: {:?}", ters);
    println!("  tipler de yer degistirdi: Transfer<Ferry, Metro> -> Transfer<Metro, Ferry>");

    println!("-- 5) where --");
    println!("  {}", report(&dakikalar));

    println!("-- 6) const generics --");
    let kucuk: Fleet<3> = Fleet { plates: [35, 41, 77] };
    let buyuk: Fleet<5> = Fleet { plates: [10, 20, 30, 40, 50] };
    println!("  {} araclik filo, plaka toplami {}", kucuk.size(), kucuk.total());
    println!("  {} araclik filo, plaka toplami {}", buyuk.size(), buyuk.total());
    println!("  Fleet<3> ile Fleet<5> AYRI tiptir - N derleme zamaninda bilinir.");

    println!("-- 7) bound'suz generic --");
    println!("  identity(42) = {}", identity(42));
    println!("  Bu fonksiyon degerinizle hicbir sey YAPAMAZ - sadece tasiyabilir.");
    println!("  Kisit gibi gorunuyor ama aslinda GARANTI: bozamaz.");
}
