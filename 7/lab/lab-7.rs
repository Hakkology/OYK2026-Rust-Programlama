// Day 7 / LAB - Capstone v6: trait Storage
// rustc main.rs && ./main
// Cozumleri Gun 8'in isinmasinda yapacagiz.

fn main() {
    lab_1_dyn();
    lab_2_storage();
    lab_3_paylasim();
}

// ---------------------------------------------------------------------------
// LAB 1 - Dunku duvari yikmak
// Gun 6 lab 2g'de Urun, Hizmet ve Abonelik'i tek Vec'e koyamamistik.
// ---------------------------------------------------------------------------
fn lab_1_dyn() {
    println!("-- lab 1 --");

    // TODO 1a: Gun 6'daki Fiyatlandirilabilir trait'ini ve uc tipi buraya alin
    //          (Para, Urun, Hizmet, Abonelik)

    // TODO 1b: Vec<Box<dyn Fiyatlandirilabilir>> olusturun, ucunu birden koyun
    //          Hepsinin ozet()'ini yazdirin

    // TODO 1c: fn toplam_dyn(k: &[Box<dyn Fiyatlandirilabilir>]) -> Para

    // TODO 1d: Gun 6'daki generic toplam<T> fonksiyonu hala duruyor mu?
    //          Ikisini yan yana koyun. Hangisi hangi durumda kullanilir?

    // TODO 1e: fn en_pahali_dyn(...) -> Option<&Box<dyn Fiyatlandirilabilir>>
    //          Donus tipi cirkin. &dyn Fiyatlandirilabilir dondurebilir misiniz?

    // TODO 1f: size_of karsilastirmasi yazdirin
    //          &Urun / &dyn Fiyatlandirilabilir / Box<Urun> / Box<dyn ...>
    //          Neden ikisi 8, ikisi 16?

    // TODO 1g: Fiyatlandirilabilir trait'ine su metodu ekleyin:
    //              fn kopyala(&self) -> Self;
    //          Artik Box<dyn Fiyatlandirilabilir> derleniyor mu? Hatayi okuyun.
    //          Sonra geri alin. (Object safety.)
}

// ---------------------------------------------------------------------------
// LAB 2 - trait Storage
// Marketplace verisi nerede duruyor? Bellekte mi, dosyada mi?
// Kodun geri kalani bunu BILMEMELI.
// ---------------------------------------------------------------------------
fn lab_2_storage() {
    println!("-- lab 2 --");

    // TODO 2a: trait Storage tanimlayin
    //          fn kaydet(&mut self, u: Urun) -> Result<(), String>;
    //          fn bul(&self, ad: &str) -> Option<&Urun>;
    //          fn hepsi(&self) -> Vec<&Urun>;
    //          fn sil(&mut self, ad: &str) -> bool;
    //          fn sayi(&self) -> usize;
    //          VARSAYILAN: fn bos_mu(&self) -> bool

    // TODO 2b: struct BellekDepo { kalemler: Vec<Urun> }
    //          Storage implemente edin

    // TODO 2c: struct LogluDepo { ic: BellekDepo, log: Vec<String> }
    //          Storage implemente edin, her islemi log'a yazsin
    //          Bu bir DECORATOR - ayni arayuz, ek davranis

    // TODO 2d: fn rapor(d: &dyn Storage) -> String
    //          Hangi depo oldugunu BILMEDEN calissin

    // TODO 2e: Box<dyn Storage> ile calisma zamaninda depo secin
    //          fn depo_sec(loglu: bool) -> Box<dyn Storage>

    // TODO 2f: ayni rapor fonksiyonunu iki depoyla da cagirin

    // TODO 2g: rapor fonksiyonunu generic yazsaydiniz:
    //              fn rapor<T: Storage>(d: &T) -> String
    //          Ne degisirdi? depo_sec hala yazilabilir miydi?
}

// ---------------------------------------------------------------------------
// LAB 3 - Paylasilan durum
// ---------------------------------------------------------------------------
fn lab_3_paylasim() {
    println!("-- lab 3 --");

    // TODO 3a: Rc<RefCell<Urun>> ile bir urun olusturun
    //          Uc farkli siparis ayni urunu tutsun

    // TODO 3b: her siparis stogu dusursun, Rc::strong_count'u izleyin

    // TODO 3c: stok yetmediginde reddeden bir uygula() yazin

    // TODO 3d: bir siparis kapsam disina ciktiginda sayac ne oluyor?
    //          Blok icine alip gozlemleyin

    // TODO 3e: Rc<RefCell<T>> yerine &mut Urun kullanabilir miydiniz?
    //          Deneyin, hangi hatayi aliyorsunuz? (E0499)

    // TODO 3f: ayni RefCell'i ayni anda iki kez borrow_mut() edin
    //          Ne oluyor? Derleme hatasi mi, panic mi? Neden bu fark onemli?

    // TODO 3g: Kategori agaci kurun
    //          struct Kategori {
    //              ad: String,
    //              alt: RefCell<Vec<Rc<Kategori>>>,
    //              ust: RefCell<Weak<Kategori>>,
    //          }
    //          Elektronik > Bilgisayar > Klavye hiyerarsisi
    //          Klavye'den yukari cikip kok kategoriye ulasin

    // TODO 3h: 3g'de ust icin Rc kullansaydiniz ne olurdu?
    //          Rc::strong_count'lari yazdirip gosterin
}
