// Gun 8 / Ders 5 - Tek Yonlu Bagli Liste
// rustc main.rs && ./main
//
// Ekibin canta destesi: son atilan canta EN USTE gelir, kacarken de en ustteki
// once alinir (LIFO). Klasik tek yonlu bagli liste.
//
// Bu ders yeni bir konu degil - kampin TOPLAMI:
//   Box (dun)          -> ozyinelemeli tip
//   Option (Gun 4)     -> "sonrasi var mi?"
//   Generic (Gun 6)    -> her tipi tasiyan liste
//   Lifetime (dun)     -> veriyi kopyalamadan gezmek
//   Associated type    -> Iterator implementasyonu
//   Drop (dun)         -> yigini patlatmadan temizlemek

// Ozyinelemeli tip okunakli olsun diye takma ad:
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Stash<T> {
    head: Link<T>,
    len: usize,
}

impl<T> Stash<T> {
    fn new() -> Stash<T> {
        Stash { head: None, len: 0 }
    }

    // EN USTE ekle. Yeni dugumun next'i eski head olur.
    fn push(&mut self, elem: T) {
        // self.head'i DOGRUDAN tasiyamayiz: elimizde &mut var, sahiplik yok.
        //   let eski = self.head;
        //   E0507: cannot move out of `self.head` which is behind a mutable reference
        // take() cozumu: yerine None birakir, eskisini bize verir.
        let yeni = Box::new(Node { elem, next: self.head.take() });
        self.head = Some(yeni);
        self.len += 1;
    }

    // EN USTTEKINI cikar. Liste bossa None.
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|dugum| {
            self.head = dugum.next;      // dugum burada sahiplenildi, alanlari serbest
            self.len -= 1;
            dugum.elem
        })
    }

    // Cikarmadan BAK. as_ref: Option<Box<Node>> -> Option<&Box<Node>>
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|dugum| &dugum.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|dugum| &mut dugum.elem)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Odunc alarak gezen iterator (Gun 6: associated type, dun: lifetime)
    fn iter(&self) -> Iter<'_, T> {
        Iter { simdiki: self.head.as_deref() }
    }
}

// ---------------------------------------------------------------
// ITERATOR - odunc alan surum
// ---------------------------------------------------------------
struct Iter<'a, T> {
    simdiki: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;                   // Gun 6: bir iterator TEK tip uretir

    fn next(&mut self) -> Option<&'a T> {
        self.simdiki.map(|dugum| {
            self.simdiki = dugum.next.as_deref();
            &dugum.elem
        })
    }
}

// Sahiplenen surum: liste tuketilir
impl<T> Iterator for Stash<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

// ---------------------------------------------------------------
// DROP - neden elle yaziyoruz
// ---------------------------------------------------------------
// Varsayilan drop OZYINELEMELI calisir: head dusurulurken next dusurulur,
// o dusurulurken bir sonraki... Uzun listede YIGIN TASAR (stack overflow).
// Iteratif surum bunu onler:
impl<T> Drop for Stash<T> {
    fn drop(&mut self) {
        let mut simdiki = self.head.take();
        while let Some(mut dugum) = simdiki {
            simdiki = dugum.next.take();  // baglantiyi kes, sonra dugum dussun
        }
    }
}

fn main() {
    println!("-- 1) temel islemler --");
    let mut stash: Stash<&str> = Stash::new();
    println!("  bos mu: {}", stash.is_empty());
    stash.push("kasa 3: tahvil");
    stash.push("kasa 7: elmas");
    stash.push("kasa 1: veri cekirdegi");
    println!("  {} canta var", stash.len());
    println!("  en ustteki: {:?}", stash.peek());

    println!("-- 2) LIFO: son atilan once alinir --");
    while let Some(canta) = stash.pop() {
        println!("    araca kondu: {}", canta);
    }
    println!("  bos mu: {}", stash.is_empty());
    println!("  bos desteden pop: {:?}", stash.pop());

    println!("-- 3) generic: her tipi tasir --");
    let mut sayilar: Stash<u32> = Stash::new();
    for n in [10, 20, 30] {
        sayilar.push(n);
    }
    println!("  icerik: {:?}", sayilar.iter().collect::<Vec<_>>());

    println!("-- 4) peek_mut ile degistirme --");
    if let Some(ilk) = sayilar.peek_mut() {
        *ilk *= 100;
    }
    println!("  icerik: {:?}", sayilar.iter().collect::<Vec<_>>());

    println!("-- 5) iterator: kopyalamadan gezmek --");
    // iter() ODUNC aliyor: liste yerinde duruyor
    let toplam: u32 = sayilar.iter().sum();
    let en_buyuk = sayilar.iter().max();
    println!("  toplam {} | en buyuk {:?} | uzunluk {}", toplam, en_buyuk, sayilar.len());
    // Gun 7'nin kombinatorleri burada da calisiyor:
    let buyukler: Vec<&u32> = sayilar.iter().filter(|n| **n > 15).collect();
    println!("  15'ten buyukler: {:?}", buyukler);

    println!("-- 6) sahiplenen iterator: liste tukenir --");
    let toplam2: u32 = sayilar.by_ref().take(2).sum();   // ilk iki cantayi TUKETIR
    println!("  ilk ikisinin toplami: {} | kalan uzunluk: {}", toplam2, sayilar.len());

    println!("-- 7) uzun liste: iteratif Drop --");
    let mut buyuk: Stash<u32> = Stash::new();
    for i in 0..200_000 {
        buyuk.push(i);
    }
    println!("  {} dugum olusturuldu", buyuk.len());
    drop(buyuk);
    println!("  temizlendi - ozyinelemeli drop olsaydi yigin tasardi");
}
