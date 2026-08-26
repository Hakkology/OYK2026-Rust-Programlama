use std::io::{self, Write};

struct Game {
    grid: Vec<f64>,
    player_idx: usize,
    score: i32,
}

impl Game {
    fn new() -> Self {
        Self {
            // Mars sıcaklık değerleri: bazıları geçerli (-125..=20), bazıları geçersiz
            grid: vec![
                -50.0,  35.0, -120.0,
                150.0, -10.0, -200.0,
                  5.0,  80.0,  -85.0,
            ],
            player_idx: 0,
            score: 0,
        }
    }

    fn draw(&self) {
        println!("\n=== MARS MUNCHER (Kural: -125.0 <= C <= 20.0) ===");
        println!("Skor: {}", self.score);
        for row in 0..3 {
            for col in 0..3 {
                let idx = row * 3 + col;
                let val = self.grid[idx];
                let is_player = if idx == self.player_idx { "[M]" } else { "   " };
                
                if val.is_nan() {
                    print!("{:<4} [ YENDİ ]  ", is_player);
                } else {
                    print!("{:<4} [{:>6.1}C] ", is_player, val);
                }
            }
            println!();
        }
        print!("Hareket (w/a/s/d) + (e: MUNCH, q: Çıkış): ");
        io::stdout().flush().unwrap();
    }

    fn munch(&mut self) {
        let val = self.grid[self.player_idx];
        if val.is_nan() {
            println!(">> Zaten boş!");
            return;
        }

        // Rust telemetri kuralı kontrolü
        if (-125.0..=20.0).contains(&val) {
            println!(">> HAM! Geçerli Mars telemetrisi yendi! (+10 Puan)");
            self.score += 10;
            self.grid[self.player_idx] = f64::NAN;
        } else {
            println!(">> BOOM! Hatalı veri sindirilemedi! (-15 Puan)");
            self.score -= 15;
        }
    }
}

fn main() {
    let mut game = Game::new();
    let mut input = String::new();

    loop {
        game.draw();
        input.clear();
        // read_line, girdi bitince (Ctrl-D ya da boru) Ok(0) doner.
        // Bu kontrol olmazsa dongu bos komutla sonsuza kadar doner.
        let okunan = io::stdin().read_line(&mut input).expect("stdin okunamadi");
        if okunan == 0 {
            println!("\n(girdi bitti)");
            break;
        }
        let cmd = input.trim().to_lowercase();

        match cmd.as_str() {
            "w" if game.player_idx >= 3 => game.player_idx -= 3,
            "s" if game.player_idx < 6 => game.player_idx += 3,
            "a" if game.player_idx % 3 != 0 => game.player_idx -= 1,
            "d" if game.player_idx % 3 != 2 => game.player_idx += 1,
            "e" => game.munch(),
            "q" => break,
            _ => println!("Geçersiz komut!"),
        }
    }
}