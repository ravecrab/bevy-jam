pub mod deck;
pub mod hand;
pub mod info;

use bevy::prelude::*;

pub fn create_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(100.), Val::Px(229.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            deck::create_container(parent);
            hand::create_container(parent);
            info::create_container(parent, asset_server);
        });
}
