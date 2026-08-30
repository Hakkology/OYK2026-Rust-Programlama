//! Mandelbrot - CPU sürümü (tek thread + çok thread).
//!
//! Bu proje GPU showcase'in CPU tarafi. Egitmenin CUDA surumu yaninda
//! calistirilip karsilastirilir: ayni hesap, uc farkli donanim.
//!
//! cargo run --release
//! cargo run --release -- 1200 800

use std::thread;
use std::time::Instant;

const MAKS_ITER: u32 = 500;

fn main() {
    let arglar: Vec<String> = std::env::args().collect();
    let genislik: usize = arglar.get(1).and_then(|s| s.parse().ok()).unwrap_or(800);
    let yukseklik: usize = arglar.get(2).and_then(|s| s.parse().ok()).unwrap_or(600);

    println!("Mandelbrot {}x{}, maks iterasyon {}", genislik, yukseklik, MAKS_ITER);
    println!();

    // --- tek thread ---
    let t = Instant::now();
    let tek = hesapla_tek(genislik, yukseklik);
    let sure_tek = t.elapsed();
    println!("tek thread   : {:?}", sure_tek);

    // --- cok thread ---
    let cekirdek = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let t = Instant::now();
    let cok = hesapla_paralel(genislik, yukseklik, cekirdek);
    let sure_cok = t.elapsed();
    println!("{} thread     : {:?}", cekirdek, sure_cok);

    println!("hizlanma     : {:.2}x", sure_tek.as_secs_f64() / sure_cok.as_secs_f64());
    assert_eq!(tek, cok, "iki surum ayni sonucu vermeli");

    // --- terminale kucuk bir onizleme ---
    println!();
    onizleme(&tek, genislik, yukseklik);

    println!();
    println!("PPM olarak kaydetmek icin:");
    println!("  cargo run --release > /dev/null && ...");
    kaydet(&tek, genislik, yukseklik);
}

/// Bir pikselin kac iterasyonda kactigini hesaplar.
/// Kacmazsa MAKS_ITER doner (kume icinde).
fn nokta(cx: f64, cy: f64) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut i = 0;

    while x * x + y * y <= 4.0 && i < MAKS_ITER {
        let yeni_x = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = yeni_x;
        i += 1;
    }
    i
}

fn satir_hesapla(sy: usize, genislik: usize, yukseklik: usize) -> Vec<u32> {
    let mut satir = Vec::with_capacity(genislik);
    let cy = 1.2 - 2.4 * (sy as f64 / yukseklik as f64);

    for sx in 0..genislik {
        let cx = -2.2 + 3.0 * (sx as f64 / genislik as f64);
        satir.push(nokta(cx, cy));
    }
    satir
}

fn hesapla_tek(genislik: usize, yukseklik: usize) -> Vec<u32> {
    let mut sonuc = Vec::with_capacity(genislik * yukseklik);
    for sy in 0..yukseklik {
        sonuc.extend(satir_hesapla(sy, genislik, yukseklik));
    }
    sonuc
}

/// Gun 8: thread::scope ile move etmeden paralellik.
/// Her thread bir SATIR BLOGU hesapliyor.
fn hesapla_paralel(genislik: usize, yukseklik: usize, thread_sayisi: usize) -> Vec<u32> {
    let blok = yukseklik.div_ceil(thread_sayisi);

    let parcalar: Vec<Vec<u32>> = thread::scope(|s| {
        let mut isler = Vec::new();

        for i in 0..thread_sayisi {
            let bas = i * blok;
            let son = ((i + 1) * blok).min(yukseklik);

            isler.push(s.spawn(move || {
                let mut p = Vec::new();
                for sy in bas..son {
                    p.extend(satir_hesapla(sy, genislik, yukseklik));
                }
                p
            }));
        }

        isler.into_iter().map(|h| h.join().unwrap()).collect()
    });

    parcalar.into_iter().flatten().collect()
}

/// Terminale ASCII onizleme.
fn onizleme(veri: &[u32], genislik: usize, yukseklik: usize) {
    let karakterler = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let adim_y = (yukseklik / 24).max(1);
    let adim_x = (genislik / 78).max(1);

    for sy in (0..yukseklik).step_by(adim_y) {
        let mut satir = String::new();
        for sx in (0..genislik).step_by(adim_x) {
            let v = veri[sy * genislik + sx];
            let i = if v >= MAKS_ITER {
                karakterler.len() - 1
            } else {
                (v as usize * karakterler.len()) / MAKS_ITER as usize
            };
            satir.push(karakterler[i.min(karakterler.len() - 1)]);
        }
        println!("{}", satir);
    }
}

/// PPM dosyasi olarak kaydet (herhangi bir goruntu goruntuleyici acar).
fn kaydet(veri: &[u32], genislik: usize, yukseklik: usize) {
    use std::io::Write;

    let dosya = match std::fs::File::create("mandelbrot.ppm") {
        Ok(f) => f,
        Err(e) => {
            println!("dosya olusturulamadi: {}", e);
            return;
        }
    };
    let mut yazici = std::io::BufWriter::new(dosya);

    let _ = writeln!(yazici, "P6\n{} {}\n255", genislik, yukseklik);
    for v in veri {
        let renk = renklendir(*v);
        let _ = yazici.write_all(&renk);
    }
    println!("mandelbrot.ppm yazildi ({}x{})", genislik, yukseklik);
}

fn renklendir(iter: u32) -> [u8; 3] {
    if iter >= MAKS_ITER {
        return [0, 0, 0]; // kume icinde - siyah
    }
    let t = iter as f64 / MAKS_ITER as f64;
    [
        (9.0 * (1.0 - t) * t * t * t * 255.0) as u8,
        (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8,
        (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merkez_kume_icinde() {
        assert_eq!(nokta(0.0, 0.0), MAKS_ITER);
    }

    #[test]
    fn uzak_nokta_hemen_kacar() {
        assert!(nokta(5.0, 5.0) < 5);
    }

    #[test]
    fn tek_ve_paralel_ayni_sonuc() {
        let a = hesapla_tek(60, 40);
        let b = hesapla_paralel(60, 40, 4);
        assert_eq!(a, b);
    }

    #[test]
    fn paralel_bolunmeyen_yukseklikte_de_dogru() {
        // 40 satir / 3 thread - tam bolunmuyor
        let a = hesapla_tek(30, 40);
        let b = hesapla_paralel(30, 40, 3);
        assert_eq!(a, b);
        assert_eq!(b.len(), 30 * 40);
    }
}
