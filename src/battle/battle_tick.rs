use bevy::prelude::*;

use crate::battle::BattleTimer;
use crate::cards::{Attack, Card, Hitpoints, InPlay, Oppos, Players};

pub fn player_battle_tick(
    time: Res<Time>,
    mut timer: ResMut<BattleTimer>,
    player_card: Query<&Attack, (With<Card>, With<Players>, With<InPlay>)>,
    oppo_card: Query<&mut Hitpoints, (With<Card>, With<Oppos>, With<InPlay>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // TODO: Play attack animation
        let attack = player_card.single();
        let hp = oppo_card.single_mut();
        hp.0 = hp.0 - attack.0;
    }
}

pub fn oppo_battle_tick() {}
