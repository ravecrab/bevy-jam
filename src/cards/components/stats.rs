// TODO: Transform into bundle
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Stats {
    health: u32,
    armor: u32,
    speed: u32,
    mana: u32,
    attack: u32,
}

impl Stats {
    #[inline]
    #[must_use]
    pub fn health(&self) -> u32 {
        self.health
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "Stats:\nHealth: {}\nArmor: {}\nSpeed: {}\nMana: {}\nAttack: {}",
            self.health, self.armor, self.speed, self.mana, self.attack
        )
    }
}
