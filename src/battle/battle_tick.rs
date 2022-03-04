use bevy::prelude::*;

use crate::{
    battle::{
        components::{InPlay, Opponents, Players},
        BattleTimer,
    },
    cards::{Action, Hitpoints},
    state::GameState,
};

/// This system runs every one second and performs the player's current in-play unit's attack
/// TODO: Needs to be generalized to encompass the various kinds of actions a unit can have
/// TODO: If the opponent's unit dies, need to remove that entity and have the opponent play a new card
///       or, if out of cards, declare victory and go to store
#[allow(clippy::type_complexity)]
pub fn player_battle_tick(
    time: Res<Time>,
    mut timer: ResMut<BattleTimer>,
    player_card: Query<&Action, (With<Players>, With<InPlay>)>,
    mut oppo_card: Query<&mut Hitpoints, (With<Opponents>, With<InPlay>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // TODO: Play attack animation
        let attack = player_card.single();
        let mut hp = oppo_card.single_mut();
        match attack {
            Action::Attack(amount) => hp.0 -= amount,
            Action::Burn(amount) => hp.0 -= amount,
        }
    }
}

/// Same as the `player_battle_tick` system above, but for the opponent
/// TODO: Needs to be generalized to encompass the various kinds of actions a unit can have
/// TODO: Need to find a way to delay the start of the timer for this system so that the two
///       units are not alway attacking at the same time
#[allow(dead_code)]
pub fn oppo_battle_tick() {}

pub fn bring_out_your_dead_player(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    dead_query: Query<(Entity, &Hitpoints), (With<Players>, With<InPlay>)>,
) {
    for (e, hp) in dead_query.iter() {
        if hp.0 <= 0 {
            commands.entity(e).despawn();
            state.set(GameState::CardPicking).unwrap();
        }
    }
}

pub fn bring_out_your_dead_oppo(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    dead_query: Query<(Entity, &Hitpoints), (With<Opponents>, With<InPlay>)>,
) {
    for (e, hp) in dead_query.iter() {
        if hp.0 <= 0 {
            commands.entity(e).despawn();
            state.set(GameState::OpponentCardPick).unwrap();
        }
    }
}
