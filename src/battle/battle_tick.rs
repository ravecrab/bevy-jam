use bevy::prelude::*;

use crate::{
    battle::{
        components::{InPlay, Opponents, Players},
        BattleTimer,
    },
    cards::{Attack, Card, Hitpoints},
};

/// This system runs every one second and performs the player's current in-play unit's attack
/// TODO: Needs to be generalized to encompass the various kinds of actions a unit can have
/// TODO: If the opponent's unit dies, need to remove that entity and have the opponent play a new card
///       or, if out of cards, declare victory and go to store
#[allow(clippy::type_complexity)]
pub fn player_battle_tick(
    time: Res<Time>,
    mut timer: ResMut<BattleTimer>,
    player_card: Query<&Attack, (With<Card>, With<Players>, With<InPlay>)>,
    mut oppo_card: Query<&mut Hitpoints, (With<Card>, With<Opponents>, With<InPlay>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // TODO: Play attack animation
        let attack = player_card.single();
        let mut hp = oppo_card.single_mut();
        hp.0 -= attack.0;
    }
}

/// Same as the `player_battle_tick` system above, but for the opponent
/// TODO: Needs to be generalized to encompass the various kinds of actions a unit can have
/// TODO: Need to find a way to delay the start of the timer for this system so that the two
///       units are not alway attacking at the same time
#[allow(dead_code)]
pub fn oppo_battle_tick() {}
