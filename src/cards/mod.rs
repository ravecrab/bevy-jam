mod action;
mod card;
mod components;
mod deck;

pub use crate::state::GameState;
pub use {action::*, card::*, components::*, deck::*};

use bevy::asset::LoadState;
use bevy::prelude::{Plugin as PluginTrait, *};
use bevy_asset_ron::RonAssetPlugin;

#[derive(Debug)]
pub struct AllCards(pub Vec<HandleUntyped>);

pub struct PlayerDeck(pub Deck);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CardRep>::new(&["card"]))
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_cards))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_loads))
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(create_player_deck));
    }
}

fn load_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server
        .load_folder("cards")
        .expect("Couldn't load assets!");
    commands.insert_resource(AllCards(handle));
}

fn check_loads(
    mut state: ResMut<State<GameState>>,
    cards: Res<AllCards>,
    asset_server: Res<AssetServer>,
) {
    for card in cards.0.clone() {
        if asset_server.get_load_state(card) != LoadState::Loaded {
            return;
        }
    }
    state.set(GameState::CardPicking).unwrap();
}
// Create the start of the player's deck of cards.
// For the moment, just add one of each type of card.
// Obviously, TODO: balance this in some way
fn create_player_deck(
    mut commands: Commands,
    assets: Res<Assets<CardRep>>,
    cards: Res<AllCards>,
    asset_server: Res<AssetServer>,
) {
    let mut deck = Deck::empty(4);

    for card in cards.0.clone() {
        let handle = card;

        if let LoadState::Loaded = asset_server.get_load_state(&handle) {
            info!("Handle loaded!");
        }
        // Debug information while struggling with the load
        info!("{:?}", handle);
        info!("{:?}", assets);
        info!("{:?}", assets.get(&handle));
        info!("{:?}", assets.get("cards/liswhistle.card"));
        // deck.cards.push(assets.get(card).unwrap().clone());
    }

    commands.insert_resource(PlayerDeck(deck));
}
