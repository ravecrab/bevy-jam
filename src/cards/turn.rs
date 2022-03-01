use bevy::prelude::*;

///Stats as a struct to hold
///Health, Armor, Speed, Mana
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct Queue(pub u32);

impl Default for Queue {
    fn default() -> Self {
        Self(0)
    }
}

impl Queue {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!("{:?}",self)
    }
}

