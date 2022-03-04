pub mod deck;
pub mod hand;
pub mod info;

use bevy::prelude::*;

pub fn create_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                padding: Rect::all(Val::Px(30.)),
                size: Size::new(Val::Percent(100.), Val::Px(360.)),
                ..Default::default()
            },
            color: Color::rgba_u8(17, 17, 17, 255).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            deck::create_container(parent);
            hand::create_container(parent);
            info::create_container(parent, asset_server);
        });
}
