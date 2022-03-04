use bevy::prelude::*;

#[derive(Component)]
pub struct DeckContainer;

#[derive(Component)]
pub struct CardInDeck(pub usize);

pub fn create_container(parent: &mut ChildBuilder) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(220.), Val::Px(300.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .insert(DeckContainer);
}

pub fn spawn_card(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, index: usize) {
    let card_back: Handle<Image> = asset_server.load("textures/cards/cardderpback1.png");

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: if index < 5 {
                        Val::Px(10. * index as f32)
                    } else {
                        Val::Px(40.)
                    },
                    ..Default::default()
                },
                size: Size::new(Val::Px(180.), Val::Px(300.)),
                ..Default::default()
            },
            image: card_back.into(),
            ..Default::default()
        })
        .insert(CardInDeck(index));
}
