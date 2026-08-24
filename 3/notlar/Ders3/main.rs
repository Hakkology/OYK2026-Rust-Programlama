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

    // pop bir Option dondurur: "Some(9)" deger var demek, liste bossa "None" gelir
    // unwrap icindekini alir (bos listede unwrap panikler)
    let son = v.pop();
    println!("pop       {:?} -> {:?}", son, v);

    let cikan = v.remove(0);            // cikar ve dondur
    println!("remove    {} -> {:?}", cikan, v);

    v.sort();                           // kucukten buyuge
    println!("sort      {:?}", v);

    v.reverse();
    println!("reverse   {:?}", v);

    println!("contains(&7)={} first={:?} last={:?}", v.contains(&7), v.first(), v.last());

    // get sinir disinda None doner, v[i] paniklerdi
    println!("get(0)={:?} get(99)={:?}", v.get(0), v.get(99));

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

    // ayni metotlarla iki farkli yapi
    // KUYRUK - ilk giren ilk cikar: push sona ekler, remove(0) bastan alir
    let mut kuyruk = vec!["ali", "veli"];
    kuyruk.push("ayse");
    while !kuyruk.is_empty() {
        let kisi = kuyruk.remove(0);    // O(n) - kalan herkes kayar
        print!("sira:{} ", kisi);
    }
    println!();

    // YIGIN - son giren ilk cikar: push sona ekler, pop sondan alir
    let mut yigin = Vec::new();
    yigin.push("birinci");
    yigin.push("ikinci");
    while !yigin.is_empty() {
        let ust = yigin.pop().unwrap(); // O(1) - hicbir sey kaymaz
        print!("ust:{} ", ust);
    }
    println!();

    // ---------------------------------------------------------------
    // ITERATOR - uc yol, tek fark SAHIPLIK
    //   kisayol      acik yazim          dongude x'in tipi
    //   &v           v.iter()            &T
    //   &mut v       v.iter_mut()        &mut T
    //   v            v.into_iter()       T
    // ---------------------------------------------------------------

    // 1) OKUMAK - liste bizde kalir
    let notlar = vec![70, 85, 90];

    let mut toplam = 0;
    for n in &notlar {                  // kisayol
        toplam += n;
    }

    let mut toplam2 = 0;
    for n in notlar.iter() {            // ayni seyin acik yazimi
        toplam2 += n;
    }
    println!("toplam={} toplam2={} liste hala var: {:?}", toplam, toplam2, notlar);

    // iterator bir DEGERDIR - degiskene alinabilir, tek basina hicbir sey yapmaz
    let gezgin = notlar.iter();
    println!("kac eleman: {}", gezgin.count());   // ancak burada calisti

    // hazir metotlar da ayni iterator uzerinden gider
    println!("sum={}", notlar.iter().sum::<i32>());

    // for icinde secip yeni listeye toplamak
    let mut gecenler = Vec::new();
    for n in &notlar {
        if *n >= 85 {
            gecenler.push(*n);          // *n ile degeri kopyaladik
        }
    }
    println!("gecenler {:?}", gecenler);

    // 2) DEGISTIRMEK - &mut T verir, hedefe inmek icin * SART
    let mut fiyatlar = vec![100, 200, 300];
    for f in &mut fiyatlar {            // kisayol
        *f = *f * 110 / 100;            // %10 zam
    }
    for f in fiyatlar.iter_mut() {      // acik yazim
        *f += 1;
    }
    println!("fiyatlar {:?}", fiyatlar);

    // 3) TUKETMEK - elemanlarin sahipligi donguye gecer
    let isimler = vec![String::from("ada"), String::from("ege")];
    let mut buyuk = Vec::new();
    for i in isimler {                  // kisayol (= isimler.into_iter())
        buyuk.push(i.to_uppercase());   // i: String, sahibi biziz
    }
    // println!("{:?}", isimler);       // E0382 - isimler tasindi, artik yok
    println!("buyuk {:?}", buyuk);

    // ayni is acik yazimla - into_iter() de listeyi tuketir
    let sehirler = vec![String::from("ankara"), String::from("izmir")];
    let mut uzunluklar = Vec::new();
    for s in sehirler.into_iter() {
        uzunluklar.push(s.chars().count());
    }
    println!("uzunluklar {:?}", uzunluklar);

    // okumak yetiyorsa TUKETME - tek karakter fark
    let kalsin = vec![String::from("bursa")];
    for s in &kalsin {
        println!("{} ({} harf)", s, s.chars().count());
    }
    println!("kalsin hala duruyor: {:?}", kalsin);

    // AYNI KURAL her koleksiyonda gecerli - HashMap'te de for bir iterator uzerinde doner:
    //   for (anahtar, deger) in &harita        okur
    //   for deger in harita.values_mut()       degistirir
    //   for (anahtar, deger) in harita         tuketir

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
    println!("format!      {s4} / {s5}");

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
