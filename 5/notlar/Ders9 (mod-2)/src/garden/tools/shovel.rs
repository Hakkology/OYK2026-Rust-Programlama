// garden::tools::shovel

#[derive(Debug)]
pub struct Shovel {
    pub size: &'static str,
}

impl Shovel {
    pub fn new() -> Self {
        // crate:: = MUTLAK yol, crate kokunden. Nereden yazarsaniz yazin calisir.
        let _ = crate::garden::vegetables::COUNT;
        Shovel { size: "orta" }
    }
}
