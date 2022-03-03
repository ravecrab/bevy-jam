mod battle_stage;
mod opponent;
pub mod player;

use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/slkscr.ttf");
    let card_back: Handle<Image> = asset_server.load("images/card/back.png");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            player::create_ui(parent, &card_back, &font);
            // battle_stage::create_ui(parent);
            // opponent::create_ui(parent, &card_back);
        });
}
