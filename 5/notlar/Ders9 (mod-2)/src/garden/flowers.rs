// garden::flowers

pub const COUNT: u32 = 1;

// enum pub ise VARYANTLARI da otomatik pub olur - struct alanlarindan farki budur
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
pub struct Rose {
    pub color: Color,
}

impl Rose {
    pub fn new(color: Color) -> Self {
        // super:: ile UST modul (garden), oradan kardes modul
        let _ = super::vegetables::COUNT;
        Rose { color }
    }
}
