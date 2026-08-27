// Gun 7 / Ders 4 - Closure'lar
// rustc main.rs && ./main
//
// Buroda "ipucu suzme" kurallari yaziyoruz. Kurallar buronun o anki
// durumunu (esik deger, yasakli muhbir listesi) bilmek zorunda.

use std::mem::size_of_val;

#[derive(Debug, Clone)]
struct Lead {
    note: String,
    weight: u8,          // 0-10 arasi guvenilirlik
}

// ---------------------------------------------------------------
// 4) CLOSURE'I FONKSIYONA GECIRMEK
// ---------------------------------------------------------------
// Her closure AYRI bir tip oldugu icin generic + trait bound kullaniyoruz.
fn filter_leads<F>(leads: &[Lead], rule: F) -> Vec<String>
where
    F: Fn(&Lead) -> bool,
{
    leads.iter().filter(|l| rule(l)).map(|l| l.note.clone()).collect()
}

// FnMut: kural cagrildikca kendi durumunu degistirebiliyor
fn audit<F>(leads: &[Lead], mut record: F)
where
    F: FnMut(&Lead),
{
    for l in leads {
        record(l);
    }
}

// FnOnce: kural sadece BIR kez calisabilir
fn finalize<F>(closer: F) -> String
where
    F: FnOnce() -> String,
{
    closer()
}

// ---------------------------------------------------------------
// 6) FN POINTER - closure degil, fonksiyon adresi
// ---------------------------------------------------------------
fn weight_over_five(l: &Lead) -> bool {
    l.weight > 5
}

// Cevre yakalamayacaksa generic'e gerek yok; tip dogrudan yazilabilir.
fn count_matching(leads: &[Lead], rule: fn(&Lead) -> bool) -> usize {
    leads.iter().filter(|l| rule(l)).count()
}

fn main() {
    let leads = vec![
        Lead { note: String::from("otoparktaki bilet"), weight: 8 },
        Lead { note: String::from("isimsiz telefon"), weight: 3 },
        Lead { note: String::from("plaka kaydi"), weight: 9 },
        Lead { note: String::from("dedikodu"), weight: 2 },
    ];

    println!("-- 1) closure cevreyi yakalar --");
    let threshold = 6;                                  // buronun o anki esigi
    let is_strong = |l: &Lead| l.weight >= threshold;   // threshold YAKALANDI
    // Ayni seyi fonksiyonla yapamayiz - fonksiyonun cevresi yoktur:
    // fn strong_fn(l: &Lead) -> bool { l.weight >= threshold }
    //   E0434: can't capture dynamic environment in a fn item
    //   -> fonksiyon ya parametre alir ya sabit kullanir.
    println!("  esik {} -> guclu ipuclari: {:?}", threshold, filter_leads(&leads, is_strong));

    println!("-- 2) uc yakalama sekli --");
    // Fn: sadece OKUYOR -> paylasimli odunc
    let banned = String::from("dedikodu");
    let allowed = |l: &Lead| l.note != banned;
    println!("  Fn      : {:?}", filter_leads(&leads, allowed));
    println!("  banned hala kullanilabilir: {}", banned);   // odunc alindi, tasinmadi

    // FnMut: DEGISTIRIYOR -> degisebilir odunc
    let mut seen = 0;
    let mut total_weight = 0u32;
    audit(&leads, |l| {
        seen += 1;
        total_weight += l.weight as u32;
    });
    println!("  FnMut   : {} ipucu, toplam agirlik {}", seen, total_weight);

    // FnOnce: SAHIPLENIYOR -> bir kez cagrilabilir
    let case_code = String::from("47-B");
    let close_case = move || format!("{} dosyasi kapatildi", case_code);
    println!("  FnOnce  : {}", finalize(close_case));
    // finalize(close_case);
    //   E0382: use of moved value - closure cagrildiginda kendini tuketti

    println!("-- 3) move: yakalamayi zorla sahiplenme yapar --");
    let detective = String::from("Alvarez");
    let sign = move || format!("imza: {}", detective);
    println!("  {}", sign());
    println!("  {}", sign());          // move ama HALA Fn: iki kez cagrilabiliyor
    // println!("{}", detective);
    //   E0382: `detective` closure'a tasindi
    // move "nasil yakaladigini" belirler, "kac kez cagrilabilecegini" DEGIL.

    println!("-- 4) closure adsiz bir struct'tir --");
    let empty = || 42;
    let small = move |x: u32| x + threshold as u32;      // threshold: u8 yakaladi
    let big_note = String::from("uzun bir dosya notu");
    let heavy = move || big_note.len();                  // String yakaladi
    println!("  yakalamayan closure : {} bayt", size_of_val(&empty));
    println!("  u8 yakalayan        : {} bayt", size_of_val(&small));
    println!("  String yakalayan    : {} bayt", size_of_val(&heavy));
    println!("  (String = ptr+len+cap = {} bayt)", size_of_val(&String::new()));

    println!("-- 5) fn pointer --");
    println!("  fonksiyon gecti     : {}", count_matching(&leads, weight_over_five));
    println!("  yakalamayan closure : {}", count_matching(&leads, |l| l.weight > 7));
    // count_matching(&leads, |l| l.weight >= threshold);
    //   E0308: closures can only be coerced to `fn` types if they do not capture
    //   -> threshold yakalandigi anda artik sadece bir adres degil.
    println!("  fn pointer boyutu   : {} bayt", size_of_val(&(weight_over_five as fn(&Lead) -> bool)));
}
