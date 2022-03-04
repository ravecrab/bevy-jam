use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Component)]
pub enum Action {
    Attack(i32),
    Burn(i32),
}

// #[allow(dead_code)]
// #[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
// pub enum Action {
//     Basic,
//     Ability,
// }

// impl Action {
//     #[cfg(feature = "debug")]
//     pub fn console_output(&self) -> String {
//         format!("{:?}", self)
//     }
// }
