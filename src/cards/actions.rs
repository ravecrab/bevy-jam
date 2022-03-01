use bevy::prelude::*;

///Stats as a struct to hold
///Health, Armor, Speed, Mana
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub enum Action {
    Basic,
    Ability,
}

impl Action {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}", self)
    }
}
