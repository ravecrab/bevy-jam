use bevy::prelude::*;

///Stats as a struct to hold
///Health, Armor, Speed, Mana
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub enum Status {
    Buff,
    Debuff,
    Aura,
}

impl Status {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}",self)
    }
}

