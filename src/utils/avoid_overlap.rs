use bevy::{
    math::bounding::{BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{DupeTimer, Enemy, Player, ENEMY_SPEED};

use super::spawn_entity::spawn_enemy_at_entity;

pub fn avoid_overlaping(
    // mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform, &mut Enemy), (With<Enemy>, Without<Player>)>,
    // player: Single<(&mut Transform, &mut Player), (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timer: ResMut<DupeTimer>,
) {
    let enemy_size = 20.;
    // let player_size = 18.;
    let mut combinations = enemies.iter_combinations_mut();
    // let mut despawned_entities = HashSet::new();

    while let Some([(_, mut trans1, _), (_, mut trans2, _)]) = combinations.fetch_next() {
        let aabb1 = BoundingCircle::new(trans1.translation.xy(), enemy_size);
        let aabb2 = BoundingCircle::new(trans2.translation.xy(), enemy_size);
        if aabb1.intersects(&aabb2) {
            let push_dir = (aabb2.center() - aabb1.center()).normalize_or_zero();
            trans1.translation -= (push_dir * ENEMY_SPEED * time.delta_secs()).extend(0.);
            trans2.translation += (push_dir * ENEMY_SPEED * time.delta_secs()).extend(0.);

            // if timer.0.tick(time.delta()).just_finished() {
            //     spawn_enemy_at_entity(
            //         commands.reborrow(),
            //         &mut meshes,
            //         &mut materials,
            //         trans1.translation,
            //     );
            // }
        }
    }
}
