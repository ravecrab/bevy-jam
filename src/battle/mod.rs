mod battle_tick;
mod components;

use bevy::prelude::{Plugin as PluginTrait, *};
use team::*;
use crate::state::GameState;

use crate::battle::battle_tick::player_battle_tick;
use crate::cards::PlayerDeck;
use crate::state::GameState;

pub struct BattleTimer(Timer);
struct AnimationTimer(Timer);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .insert_resource(BattleTimer(Timer::from_seconds(1.0, true))) // Change this value to change how often the units perform their actions
            .insert_resource(AnimationTimer(Timer::from_seconds(0.1, true)))
            .add_system_set(
                SystemSet::on_update(GameState::Battle)
                    .with_system(battle_tick::player_battle_tick)
                    .with_system(battle_tick::oppo_battle_tick)
                    .with_system(idle_animation),
            )
            .add_system_set(SystemSet::on_enter(GameState::CardPicking).with_system(setup_hand)) // This system does not exist yet. Shoud deal cards to player.
            .add_system_set(SystemSet::on_update(GameState::CardPicking).with_system(pick_unit));
    }
}

/// This system runs the animation of any entity that has a TextureAtlas handle
fn idle_animation(time: Res<Time>, texture_atlases: Res<Assets<TextureAtlas>>, mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = sprite.index + 1 % texture_atlas.textures().len();
        }
    }
}

/// This system is going to be responsible for recieving the player's click on a card in their hand and placing that card into play
/// TODO: This system must load the sprite texture for the unit that the player chooses and spawn an entity with the appropriate components
fn pick_unit(mut state: ResMut<State<GameState>>, mut interaction_query: Query<&Interaction, (Changed<Interaction>)) {}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let card_front: Handle<Image> = asset_server.load("images/card/front.png");
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
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::SpaceBetween,
                                    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                                    ..Default::default()
                                },
                                color: Color::rgba_u8(0, 0, 0, 0).into(),
                                ..Default::default()
                            });
                            // .with_children(|parent| {
                            //     spawn_team(
                            //         parent,
                            //         card_front,
                            //         font.clone(),
                            //         player_deck.0.clone(),
                            //     );
                            // });

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

pub fn spawn_team(parent: &mut ChildBuilder, image: Handle<Image>, font: Handle<Font>, deck: Deck) {
    for card in deck.cards {
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
                        card.name,
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
