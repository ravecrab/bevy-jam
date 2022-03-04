mod attack;
mod stats;
mod status;

pub use {attack::*, stats::*, status::*};

use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Speed(pub u32);

#[derive(Component)]
pub struct Hitpoints(pub i32);

#[derive(Component)]
pub struct Card(pub u32);
