use core::f32;

use bevy::{
    color::palettes::css::{GRAY, PURPLE, RED},
    math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
    window::PrimaryWindow,
    winit::WinitWindows,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_light_2d::{light::PointLight2d, plugin::Light2dPlugin};
mod utils;
use utils::{
    avoid_overlap::avoid_overlaping,
    damage_entity::{default_melee_attack, projectiles_damage},
    default_melee::move_and_show_hitbox,
    hitbox_entity::{update_entities_volumes, update_projectiles_volumes},
    path_finding::enemy_follow_player,
    projectile::{move_projectiles, projectiles_remover, shoot_default_projectile},
    spawn_entity::spawn_enemy_random_position,
    stats::buff_handle,
    utils::{get_screen_size, update_immunity_timer},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin, WorldInspectorPlugin::new()))
        .register_type::<Enemy>()
        .register_type::<Player>()
        .register_type::<Projectile>()
        .register_type::<EntityHitBox>()
        .register_type::<DefaultMelee>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                invisible.chain(),
                spawn_enemy,
                avoid_overlaping,
                update_immunity_timer,
                spawn_enemy2,
                //hitbox_entity
                (enemy_follow_player, update_entities_volumes).chain(),
                update_projectiles_volumes,
                //projectiles
                shoot_default_projectile,
                projectiles_damage,
                move_projectiles,
                projectiles_remover,
                (default_melee_attack, move_and_show_hitbox).chain(),
                buff_handle,
            ),
        )
        .insert_resource(DupeTimer(Timer::from_seconds(2., TimerMode::Repeating)))
        .run();
}

const PLAYER_SPEED: f32 = 400.;
const ENEMY_SPEED: f32 = 100.;
const IMMUNITY_TIMER: f32 = 1.;
const DEFAULT_MELEE_OFFSET: f32 = 80.;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player {
    base_hp: f32,
    damage_multiplier: f32,
    immunity_timer: f32,
}
#[derive(Resource)]
struct DupeTimer(Timer);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Enemy {
    base_hp: f32,
    immunity_timer: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct DefaultMelee {
    hitbox: Aabb2d,
    base_damage: f32,
    is_active: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Projectile {
    hitbox: Aabb2d,
    offset: f32,
    base_damage: f32,
    lifetime: f32,
    piercing: i32,
    velocity: Vec2,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum EntityHitBox {
    Aabb2d(Aabb2d),
    BoundingCircle(BoundingCircle),
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(0., 0., -999.),
        Mesh2d(meshes.add(Rectangle::new(1000., 1000.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GRAY))),
    ));
    commands.spawn((
        Player {
            base_hp: f32::MAX,
            immunity_timer: 0.,
            damage_multiplier: 1.,
        },
        DefaultMelee {
            hitbox: Aabb2d::new(Vec2::ZERO, Vec2::new(40., 40.)),
            is_active: false,
            base_damage: 3.,
        },
        Transform::from_xyz(0., 0., 999.),
        Visibility::Visible,
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(PURPLE))),
        PointLight2d {
            radius: 400.,
            intensity: 3.,
            ..Default::default()
        },
    ));
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    inputs: Res<ButtonInput<KeyCode>>,
    inputs_mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();
    let Some(window) = window.cursor_position() else {
        return;
    };
    let Ok(pointing) = camera.viewport_to_world_2d(camera_transform, window) else {
        return;
    };

    if inputs_mouse.just_pressed(MouseButton::Right) {
        commands.spawn((
            Enemy {
                base_hp: 5.,
                immunity_timer: 0.,
            },
            EntityHitBox::BoundingCircle(BoundingCircle::new(Vec2::ZERO, 20.)),
            Transform::from_translation(pointing.extend(1.)),
            Visibility::Visible,
            Mesh2d(meshes.add(Circle::new(20.))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ));
    }
    if inputs.just_pressed(KeyCode::Enter) {
        commands.spawn((
            Enemy {
                base_hp: 5.,
                immunity_timer: 0.,
            },
            EntityHitBox::BoundingCircle(BoundingCircle::new(Vec2::ZERO, 20.)),
            Transform::from_xyz(0., 0., 1.),
            Visibility::Visible,
            Mesh2d(meshes.add(Circle::new(20.))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        ));
    }
}

fn spawn_enemy2(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    inputs: Res<ButtonInput<MouseButton>>,
    winit_windows: NonSend<WinitWindows>,
    window_query: Single<Entity, With<PrimaryWindow>>,
) {
    if inputs.pressed(MouseButton::Right) {
        spawn_enemy_random_position(
            commands,
            &mut meshes,
            &mut materials,
            get_screen_size(winit_windows, window_query),
        );
    }
}

fn invisible(
    player: Single<(&mut Visibility, &mut Transform), With<Player>>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_visiblity, mut player_tranform) = player.into_inner();
    if inputs.just_pressed(KeyCode::Space) {
        player_visiblity.toggle_visible_hidden();
    }

    let mut direction = Vec2::ZERO;

    if inputs.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if inputs.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if inputs.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if inputs.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player_tranform.translation += move_delta.extend(0.);
}
