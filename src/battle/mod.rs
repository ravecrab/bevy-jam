mod battle_tick;
mod components;
mod ui;

use bevy::prelude::{Plugin as PluginTrait, *};
use bevy_kira_audio::Audio;

use crate::{
    battle::{
        components::{InPlay, Players},
        ui::{
            battle_stage::PlayerStage,
            player::{
                hand::{self, CardInHand, HandContainer},
                info::InfoText,
            },
            setup_ui,
        },
    },
    cards::{Hitpoints, Name, PlayerDeck},
    state::GameState,
};

pub struct BattleTimer(Timer);

struct AnimationTimer(Timer);

pub struct Plugin;

impl PluginTrait for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BattleTimer(Timer::from_seconds(1.0, true))) // Change this value to change how often the units perform their actions
            .insert_resource(AnimationTimer(Timer::from_seconds(0.1, true)))
            .add_system_set(
                SystemSet::on_update(GameState::CardPicking)
                    // .with_system(battle_tick::player_battle_tick)
                    // .with_system(battle_tick::oppo_battle_tick)
                    // Commented out the above to test the animation
                    .with_system(idle_animation),
            )
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup_ui))
            .add_system_set(
                SystemSet::on_enter(GameState::CardPicking)
                    .with_system(setup_hand)
                    .with_system(play_music),
            )
            .add_system_set(SystemSet::on_enter(GameState::CardPicking).with_system(setup_hand))
            .add_system_set(
                SystemSet::on_update(GameState::CardPicking).with_system(interact_with_card),
            );
    }
}

/// This system runs the animation of any entity that has a TextureAtlas handle
fn idle_animation(
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("sounds/battle-theme-demo.ogg"));
}

// This system is not implemented yet. Shoud deal cards to player.
fn setup_hand(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut deck: ResMut<PlayerDeck>,
    mut query: Query<Entity, With<HandContainer>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/slkscr.ttf");
    let card_front: Handle<Image> = asset_server.load("images/card/front.png");

    for _ in 0..3 {
        let card = deck.0.cards.pop();
        match card {
            Some(c) => {
                let texture_handle = asset_server.load(&c.sprites);

                if let Ok(node_entity) = query.get_single_mut() {
                    commands.entity(node_entity).with_children(|parent| {
                        hand::spawn_card(parent, &card_front, &font, &c, &texture_handle)
                    });
                };
            }
            None => break, // No cards left in the player's deck... TODO: what now?
        };
    }
}

// COMPLETED:This system must load the sprite texture for the unit that
// the player chooses and spawn an entity with the appropriate components
// TODO: Split this function into other different functions or systems
/// This system is going to be responsible for recieving the
/// player's click on a card in their hand and placing that card into play
fn interact_with_card(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    inter_query: Query<(&Interaction, &CardInHand), (Changed<Interaction>, With<Button>)>,
    mut info_query: Query<&mut Text, With<InfoText>>,
    mut stage_query: Query<Entity, With<PlayerStage>>,
    sprite_query: Query<Entity, (With<TextureAtlasSprite>, With<InPlay>, With<Players>)>,
) {
    for (interaction, card) in inter_query.iter() {
        match *interaction {
            Interaction::Hovered => {
                if let Ok(mut text) = info_query.get_single_mut() {
                    text.sections[0].value = card.0.desc.clone();
                }
            }
            Interaction::Clicked => {
                // Kinda hackish way to despawn?
                // TODO: Move into own system
                if let Ok(sprite_entity) = sprite_query.get_single() {
                    commands.entity(sprite_entity).despawn();
                }

                let texture_handle = asset_server.load(&card.0.sprites);

                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(card.0.sprite_size_w, card.0.sprite_size_h),
                    card.0.sprite_cols,
                    card.0.sprite_rows,
                );

                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                commands
                    .spawn()
                    .insert_bundle((Name(card.0.name.clone()), Hitpoints(3), InPlay, Players))
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            // TODO: Make coordinates relative to screen size
                            translation: Vec3::new(500.0, 0.0, 2.0),
                            // Flip sprite to face opponent
                            rotation: Quat::from_rotation_y(std::f32::consts::PI),
                            scale: Vec3::splat(6.0),
                        },
                        ..Default::default()
                    });
            }
            // No interaction
            _ => (),
        }
    }
}
