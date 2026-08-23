// Gun 3 / Ders 3 - Vec Metotlari ve Metin Tipleri
// rustc main.rs && ./main

fn main() {
    // ---------------------------------------------------------------
    // BOLUM 1 - Vec metotlari, hizli gecis
    // ---------------------------------------------------------------
    let mut v = vec![3, 1, 2];
    println!("{:?} len={} bos_mu={}", v, v.len(), v.is_empty());

    v.push(9);                          // sona ekle
    println!("push      {:?}", v);

    let son = v.pop();                  // sondan al, Option doner
    println!("pop       {:?} -> {:?}", son, v);

    v.insert(1, 7);                     // araya ekle, sonrakiler kayar
    println!("insert    {:?}", v);

    let cikan = v.remove(0);            // cikar ve dondur
    println!("remove    {} -> {:?}", cikan, v);

    v.sort();                           // kucukten buyuge
    println!("sort      {:?}", v);

    v.reverse();
    println!("reverse   {:?}", v);

    v.swap(0, 1);
    println!("swap      {:?}", v);

    println!("contains(&7)={} first={:?} last={:?}", v.contains(&7), v.first(), v.last());

    // get sinir disinda None doner, v[i] paniklerdi
    println!("get(0)={:?} get(99)={:?}", v.get(0), v.get(99));

    v.extend(vec![4, 4, 8]);
    println!("extend    {:?}", v);

    v.sort();
    v.dedup();                          // yan yana tekrarlari siler
    println!("dedup     {:?}", v);

    v.retain(|x| x % 2 == 0);           // kosulu saglamayanlari at
    println!("retain    {:?}", v);

    v.truncate(1);
    println!("truncate  {:?}", v);

    println!("cap={}", v.capacity());

    // kapasite dolunca YENI blok alinir, veri tasinir, kapasite ikiye katlanir
    // (allocator bazen yerinde buyutur, o zaman adres ayni kalir - garantisi yok)
    let mut buyume = Vec::new();
    let mut onceki = buyume.capacity();
    for i in 0..17 {
        buyume.push(i);
        if buyume.capacity() != onceki {
            println!("  len={:<3} cap={:<3} adres={:p}",
                buyume.len(), buyume.capacity(), buyume.as_ptr());
            onceki = buyume.capacity();
        }
    }
    v.clear();
    println!("clear     {:?} bos_mu={}", v, v.is_empty());

    // gezinme uc sekilde
    let mut g = vec![1, 2, 3];
    for x in &g {
        print!("{} ", x);               // okur
    }
    println!();
    for x in &mut g {
        *x *= 10;                       // degistirir
    }
    println!("{:?}", g);
    for x in g {
        print!("{} ", x);               // TUKETIR - g bundan sonra yok
    }
    println!();

    // Vec'ten TASIMA yasak - E0507
    let sahipli = vec![String::from("a"), String::from("b")];
    // let ilk = sahipli[0];            // E0507 cannot move out of index
    let ilk = &sahipli[0];              // odunc al
    let kopya = sahipli[1].clone();     // ya da kopyala
    println!("E0507 cozumu: {} {}", ilk, kopya);

    // dilim - sahiplik tasimaz, sadece pencere
    let p = vec![10, 20, 30, 40, 50];
    println!("{:?} {:?} {:?} {:?}", &p[..], &p[2..], &p[..2], &p[1..=3]);
    println!("dilim {} bayt, normal referans {} bayt",
        std::mem::size_of_val(&&p[1..4]), std::mem::size_of_val(&&p[0]));

    // f64 icin sort yok, partial_cmp gerekir
    let mut f = vec![2.5, 1.5, 3.0];
    f.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", f);

    // ---------------------------------------------------------------
    // BOLUM 2 - String ve &str
    // ---------------------------------------------------------------
    println!("--- metin ---");

    // sabit metin ikilinin icinde, tipi &str
    let a: &str = "merhaba";
    let b: String = a.to_string();      // heap'e kopyala, sahiplen
    let c: &str = &b;                   // geri pencere ac
    println!("{} {} {}", a, b, c);

    println!("String={} bayt  &str={} bayt",
        std::mem::size_of::<String>(), std::mem::size_of::<&str>());

    // &String verilen yerde &str beklenen fonksiyon calisir
    println!("{}", uzunluk(&b));
    println!("{}", uzunluk("sabit"));

    // UTF-8: len() BAYT sayar
    let t = String::from("şğü");
    println!("\"{}\" len={} chars={}", t, t.len(), t.chars().count());

    // dilim bayt cinsinden ve harf sinirinda olmali
    println!("&t[0..2] = {:?}", &t[0..2]);        // "ş" - iki bayt
    println!("t.get(0..1) = {:?}", t.get(0..1));  // yarim harf -> None
    // println!("{}", &t[0..1]);                  // PANIK: byte index not a char boundary
    // println!("{}", t[0]);                      // E0277: String indekslenemez

    for (i, k) in t.char_indices() {
        println!("  bayt {} -> {} ({} bayt)", i, k, k.len_utf8());
    }

    // String uretmenin yollari - hepsi ayni kapiya cikar
    let y1 = String::new();
    let y2 = String::from("abc");
    let y3 = "abc".to_string();
    let y4 = "abc".to_owned();
    let y5 = format!("{}-{}", "abc", 1);
    println!("{:?} {} {} {} {}", y1, y2, y3, y4, y5);

    // n. karakteri almak O(n) - bastan taramak gerekiyor
    println!("chars().nth(1) = {:?}", t.chars().nth(1));

    // metnin bir parcasini dondurmek - kopya degil, dilim
    println!("ilk_kelime = {:?}", ilk_kelime("merhaba dunya"));
    println!("ilk_kelime = {:?}", ilk_kelime("tekkelime"));

    // sik kullanilan metotlar
    let mut m = String::from("  Rust kis kampi  ");
    println!("trim         {:?}", m.trim());
    println!("uppercase    {}", m.trim().to_uppercase());
    println!("contains     {}", m.contains("kis"));
    println!("starts_with  {}", m.trim().starts_with("Rust"));
    println!("find         {:?}", m.find("kis"));
    println!("replace      {}", m.trim().replace("kis", "yaz"));

    m.push_str("2026");
    m.push('!');
    println!("push_str     {:?}", m);
    println!("pop          {:?}", m.pop());

    // split(' ') bosluk yigilmasini HALLETMEZ, split_whitespace eder
    println!("split(' ')          {:?}", "a  b".split(' ').collect::<Vec<&str>>());
    println!("split_whitespace()  {:?}", "a  b".split_whitespace().collect::<Vec<&str>>());

    // split tembeldir, collect etmeden liste olmaz
    let kelimeler: Vec<&str> = m.trim().split_whitespace().collect();
    println!("split        {:?} ({} kelime)", kelimeler, kelimeler.len());
    println!("join         {}", kelimeler.join("-"));
    println!("repeat       {}", "ab".repeat(3));

    // parse Result doner
    println!("parse ok     {:?}", "42".parse::<i32>());
    println!("parse hata   {:?}", "kirk".parse::<i32>().is_err());

    // + sol tarafin SAHIPLIGINI alir
    let s1 = String::from("merhaba");
    let s2 = String::from("dunya");
    let s3 = s1 + " " + &s2;            // s1 tasindi
    // println!("{}", s1);              // E0382
    println!("+            {}", s3);

    // format! hicbirini tuketmez
    let s4 = String::from("iyi");
    let s5 = format!("{} {}", s4, "gunler");
    println!("format!      {} / {}", s4, s5);

    // Turkce tuzagi
    println!("Istanbul len={} chars={}", "İstanbul".len(), "İstanbul".chars().count());
    println!("'i' buyuk harf: {}   (Turkce'de I degil, noktali olmali)", 'i'.to_uppercase());
    println!("'I' kucuk harf: {}   (Turkce'de i degil, noktasiz olmali)", 'I'.to_lowercase());
    // "İ" kucultulunce IKI kod noktasi cikar - metin uzayabilir
    println!("\"İ\".to_lowercase() -> {} kod noktasi", "İ".to_lowercase().chars().count());
}

fn ilk_kelime(s: &str) -> &str {
    match s.find(' ') {
        Some(k) => &s[..k],
        None => s,
    }
}

fn uzunluk(s: &str) -> usize {
    s.chars().count()
}
