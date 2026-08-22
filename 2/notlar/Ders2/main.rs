// Gun 2 / Ders 2 - Ownership
// rustc main.rs && ./main

struct Iz(String);

impl Drop for Iz {
    fn drop(&mut self) {
        println!("drop: {}", self.0);
    }
}

fn main() {
    // uc kural - her degerin bir sahibi var, ayni anda tek sahip,
    // sahip kapsam disina cikinca deger duser
    let s1 = String::from("merhaba");
    let s2 = s1;
    // println!("{}", s1);              // E0382 borrow of moved value
    println!("{}", s2);

    // move sig kopyadir - heap verisi kopyalanmaz, ucluyu s2 devralir
    println!("{:p}", s2.as_ptr());

    // clone acik ve pahali - derleyici sizin yerinize yapmaz
    let a1 = String::from("veri");
    let a2 = a1.clone();
    println!("{} {}", a1, a2);
    println!("{:p} {:p}", a1.as_ptr(), a2.as_ptr());

    // Copy tipler tasinmaz kopyalanir - stack'te ucuz oldugu icin
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let t = (1, 2.5, 'a');              // tum alanlari Copy ise tuple da Copy
    let u = t;
    println!("{:?} {:?}", t, u);

    let d = [1, 2, 3];                  // dizi de Copy
    let e = d;
    println!("{:?} {:?}", d, e);

    // heap tutan hicbir sey Copy degil
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    // println!("{:?}", v1);            // E0382
    println!("{:?}", v2);

    // String icinde tutan tuple da Copy degil
    let p1 = (String::from("ad"), 30);
    let p2 = p1;
    // println!("{:?}", p1);            // E0382
    println!("{:?}", p2);

    // kismi move - bir alan tasininca tuple'in tumu kullanilamaz olur
    let k1 = (String::from("ilk"), String::from("ikinci"));
    let ilk = k1.0;
    // println!("{:?}", k1);            // E0382 partially moved
    println!("{} {}", ilk, k1.1);       // k1.1 hala erisilebilir

    // shadowing move degil, eski deger hemen duser
    let g = String::from("eski");
    let g = String::from("yeni");
    println!("{}", g);

    // kapsam biter, drop calisir, RAII
    {
        let _i = Iz(String::from("kapsam ici"));
        println!("icerideyiz");
    }
    println!("disari ciktik");

    // move edilen deger yeni sahibinin kapsaminda duser
    let i1 = Iz(String::from("tasinan"));
    tasi_ve_dusur(i1);
    println!("fonksiyon dondu - deger coktan dustu");

    // drop sirasi ters
    let _b1 = Iz(String::from("birinci"));
    let _b2 = Iz(String::from("ikinci"));

    // mem::drop ile erken dusurulebilir
    let erken = Iz(String::from("erken"));
    drop(erken);
    println!("erken dustu, main devam ediyor");

    println!("main bitiyor");
}

fn tasi_ve_dusur(i: Iz) {
    println!("fonksiyon icinde: {}", i.0);
}
