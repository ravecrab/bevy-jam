use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::RonAssetPlugin;
pub use {actions::*, attack::*, card::*, stats::*};

mod actions;
mod attack;
mod card;
mod stats;

#[allow(unused_imports)]
use serde::Deserialize;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Speed(pub u32);

#[derive(Component)]
pub struct Attack(pub u32);

#[derive(Component)]
pub struct Hitpoints(pub u32);

#[derive(Component)]
pub struct Hand;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct InPlay;

#[derive(Component)]
pub struct Players;

#[derive(Component)]
pub struct Oppos;

/// Only exists to load cards from text files and instantiate them. Not actually part of the ECS.
#[allow(dead_code)]
#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "41035a43-8099-4c30-a85e-72c45dbba279"]
pub struct CardRep {
    name: String,
    desc: String,
    actions: Vec<Action>,
    sprites: String, // Path to sprite sheet under `<project_root>/assets/`
}

#[derive(serde::Deserialize)]
pub enum Action {
    Attack(u8),
    Burn(u8),
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CardRep>::new(&["card"]))
            .add_startup_system(load_cards);
    }
}

fn load_cards(server: Res<AssetServer>, mut commands: Commands) {
    let card_handles = server.load_folder("cards").unwrap();
    info!("Loaded {} cards.", card_handles.len());

    commands.insert_resource(card_handles);
}

pub struct Deck {
    cards: Vec<CardRep>,
}

impl Deck {
    pub fn new() -> Self {
        Deck { cards: Vec::new() }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
