// garden::vegetables

pub const COUNT: u32 = 2;

#[derive(Debug)]
pub struct Asparagus {
    height_cm: u32,          // private alan: disaridan Asparagus { .. } kurulamaz
}

impl Asparagus {
    pub fn new(height_cm: u32) -> Self {
        Asparagus { height_cm }
    }
    pub fn height_cm(&self) -> u32 {
        self.height_cm
    }
}

#[derive(Debug)]
pub struct Tomato {
    pub variety: &'static str,   // pub ALAN: disaridan okunabilir
}

impl Tomato {
    pub fn new(variety: &'static str) -> Self {
        Tomato { variety }
    }
}

// pub yok -> sadece bu modul ve alt modulleri gorur
pub(super) fn soil_check() {
    println!("toprak   : uygun");
}
