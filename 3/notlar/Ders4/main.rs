// Gun 3 / Ders 4 - HashMap
// rustc main.rs && ./main

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // olusturma - HashMap prelude'da yok, use gerekir
    let mut plaka: HashMap<String, u32> = HashMap::new();
    plaka.insert(String::from("Ankara"), 6);
    plaka.insert(String::from("Istanbul"), 34);
    plaka.insert(String::from("Izmir"), 35);
    println!("len={} {:?}", plaka.len(), plaka.get("Ankara"));

    // hazir dizi ve ciftlerden toplama
    let sabit = HashMap::from([("Konya", 42), ("Bursa", 16)]);
    let ciftler = vec![("Antalya", 7), ("Adana", 1)];
    let toplanan: HashMap<&str, u32> = ciftler.into_iter().collect();
    println!("{:?} {:?}", sabit.get("Konya"), toplanan.get("Adana"));

    // insert ayni anahtara gelirse UZERINE YAZAR, eskisini dondurur
    let eski = plaka.insert(String::from("Ankara"), 61);
    println!("uzerine yazildi, eski deger: {:?}", eski);
    plaka.insert(String::from("Ankara"), 6);            // duzeltelim

    // get Option doner - anahtarin olmamasi hata degil, normal durum
    match plaka.get("Ankara") {
        Some(k) => println!("Ankara = {}", k),
        None => println!("kayit yok"),
    }
    println!("{:?}", plaka.get("Yok"));
    println!("varsayilanli okuma: {}", plaka.get("Yok").copied().unwrap_or(0));

    // koseli parantez calisir ama anahtar yoksa PANIKLER
    println!("plaka[\"Izmir\"] = {}", plaka["Izmir"]);
    // println!("{}", plaka["Yok"]);                    // panic

    // contains_key / remove / len
    println!("contains_key(Bursa) = {}", plaka.contains_key("Bursa"));
    let silinen = plaka.remove("Izmir");                // sahipligi geri verir
    println!("remove(Izmir) -> {:?}, kalan {}", silinen, plaka.len());

    // get_mut ile degeri yerinde degistir
    if let Some(deger) = plaka.get_mut("Istanbul") {
        *deger += 100;
    }
    println!("Istanbul = {:?}", plaka.get("Istanbul"));

    // ENTRY - varsa bul, yoksa yarat
    let mut stok: HashMap<&str, i32> = HashMap::new();
    stok.insert("elma", 3);
    *stok.entry("elma").or_insert(0) += 1;              // vardi, arttirdi
    *stok.entry("armut").or_insert(0) += 1;             // yoktu, 0 koyup arttirdi
    stok.entry("kiraz").or_insert_with(|| 10);          // deger sadece gerekirse uretilir
    stok.entry("elma").and_modify(|v| *v *= 2).or_insert(1);
    println!("elma={:?} armut={:?} kiraz={:?}",
        stok.get("elma"), stok.get("armut"), stok.get("kiraz"));

    // klasik ornek - kelime sayma
    let metin = "rust hizli rust guvenli rust pratik";
    let mut sayim: HashMap<&str, i32> = HashMap::new();
    for kelime in metin.split_whitespace() {
        *sayim.entry(kelime).or_insert(0) += 1;
    }
    println!("{:?}", sayim.get("rust"));

    // gezinme - SIRA YOKTUR, her calistirmada degisebilir
    for (anahtar, deger) in &sayim {
        print!("{}={} ", anahtar, deger);
    }
    println!();

    // sirali cikti istiyorsak anahtarlari toplayip siralariz
    let mut anahtarlar: Vec<&&str> = sayim.keys().collect();
    anahtarlar.sort();
    for a in &anahtarlar {
        print!("{}={} ", a, sayim[**a]);
    }
    println!();

    // sirali gezmek asil isinizse BTreeMap - anahtari sirali tutar, O(log n)
    let mut agac: BTreeMap<&str, i32> = BTreeMap::new();
    agac.insert("zeytin", 3);
    agac.insert("armut", 1);
    agac.insert("elma", 2);
    for (k, v) in &agac {
        print!("{}={} ", k, v);         // her zaman alfabetik
    }
    println!();

    // values_mut ile tum degerleri degistir
    let mut zam: HashMap<&str, i32> = HashMap::from([("a", 10), ("b", 20)]);
    for d in zam.values_mut() {
        *d *= 2;
    }
    let mut liste: Vec<(&&str, &i32)> = zam.iter().collect();
    liste.sort();
    println!("{:?}", liste);

    // sahiplik - insert anahtari ve degeri TASIR
    let ad = String::from("anahtar");
    let veri = String::from("deger");
    let mut sahiplik: HashMap<String, String> = HashMap::new();
    sahiplik.insert(ad, veri);
    // println!("{}", ad);                              // E0382 - harita devraldi
    println!("{:?}", sahiplik.get("anahtar"));

    // get odunc verir - donen referans yasarken haritayi degistiremeyiz
    let referans = sahiplik.get("anahtar");
    println!("{:?}", referans);                         // oduncun son kullanimi
    sahiplik.insert(String::from("yeni"), String::from("x"));   // artik serbest
    println!("len={}", sahiplik.len());

    // HASHSET - degersiz HashMap, sadece "var mi" sorusuna cevap verir
    let mut kume = HashSet::new();
    println!("ilk ekleme: {}", kume.insert("elma"));    // true
    println!("ikinci ekleme: {}", kume.insert("elma")); // false - kume tekrar tutmaz
    kume.insert("armut");
    println!("contains(elma)={} len={}", kume.contains("elma"), kume.len());

    // tekrarlari ayiklamak icin pratik yol
    let tekrarli = vec![1, 2, 2, 3, 3, 3];
    let tekil: HashSet<i32> = tekrarli.into_iter().collect();
    let mut tekil_liste: Vec<i32> = tekil.into_iter().collect();
    tekil_liste.sort();
    println!("{:?}", tekil_liste);
}
