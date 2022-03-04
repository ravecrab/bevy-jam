use bevy::{prelude::*, ui::FocusPolicy};

use crate::cards::CardRep;

#[derive(Component)]
pub struct HandContainer;

#[derive(Component)]
pub struct CardInHand(pub CardRep);

pub fn create_container(parent: &mut ChildBuilder) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                size: Size::new(Val::Px(600.), Val::Px(300.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .insert(HandContainer);
}

pub fn spawn_card(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, card: &CardRep) {
    let card_front: Handle<Image> = asset_server.load("textures/cards/cardderpfront1.png");
    let font: Handle<Font> = asset_server.load("fonts/slkscr.ttf");
    let texture: Handle<Image> = asset_server.load(&card.sprites);

    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(180.), Val::Px(300.)),
                overflow: Overflow::Hidden,
                margin: Rect {
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            image: card_front.into(),
            ..Default::default()
        })
        // TODO: Definitely replace with something better
        .insert(CardInHand(card.clone()))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        // Overflow hack to show the first frame of the texture atlas.
                        // TODO: Replace for something better?
                        size: Size::new(Val::Px(144. * card.sprite_cols as f32), Val::Px(144.)),
                        position: Rect {
                            top: Val::Px(0.),
                            bottom: Val::Px(130.),
                            left: Val::Px(20.),
                            right: Val::Px(0.),
                        },
                        ..Default::default()
                    },
                    image: texture.clone().into(),
                    ..Default::default()
                })
                // Allow passing interactions to the button
                .insert(FocusPolicy::Pass);
            parent.spawn_bundle(TextBundle {
                focus_policy: FocusPolicy::Pass,
                style: Style {
                    max_size: Size::new(Val::Px(150.), Val::Px(100.)),
                    position: Rect {
                        top: Val::Px(50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    &card.name,
                    TextStyle {
                        font_size: 16.0,
                        color: Color::rgba_u8(17, 17, 17, 255),
                        font: font,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}
