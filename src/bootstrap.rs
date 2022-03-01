use bevy::prelude::{Plugin as PluginTrait, *};

use crate::state::GameState;
use crate::cards::Deck;

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera)
            .add_startup_system(init_deck)
            // TODO: Implement a context-aware exit on esc to replace this one
            .add_system(bevy::input::system::exit_on_esc_system)
            // TODO: Switch to `Intro` state for a release build or
            // another state for debug builds. Also, this could be in
            // a config file, maybe?
            .add_state(GameState::Battle);
    }
}

pub fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Create the start of the player's deck of cards.
// For the moment, just add one of each type of card.
// Obviously, TODO: balance this in some way
pub fn init_deck(commands: Commands) {
    let mut deck = Deck::new();

    let card_list = asset_server.get_handle("cards");

    for card in card_list {
        deck.cards.push(card.clone());
    }

    commands.insert_resource(deck);
}