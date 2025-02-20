use bevy::
    prelude::*
;

use crate::{Enemy, Player, ENEMY_SPEED};

pub fn enemy_follow_player(
    time: Res<Time>,
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_position = player.single().translation.xy();

    for mut enemy in enemies.iter_mut() {
        let enemy_position = enemy.translation.xy();
        let direction = (player_position - enemy_position)
            .normalize_or_zero()
            .extend(0.0);

        enemy.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}