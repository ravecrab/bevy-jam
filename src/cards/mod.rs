mod action;
mod card;
mod components;
mod deck;

pub use {action::*, card::*, components::*, deck::*};

use bevy::prelude::{Plugin as PluginTrait, *};
use bevy_asset_ron::RonAssetPlugin;

pub struct CardsList(pub Vec<HandleUntyped>);

pub struct PlayerDeck(pub Deck);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CardRep>::new(&["card"]));
        // .add_startup_system(setup);
    }
}

// Create the start of the player's deck of cards.
// For the moment, just add one of each type of card.
// Obviously, TODO: balance this in some way
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>, cards: Res<Assets<CardRep>>) {
//     let mut deck = Deck::empty(4);
//     let handles = asset_server.load_folder("cards").unwrap();

//     info!("Loaded {} cards.", handles.len());

//     for card in handles.clone() {
//         deck.cards
//             .push(cards.get(card.typed::<CardRep>()).unwrap().clone());
//     }

//     commands.insert_resource(CardsList(handles));
//     commands.insert_resource(PlayerDeck(deck));
// }
