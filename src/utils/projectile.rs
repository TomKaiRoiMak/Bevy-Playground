use bevy::{
    color::palettes::css::BLUE, math::bounding::Aabb2d, prelude::*,
};

use crate::{Player, Projectile};


pub fn shoot_default_projectile(
    mut commands: Commands,
    player_trans: Single<&Transform, With<Player>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    inputs: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if inputs.pressed(MouseButton::Left) {
        let (camera, camera_trans) = *camera_query;
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Ok(mouse_point2d) = camera.viewport_to_world_2d(camera_trans, cursor_position) else {
            return;
        };

        let target = (mouse_point2d.xy() - player_trans.translation.xy()).normalize_or_zero();
        let angle = target.y.atan2(target.x);

        

        commands.spawn((
            Transform {
                translation: player_trans.translation,
                rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            },
            Mesh2d(meshes.add(Rectangle::new(40., 10.))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
            Projectile {
                hitbox: Aabb2d::new(player_trans.translation.xy(), Vec2::new(10., 10.)),
                offset: 30.,
                base_damage: 3.,
                lifetime: 1.,
                piercing: 3,
                velocity: target * 800.,
            },
        ));
    }
}

pub fn move_projectiles(
    mut projectiles: Query<(&mut Projectile, &mut Transform)>,
    time: Res<Time>,
) {
    for (projectile, mut projectile_trans) in projectiles.iter_mut() {
        projectile_trans.translation += projectile.velocity.extend(0.) * time.delta_secs();
    }
}

pub fn projectiles_remover(
    mut command: Commands,
    mut projectiles: Query<(&mut Projectile, Entity)>,
    time: Res<Time>,
) {
    for (mut projectile, projectile_entity) in projectiles.iter_mut() {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0. {
            command.entity(projectile_entity).despawn();
        }
    }
}
