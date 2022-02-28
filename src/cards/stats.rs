use rand::{thread_rng, Rng};

///Stats as a struct to hold
///Health, Armor, Speed, Mana
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Stats{
    health: u32,
    armor: u32,
    speed: u32,
    mana: u32,
    attack: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            health: 0,
            armor: 0,
            speed: 1000,
            mana: 0,
            attack: 0,
        }
    }
}

impl Stats {
    pub fn random(&mut self) {
        let mut rng = thread_rng();
        self.health = rng.gen();
        self.armor = rng.gen();
        self.speed = rng.gen();
        self.mana = rng.gen();
        self.attack = rng.gen();
    }
    #[inline]
    #[must_use]
    pub fn health(&self) -> u32 {
        self.health
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}",self)
    }
}
