use bevy::{color::palettes::css::RED, math::bounding::BoundingCircle, prelude::*};
use rand::Rng;

use crate::{Enemy, EntityHitBox};

pub fn spawn_enemy_random_position(
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    screen_size: (f32, f32),
) -> Entity {
    let (width, height) = screen_size;
    let mut rng = rand::thread_rng();
    let spawn_offset: f32 = 50.0;
    let spawn_x = if rng.gen_bool(0.5) {
        if rng.gen_bool(0.5) {
            -width - spawn_offset
        } else {
            width + spawn_offset
        }
    } else {
        rng.gen_range(-width..=width)
    };

    let spawn_y = if spawn_x.abs() > width {
        rng.gen_range(-height..=height)
    } else {
        if rng.gen_bool(0.5) {
            -height - spawn_offset
        } else {
            height + spawn_offset
        }
    };

    commands
        .spawn((
            Enemy {
                base_hp: 5.0,
                immunity_timer: 0.0,
            },
            EntityHitBox::BoundingCircle(BoundingCircle::new(Vec2::ZERO, 20.0)),
            Transform::from_xyz(spawn_x, spawn_y, 1.0),
            Visibility::Visible,
            Mesh2d(meshes.add(Circle::new(20.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ))
        .id()
}
pub fn spawn_enemy_at_entity(
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    entity_trans: Vec3,
) -> Entity {
    let (x, y) = (entity_trans.x, entity_trans.y);

    commands
        .spawn((
            Enemy {
                base_hp: 5.0,
                immunity_timer: 0.0,
            },
            EntityHitBox::BoundingCircle(BoundingCircle::new(Vec2::ZERO, 20.0)),
            Transform::from_xyz(x, y, 1.0),
            Visibility::Visible,
            Mesh2d(meshes.add(Circle::new(20.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ))
        .id()
}
