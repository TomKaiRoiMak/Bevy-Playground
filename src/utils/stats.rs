use bevy::prelude::*;

use crate::Player;

pub fn buff_handle(player_query: Single<&mut Player>, inputs: Res<ButtonInput<KeyCode>>) {
    let mut player = player_query.into_inner();
    if inputs.just_pressed(KeyCode::ArrowUp) {
        player.damage_multiplier += 0.1;
    }
    if inputs.just_pressed(KeyCode::ArrowDown) {
        player.damage_multiplier -= 0.1;
    }
}
