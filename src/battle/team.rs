use crate::cards::Card;
use std::ops::{Deref, DerefMut};

/// Base tile map
#[derive(Debug, Clone)]
pub struct Team {
    pub count: u8,
    pub team: Vec<Card>,
}

impl Team {
    /// Generates an empty map
    #[inline]
    #[must_use]
    pub fn empty(count: u8) -> Self {
        let team: Vec<Card> = (0..count).into_iter().map(|_| Card::default()).collect();
        Self { count, team }
    }

    /// Randomize couplets till max count and set them in the Team
    pub fn random(&mut self) {
        for card in self.team.iter_mut() {
            card.random();
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!("Team of {} cards:\n", self.count);
        for card in self.team.iter() {
            buffer = format!("{}{}\n", buffer, card.console_output());
        }
    }
}

impl Deref for Team {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.team
    }
}

impl DerefMut for Team {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.team
    }
}
