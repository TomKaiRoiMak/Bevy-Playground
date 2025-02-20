use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};

use crate::{Enemy, Player};



pub fn update_immunity_timer(
    mut enemies: Query<&mut Enemy>,
    mut player: Single<&mut Player>,
    time: Res<Time>,
) {
    for mut enemy in enemies.iter_mut() {
        enemy.immunity_timer -= time.delta_secs();
        if enemy.immunity_timer < 0.0 {
            enemy.immunity_timer = 0.0;
        }
    }
    player.immunity_timer -= time.delta_secs();
    if player.immunity_timer < 0.0 {
        player.immunity_timer = 0.0;
    }
}

pub fn get_screen_size(
    winit_windows: NonSend<WinitWindows>,
    window_query: Single<Entity, With<PrimaryWindow>>,
) -> (f32, f32) {
    let monitor = winit_windows
        .get_window(window_query.into_inner())
        .unwrap()
        .current_monitor()
        .unwrap()
        .size();
    let (width, height) = (monitor.height as f32, monitor.width as f32);
    (width, height)
}
