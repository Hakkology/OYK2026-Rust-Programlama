// Gun 6 / Ders 2 - Trait Tanimi, Varsayilan Metotlar ve Bound'lar
// rustc main.rs && ./main
//
// Dunya: kucuk bir savas simulasyonu. Okcu, sovalye, ejderha, sifaci...
// Hepsi tamamen farkli seyler yapar ama hepsinin ortak bir SOZLESMESI vardir:
// canlari vardir, vurus gucleri vardir, savas narasi atarlar.

use std::fmt::Debug;
use std::mem::size_of;

// ---------------------------------------------------------------
// 1) TRAIT = SOZLESME
// ---------------------------------------------------------------
trait Unit {
    // zorunlu: implemente eden herkes yazmak ZORUNDA
    fn name(&self) -> &str;
    fn hp(&self) -> i32;
    fn attack_power(&self) -> i32;

    // VARSAYILAN metot: gövdesi burada. Isteyen ezer, istemeyen bedava alir.
    fn battle_cry(&self) -> String {
        format!("{} savasa hazir!", self.name())
    }

    // varsayilan metot ZORUNLU metotlari cagirabilir
    fn is_alive(&self) -> bool {
        self.hp() > 0
    }

    fn status(&self) -> String {
        let durum = if self.is_alive() { "ayakta" } else { "dusmus" };
        format!("{:<10} {:>4} can  {:>3} vurus  [{}]", self.name(), self.hp(), self.attack_power(), durum)
    }
}

struct Archer {
    hp: i32,
    arrows: u32,
}

struct Knight {
    hp: i32,
    armor: i32,
}

struct Dragon {
    hp: i32,
    rage: i32,
}

struct Healer;                              // alani olmayan tip de olur

// Trait metotlari ile TIPIN KENDI metotlari (inherent) bir arada yasar.
// Bu metot trait'e ait degil, sadece Archer'da var.
impl Archer {
    fn quiver(&self) -> String {
        format!("{} ok kaldi", self.arrows)
    }
}

impl Unit for Archer {
    fn name(&self) -> &str { "Archer" }
    fn hp(&self) -> i32 { self.hp }
    fn attack_power(&self) -> i32 { 12 }
    // battle_cry, is_alive, status VARSAYILAN haliyle geliyor
}

impl Unit for Knight {
    fn name(&self) -> &str { "Knight" }
    fn hp(&self) -> i32 { self.hp }
    // zirh vurusa degil, dayanikliliga katki saglar; guc sabit
    fn attack_power(&self) -> i32 { 18 }

    // varsayilani EZIYORUZ
    fn battle_cry(&self) -> String {
        format!("Knight kalkanini kaldirdi! ({} zirh)", self.armor)
    }
}

impl Unit for Dragon {
    fn name(&self) -> &str { "Dragon" }
    fn hp(&self) -> i32 { self.hp }
    fn attack_power(&self) -> i32 { 40 + self.rage }   // ofke vurusa ekleniyor

    fn battle_cry(&self) -> String {
        String::from("GRAAAH! Alevler yukseliyor!")
    }
}

impl Unit for Healer {
    fn name(&self) -> &str { "Healer" }
    fn hp(&self) -> i32 { 60 }
    fn attack_power(&self) -> i32 { 3 }
}

// ---------------------------------------------------------------
// 2) BOUND'UN UC YAZIMI - ucu de ayni sey
// ---------------------------------------------------------------
fn announce_a<T: Unit>(u: &T) -> String {
    u.battle_cry()
}

fn announce_b<T>(u: &T) -> String
where
    T: Unit,
{
    u.battle_cry()
}

fn announce_c(u: &impl Unit) -> String {
    u.battle_cry()
}

// ---------------------------------------------------------------
// 3) FARK NEREDE ORTAYA CIKIYOR: iki parametre
// ---------------------------------------------------------------
// T tek bir tip: iki arguman AYNI tip olmak zorunda (ayni sinif dusellosu)
fn duel<T: Unit>(a: &T, b: &T) -> String {
    if a.attack_power() >= b.attack_power() {
        format!("{} kazandi", a.name())
    } else {
        format!("{} kazandi", b.name())
    }
}

// impl Trait: iki arguman FARKLI tip olabilir (karma savas)
fn skirmish(a: &impl Unit, b: &impl Unit) -> String {
    format!("{} ({}) vs {} ({})", a.name(), a.attack_power(), b.name(), b.attack_power())
}

// ---------------------------------------------------------------
// 4) COKLU BOUND
// ---------------------------------------------------------------
#[derive(Debug)]
struct Goblin {
    hp: i32,
}

impl Unit for Goblin {
    fn name(&self) -> &str { "Goblin" }
    fn hp(&self) -> i32 { self.hp }
    fn attack_power(&self) -> i32 { 5 }
}

fn debug_spawn<T: Unit + Debug>(u: &T) {
    println!("  {:?} -> {}", u, u.battle_cry());
}

// ---------------------------------------------------------------
// 5) DONUSTE impl Trait: somut tip gizlenir
// ---------------------------------------------------------------
fn spawn_starter() -> impl Unit {
    Archer { hp: 80, arrows: 20 }
}

// DINAMIK dispatch: hangi metodun calisacagi vtable'dan bakilir
fn dynamic_report(u: &dyn Unit) -> String {
    u.status()
}

// STATIK dispatch: her somut tip icin ayri kod uretilir (Ders 1)
fn static_report<T: Unit>(u: &T) -> String {
    u.status()
}

// impl Unit ile YAPAMADIGIMIZ sey: iki farkli tipten birini dondurmek.
// Box<dyn Unit> hep ayni boyutta - bir pointer.
fn spawn(boss: bool) -> Box<dyn Unit> {
    if boss {
        Box::new(Dragon { hp: 500, rage: 15 })
    } else {
        Box::new(Archer { hp: 80, arrows: 20 })
    }
}

// AMA tek bir somut tip olmak zorunda:
// fn spawn(boss: bool) -> impl Unit {
//     if boss { Dragon { hp: 500, rage: 10 } } else { Goblin { hp: 20 } }
// }
//   E0308: `if` and `else` have incompatible types
//   -> derleyicinin donus degerinin BOYUTUNU bilmesi gerekiyor

fn main() {
    let archer = Archer { hp: 80, arrows: 20 };
    let knight = Knight { hp: 140, armor: 25 };
    let dragon = Dragon { hp: 500, rage: 15 };
    let healer = Healer;
    let goblin = Goblin { hp: 0 };          // dusmus birim

    println!("-- varsayilan metot vs ezilmis metot --");
    println!("  {}", archer.battle_cry());     // varsayilan gövde
    println!("  {}", healer.battle_cry());   // varsayilan gövde
    println!("  {}", knight.battle_cry());  // EZILMIS
    println!("  {}", dragon.battle_cry());  // EZILMIS

    println!("-- varsayilan metot zorunlu metodu cagiriyor --");
    println!("  {}", archer.status());
    println!("  {}", dragon.status());
    println!("  {}", goblin.status());       // hp 0 -> is_alive() false

    println!("-- trait'e ait olmayan, tipin kendi metodu --");
    println!("  {}", archer.quiver());
    // sovalye.quiver();  -> E0599: Knight'ta boyle bir metot yok

    println!("-- uc bound yazimi, ayni sonuc --");
    println!("  {}", announce_a(&archer));
    println!("  {}", announce_b(&archer));
    println!("  {}", announce_c(&archer));

    println!("-- iki parametre: T ile impl Trait farki --");
    let archer2 = Archer { hp: 75, arrows: 12 };
    println!("  ayni sinif : {}", duel(&archer, &archer2));
    println!("  karma      : {}", skirmish(&archer, &dragon));
    // duel(&okcu, &ejderha);
    //   E0308: mismatched types - T zaten Archer'a baglandi, ikincisi Dragon

    println!("-- coklu bound: Unit + Debug --");
    debug_spawn(&goblin);
    // debug_spawn(&okcu);
    //   E0277: `Archer` doesn't implement `Debug` - derive eklemediniz

    println!("-- donuste impl Trait --");
    let starter = spawn_starter();
    println!("  {}", starter.status());

    println!("-- ordu gucu --");
    let total = archer.attack_power() + knight.attack_power()
        + dragon.attack_power() + healer.attack_power();
    println!("  toplam vurus: {}", total);

    // DUVAR: dort birimi TEK BIR orduya (Vec) koyamiyoruz.
    // let ordu = vec![archer, knight, dragon, healer];
    //   E0308: mismatched types - Vec tek tip tutar, bunlar dort ayri tip
    // Trait onlari DAVRANISTA birlestirdi, TIPTE birlestirmedi.

    println!("-- duvari yikmak: Box<dyn Unit> --");
    // Vec yine TEK tip tutuyor; o tip artik Box<dyn Unit>.
    let army: Vec<Box<dyn Unit>> = vec![
        Box::new(Archer { hp: 80, arrows: 20 }),
        Box::new(Knight { hp: 140, armor: 25 }),
        Box::new(Dragon { hp: 500, rage: 15 }),
        Box::new(Healer),
    ];
    for u in &army {
        println!("  {}", u.status());
    }
    let army_power: i32 = army.iter().map(|u| u.attack_power()).sum();
    println!("  ordu vurusu: {}", army_power);

    println!("-- &dyn: sahiplik gerekmiyorsa --");
    let front: Vec<&dyn Unit> = vec![&archer, &dragon];
    for u in &front {
        println!("  {}", u.battle_cry());
    }

    println!("-- ayni satir, iki farkli dispatch --");
    println!("  statik : {}", static_report(&archer));
    println!("  dinamik: {}", dynamic_report(&archer));

    println!("-- donuste dyn: if/else artik mumkun --");
    println!("  {}", spawn(true).name());
    println!("  {}", spawn(false).name());

    println!("-- fat pointer: dyn iki pointer tasir --");
    println!("  &Archer        {:>3} bayt", size_of::<&Archer>());
    println!("  &dyn Unit      {:>3} bayt", size_of::<&dyn Unit>());
    println!("  Box<Archer>    {:>3} bayt", size_of::<Box<Archer>>());
    println!("  Box<dyn Unit>  {:>3} bayt", size_of::<Box<dyn Unit>>());
    // Gun 3'te slice ve &str de fat pointer'di (ptr + uzunluk).
    // Burada da fat pointer, ama ikinci alan vtable pointeri.
}
