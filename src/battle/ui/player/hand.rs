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
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .insert(HandContainer);
}

pub fn spawn_card(
    parent: &mut ChildBuilder,
    card_front: &Handle<Image>,
    font: &Handle<Font>,
    card: &CardRep,
    texture: &Handle<Image>,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
            image: card_front.clone().into(),
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
                        size: Size::new(
                            Val::Percent(card.sprite_cols as f32 * 100.),
                            Val::Percent(80.),
                        ),
                        position: Rect {
                            top: Val::Px(0.),
                            bottom: Val::Px(20.),
                            left: Val::Px(0.),
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
                        top: Val::Px(0.),
                        bottom: Val::Px(7.),
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    &card.name,
                    TextStyle {
                        font_size: 18.0,
                        color: Color::BLACK,
                        font: font.clone(),
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
