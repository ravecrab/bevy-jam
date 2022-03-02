use crate::cards::Action;

use bevy::reflect::TypeUuid;

/// Only exists to load cards from text files and instantiate them. Not actually part of the ECS.
#[allow(dead_code)]
#[derive(serde::Deserialize, TypeUuid, Debug, Clone, Eq, PartialEq, Default)]
#[uuid = "41035a43-8099-4c30-a85e-72c45dbba279"]
pub struct CardRep {
    pub name: String,
    pub desc: String,
    pub actions: Vec<Action>,
    pub sprites: String, // Path to sprite sheet under `<project_root>/assets/`
}

impl CardRep {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!("Card: {}\n", self.attack.console_output());
        buffer = format!("{}{}", buffer, self.stats.console_output());
        buffer
    }
}
