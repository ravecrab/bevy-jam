mod battle_tick;
mod components;
mod ui;

use bevy::prelude::{Plugin as PluginTrait, *};

use crate::{
    battle::{
        components::{Hand, Players},
        ui::{
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
                SystemSet::on_update(GameState::Battle)
                    .with_system(battle_tick::player_battle_tick)
                    .with_system(battle_tick::oppo_battle_tick)
                    .with_system(idle_animation),
            )
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup_ui))
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
            sprite.index += 1 % texture_atlas.textures.len();
        }
    }
}

// This system is not implemented yet. Shoud deal cards to player.
fn setup_hand(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(c.sprite_size_w, c.sprite_size_h),
                    c.sprite_cols,
                    c.sprite_rows,
                );
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                commands
                    .spawn()
                    .insert_bundle((Name(c.name), Hitpoints(3), Hand, Players))
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform::from_scale(Vec3::splat(6.0)),
                        ..Default::default()
                    });
            }
            None => break, // No cards left in the player's deck... TODO: what now?
        };
    }
}

/// This system is going to be responsible for recieving the player's click on a card in their hand and placing that card into play
/// TODO: This system must load the sprite texture for the unit that the player chooses and spawn an entity with the appropriate components
/// TODO: Maybe move this into UI?
fn interact_with_card(
    inter_query: Query<(&Interaction, &CardInHand), (Changed<Interaction>, With<Button>)>,
    mut info_query: Query<&mut Text, With<InfoText>>,
) {
    for (interaction, card) in inter_query.iter() {
        match *interaction {
            Interaction::Hovered => {
                if let Ok(mut text) = info_query.get_single_mut() {
                    text.sections[0].value = card.0.desc.clone();
                }
            }
            Interaction::Clicked => {
                info!("click {:?}", card.0);
            }
            _ => (),
        }
    }
}
