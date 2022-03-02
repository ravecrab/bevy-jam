use bevy::prelude::*;

use crate::{
    battle::{
        components::{InPlay, Opponents, Players},
        BattleTimer,
    },
    cards::{Attack, Card, Hitpoints},
};

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

#[allow(dead_code)]
pub fn oppo_battle_tick() {}
