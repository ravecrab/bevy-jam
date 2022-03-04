mod opponent;
pub mod player;

use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            player::create_ui(parent, &asset_server);
        });
}
