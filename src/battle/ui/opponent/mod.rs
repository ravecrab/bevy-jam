use bevy::prelude::*;

#[allow(dead_code)]
pub fn create_ui(parent: &mut ChildBuilder, card_back: &Handle<Image>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexEnd,
                padding: Rect::all(Val::Px(30.)),
                size: Size::new(Val::Percent(100.), Val::Percent(30.)),
                ..Default::default()
            },
            color: Color::rgba_u8(17, 17, 17, 255).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                        ..Default::default()
                    },
                    color: Color::rgba_u8(0, 0, 0, 0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            position: Rect {
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                                left: Val::Px(0.),
                                right: Val::Px(-280.),
                            },
                            size: Size::new(Val::Px(240.), Val::Percent(100.)),
                            ..Default::default()
                        },
                        image: card_back.clone().into(),
                        ..Default::default()
                    });
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            position: Rect {
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                                left: Val::Px(0.),
                                right: Val::Px(-140.),
                            },
                            size: Size::new(Val::Px(240.), Val::Auto),
                            ..Default::default()
                        },
                        image: card_back.clone().into(),
                        ..Default::default()
                    });
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            position: Rect {
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                                left: Val::Px(0.),
                                right: Val::Px(0.),
                            },
                            size: Size::new(Val::Px(240.), Val::Auto),
                            ..Default::default()
                        },
                        image: card_back.clone().into(),
                        ..Default::default()
                    });
                });
        });
}
