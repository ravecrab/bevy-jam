use crate::cards::CardRep;
use bevy::reflect::TypeUuid;
use std::ops::{Deref, DerefMut};

#[derive(TypeUuid, Debug, Clone)]
#[uuid = "8c2b8013-4b9d-4a4e-9f75-c2976cf46e67"]
pub struct Deck {
    pub count: u8,
    pub cards: Vec<CardRep>,
}

impl Deck {
    /// Generates an empty team
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            count: 0,
            cards: vec![],
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!("Deck of {} cards:\n", self.count);
        for card in self.cards.iter() {
            buffer = format!("{}{}\n", buffer, card.console_output());
        }
    }
}

impl Deref for Deck {
    type Target = Vec<CardRep>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}
