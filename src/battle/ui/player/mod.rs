pub mod hand;

use bevy::prelude::*;

pub fn create_ui(parent: &mut ChildBuilder, card_back: &Handle<Image>, font: &Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                padding: Rect::all(Val::Px(30.)),
                size: Size::new(Val::Percent(100.), Val::Percent(30.)),
                ..Default::default()
            },
            color: Color::rgba_u8(17, 17, 17, 255).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            create_deck_container(parent, card_back);
            create_hand_and_info_wrapper(parent, font);
        });
}

pub fn create_deck_container(parent: &mut ChildBuilder, card_back: &Handle<Image>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..3 {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        position: Rect {
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                            left: Val::Px(i as f32 * (-175.)),
                            right: Val::Px(0.),
                        },
                        size: Size::new(Val::Px(200.), Val::Auto),
                        ..Default::default()
                    },
                    image: card_back.clone().into(),
                    ..Default::default()
                });
            }
        });
}

pub fn create_hand_and_info_wrapper(parent: &mut ChildBuilder, font: &Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceEvenly,
                size: Size::new(Val::Percent(70.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            hand::create_container(parent);
            create_info_container(parent, font);
        });
}

pub fn create_info_container(parent: &mut ChildBuilder, font: &Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::TEAL.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Card Info",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        font: font.clone(),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}
