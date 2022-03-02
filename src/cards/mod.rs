mod action;
mod card;
mod components;
mod deck;

pub use {action::*, card::*, components::*, deck::*};

use bevy::prelude::{Plugin as PluginTrait, *};
use bevy_asset_ron::RonAssetPlugin;

#[derive(Debug)]
pub struct AllCards(pub Vec<Handle<CardRep>>);

pub struct PlayerDeck(pub Deck);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CardRep>::new(&["card"]))
            .add_startup_stage("cards", SystemStage::parallel())
            .add_startup_stage_after("cards", "deck", SystemStage::parallel())
            .add_startup_system_to_stage("cards", load_cards)
            .add_startup_system_to_stage("deck", create_player_deck);
    }
}

fn load_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<CardRep> = asset_server.load("cards/liswhistle.card");
    commands.insert_resource(AllCards(vec![handle]));
}

// Create the start of the player's deck of cards.
// For the moment, just add one of each type of card.
// Obviously, TODO: balance this in some way
fn create_player_deck(mut commands: Commands, assets: Res<Assets<CardRep>>, cards: Res<AllCards>) {
    let mut deck = Deck::empty(4);

    for card in cards.0.clone() {
        let handle = card;
        // Debug information while struggling with the load
        info!("{:?}", handle);
        info!("{:?}", assets);
        info!("{:?}", assets.get(&handle));
        info!("{:?}", assets.get("cards/liswhistle.card"));
        // deck.cards.push(assets.get(card).unwrap().clone());
    }

    commands.insert_resource(PlayerDeck(deck));
}
