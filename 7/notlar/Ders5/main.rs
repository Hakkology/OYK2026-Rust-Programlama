// Gun 7 / Ders 5 - Closure'larla Calismak
// rustc main.rs && ./main
//
// Kural motoru: buro her dosya icin farkli suzme kurallari kuruyor,
// kurallari saklıyor, zincirliyor ve calisma zamaninda seciyor.

#[derive(Debug, Clone)]
struct Lead {
    note: String,
    weight: u8,
    informant: String,
}

// ---------------------------------------------------------------
// 1) CLOSURE DONDURMEK
// ---------------------------------------------------------------
// impl Fn: TEK somut tip. Statik dispatch, hizli. (Gun 6, Ders 2)
fn min_weight_rule(threshold: u8) -> impl Fn(&Lead) -> bool {
    move |l: &Lead| l.weight >= threshold
}

// Cevre YAKALAYAN iki closure'i impl Fn ile donduremeyiz - iki ayri tiptirler:
// fn rule_for_broken(mode: &str, threshold: u8) -> impl Fn(&Lead) -> bool {
//     if mode == "strict" { move |l: &Lead| l.weight >= threshold }
//     else { move |l: &Lead| l.weight >= threshold / 2 }
// }
//   E0308: `if` and `else` have incompatible types
//   "expected closure, found a different closure"
//   Gun 6'da Dragon/Archer ile ayni duvara carpmistik; cozum de ayni: Box<dyn>.
//
// DIKKAT: hicbir sey yakalamayan iki closure ile ayni sey DERLENIR - cunku
// ikisi de fn(&Lead) -> bool pointer'ina donusur (Ders 4) ve ortak tipte bulusurlar.

// Box<dyn Fn>: if/else ile FARKLI closure'lar donebiliyoruz.
fn rule_for(mode: &str) -> Box<dyn Fn(&Lead) -> bool> {
    match mode {
        "strict" => Box::new(|l: &Lead| l.weight >= 8),
        "loose" => Box::new(|l: &Lead| l.weight >= 3),
        _ => Box::new(|_l: &Lead| true),
    }
}

// ---------------------------------------------------------------
// 2) CLOSURE'I STRUCT ICINDE SAKLAMAK
// ---------------------------------------------------------------
// (a) generic alan: tek bir kural tipi, sifir maliyet
struct Screen<F>
where
    F: Fn(&Lead) -> bool,
{
    name: String,
    rule: F,
}

impl<F: Fn(&Lead) -> bool> Screen<F> {
    fn apply(&self, leads: &[Lead]) -> usize {
        leads.iter().filter(|l| (self.rule)(l)).count()
    }
}

// (b) Box<dyn Fn> alan: farkli kurallar ayni tipte saklanabiliyor
struct RuleBook {
    rules: Vec<(String, Box<dyn Fn(&Lead) -> bool>)>,
}

impl RuleBook {
    fn new() -> RuleBook {
        RuleBook { rules: Vec::new() }
    }

    fn add(&mut self, name: &str, rule: Box<dyn Fn(&Lead) -> bool>) {
        self.rules.push((name.to_string(), rule));
    }

    // Tum kurallardan gecen ipuclari
    fn passing<'a>(&self, leads: &'a [Lead]) -> Vec<&'a Lead> {
        leads
            .iter()
            .filter(|l| self.rules.iter().all(|(_, rule)| rule(l)))
            .collect()
    }
}

fn main() {
    let leads = vec![
        Lead { note: String::from("otoparktaki bilet"), weight: 8, informant: String::from("bekci") },
        Lead { note: String::from("isimsiz telefon"), weight: 3, informant: String::from("bilinmiyor") },
        Lead { note: String::from("plaka kaydi"), weight: 9, informant: String::from("trafik") },
        Lead { note: String::from("dedikodu"), weight: 2, informant: String::from("bilinmiyor") },
    ];

    println!("-- 1) closure donduren fonksiyon --");
    let strong = min_weight_rule(6);
    let any_lead = min_weight_rule(0);
    println!("  esik 6 gecen: {}", leads.iter().filter(|l| strong(l)).count());
    println!("  esik 0 gecen: {}", leads.iter().filter(|l| any_lead(l)).count());

    println!("-- 2) calisma zamaninda kural secmek --");
    for mode in ["strict", "loose", "off"] {
        let rule = rule_for(mode);                       // Box<dyn Fn>
        println!("  {:<7} -> {} ipucu", mode, leads.iter().filter(|l| rule(l)).count());
    }

    println!("-- 3) struct icinde closure --");
    let named = Screen { name: String::from("guvenilir muhbir"), rule: |l: &Lead| l.informant != "bilinmiyor" };
    println!("  {}: {} ipucu", named.name, named.apply(&leads));

    let mut book = RuleBook::new();
    book.add("agirlik >= 3", Box::new(|l| l.weight >= 3));
    book.add("muhbir belli", Box::new(|l| l.informant != "bilinmiyor"));
    let passing = book.passing(&leads);
    println!("  {} kuraldan gecenler: {:?}",
        book.rules.len(),
        passing.iter().map(|l| l.note.as_str()).collect::<Vec<_>>());

    println!("-- 4) iterator kombinatorleri (Gun 4'ten tanidik) --");
    // Kombinatorler TEMBELDIR: collect/sum/count cagrilana kadar hicbir sey olmaz.
    let notes: Vec<String> = leads
        .iter()
        .filter(|l| l.weight >= 5)
        .map(|l| format!("{} ({})", l.note, l.weight))
        .collect();
    println!("  filter + map : {:?}", notes);

    let total: u32 = leads.iter().map(|l| l.weight as u32).sum();
    println!("  sum          : {}", total);

    let heaviest = leads.iter().max_by_key(|l| l.weight);
    println!("  max_by_key   : {:?}", heaviest.map(|l| &l.note));

    let mut sorted: Vec<&Lead> = leads.iter().collect();
    sorted.sort_by_key(|l| std::cmp::Reverse(l.weight));
    println!("  sort_by_key  : {:?}", sorted.iter().map(|l| l.weight).collect::<Vec<_>>());

    println!("  any / all    : {} / {}",
        leads.iter().any(|l| l.weight > 8),
        leads.iter().all(|l| l.weight > 1));

    println!("-- 5) Option uzerinde kombinatorler --");
    let found = leads.iter().find(|l| l.note.contains("plaka"));
    // map: Some'in ICINI donusturur, None'a dokunmaz
    println!("  map          : {:?}", found.map(|l| l.weight));
    // unwrap_or_else: closure SADECE None ise calisir
    println!("  unwrap_or_else: {}", found.map(|l| l.weight).unwrap_or_else(|| 0));
    // and_then: Option donduren bir islemi zincirler
    let first_word = found.and_then(|l| l.note.split(' ').next());
    println!("  and_then     : {:?}", first_word);
    // filter: Some'i kosula sokar
    println!("  filter       : {:?}", found.filter(|l| l.weight > 9).map(|l| &l.note));
    // ok_or: Option -> Result (Gun 5)
    let as_result: Result<&Lead, &str> = found.ok_or("plaka ipucu yok");
    println!("  ok_or        : {:?}", as_result.map(|l| &l.note));
}
