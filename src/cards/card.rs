use crate::cards::stats::*;
use crate::cards::attack::*;

///Card as a struct to hold stats and attack
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card{
    stats: Stats,
    attack: Attack,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            stats: Stats::default(),
            attack: Attack::Melee,
        }
    }
}

impl Card {
    #[inline]
    #[must_use]
    pub fn stats(&self) -> Stats {
        self.stats
    }

    pub fn random(&mut self) {
        self.stats.random();
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}",self)
    }
}
