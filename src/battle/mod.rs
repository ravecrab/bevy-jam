use bevy::prelude::{Plugin as PluginTrait, *};
use team::*;

pub mod battle_tick;
pub mod team;

pub struct BattleTimer(Timer);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BattleTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(ui)
            .add_system_set(SystemSet::on_enter(state::Battle).with_system(battle_tick));
    }
}

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_front = asset_server.load("images/card/front.png");
    let font = asset_server.load("fonts/slkscr.ttf");
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
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(240.), Val::Percent(100.)),
                                    ..Default::default()
                                },
                                image: asset_server.load("images/card/back.png").into(),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    position: Rect {
                                        top: Val::Px(0.),
                                        bottom: Val::Px(0.),
                                        left: Val::Px(-140.),
                                        right: Val::Px(0.),
                                    },
                                    size: Size::new(Val::Px(240.), Val::Auto),
                                    ..Default::default()
                                },
                                image: asset_server.load("images/card/back.png").into(),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    position: Rect {
                                        top: Val::Px(0.),
                                        bottom: Val::Px(0.),
                                        left: Val::Px(-280.),
                                        right: Val::Px(0.),
                                    },
                                    size: Size::new(Val::Px(240.), Val::Auto),
                                    ..Default::default()
                                },
                                image: asset_server.load("images/card/back.png").into(),
                                ..Default::default()
                            });
                        });

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
                                .with_children(|parent| {
                                    spawn_team(parent, card_front, font.clone())
                                });

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
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(40.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                            ..Default::default()
                        },
                        color: Color::GRAY.into(),
                        ..Default::default()
                    });

                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                            ..Default::default()
                        },
                        color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });

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
                                image: asset_server.load("images/card/back.png").into(),
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
                                image: asset_server.load("images/card/back.png").into(),
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
                                image: asset_server.load("images/card/back.png").into(),
                                ..Default::default()
                            });
                        });
                });
        });
}

pub fn spawn_team(parent: &mut ChildBuilder, image: Handle<Image>, font: Handle<Font>) {
    let mut team = Team::empty(4); // should move to arguments
    team.random();
    for card in team.team {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    ..Default::default()
                },
                image: image.clone().into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        card.console_output(),
                        TextStyle {
                            font_size: 12.0,
                            color: Color::BLACK,
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
}
