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
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}",self)
    }
}
