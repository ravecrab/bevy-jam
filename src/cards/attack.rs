use rand::Rng;
use bevy::prelude::*;

///Stats as a struct to hold
///Health, Armor, Speed, Mana
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub enum Attack {
    Melee,
    Ranged,
    Magic,
}

impl Attack {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..=2) {
            1 => Attack::Melee,
            2 => Attack::Ranged,
            _ => Attack::Magic,
        }
    }
    //#[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}",self)
    }
}
