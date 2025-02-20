use bevy::{
    color::palettes::css::{GREEN, RED},
    math::bounding::BoundingVolume,
    prelude::*,
};

use crate::{DefaultMelee, DEFAULT_MELEE_OFFSET};

pub fn move_and_show_hitbox(
    mut gizmos: Gizmos,
    hitbox: Single<(&mut DefaultMelee, &Transform)>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let (mut default_melee, entity_trans) = hitbox.into_inner();

    if inputs.just_pressed(KeyCode::KeyH) {
        default_melee.is_active = !default_melee.is_active;
    }

    //update volumes
    if default_melee.is_active {
        let half_size = default_melee.hitbox.half_size();
        let offet_y = Vec2::new(0., DEFAULT_MELEE_OFFSET);
        let entity_trans = entity_trans.translation.truncate();
        default_melee.hitbox.min = entity_trans - half_size + offet_y;
        default_melee.hitbox.max = entity_trans + half_size + offet_y;
    }


    gizmos.rect_2d(
        default_melee.hitbox.center(),
        default_melee.hitbox.half_size() * 2.,
        if default_melee.is_active { GREEN } else { RED },
    );
}
