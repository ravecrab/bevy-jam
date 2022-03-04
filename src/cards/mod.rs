mod action;
mod card;
mod components;
mod deck;

pub use crate::state::GameState;
pub use {action::*, card::*, components::*, deck::*};

use bevy::asset::LoadState;
use bevy::prelude::{Plugin as PluginTrait, *};
use bevy_asset_ron::RonAssetPlugin;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct AllCards(pub Vec<Handle<CardRep>>);

pub struct PlayerDeck(pub Deck);

pub struct OpponentDeck(pub Deck);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CardRep>::new(&["card"]))
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_cards))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_loads))
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(create_player_deck))
            .add_system_set(
                SystemSet::on_exit(GameState::Loading).with_system(create_opponent_deck),
            );
    }
}

fn load_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    // `load_folder` does not work on wasm
    // see: https://github.com/bevyengine/bevy/issues/2916
    let catipular: Handle<CardRep> = asset_server.load("cards/catipular.card");
    let liswhistle: Handle<CardRep> = asset_server.load("cards/liswhistle.card");
    commands.insert_resource(AllCards(vec![catipular, liswhistle]));
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
    let mut deck = Deck::empty();
    let cards_ref = &cards.0;

    for _ in 0..5 {
        let handle = cards_ref.choose(&mut thread_rng()).unwrap();

        if let LoadState::Loaded = asset_server.get_load_state(handle) {
            let card = assets.get(handle).unwrap().clone();
            deck.count += 1;
            deck.cards.push(card);
        }
    }

    commands.insert_resource(PlayerDeck(deck));
}

// TODO: DRY this system and the one above
fn create_opponent_deck(
    mut commands: Commands,
    assets: Res<Assets<CardRep>>,
    cards: Res<AllCards>,
    asset_server: Res<AssetServer>,
) {
    let mut deck = Deck::empty();
    let cards_ref = &cards.0;

    for _ in 0..5 {
        let handle = cards_ref.choose(&mut rand::thread_rng()).unwrap();

        if let LoadState::Loaded = asset_server.get_load_state(handle) {
            let card = assets.get(handle).unwrap().clone();
            deck.count += 1;
            deck.cards.push(card);
        }
    }

    commands.insert_resource(OpponentDeck(deck));
}
