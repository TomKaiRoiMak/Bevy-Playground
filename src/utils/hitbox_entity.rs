use bevy::{color::palettes::css::GREEN, math::bounding::BoundingVolume, prelude::*};

use crate::{Enemy, EntityHitBox, Projectile};

pub fn update_entities_volumes(
    mut gizmos: Gizmos,
    mut entities_hitbox: Query<(&mut Transform, &mut EntityHitBox), With<Enemy>>,
) {
    for (hitbox_trans, mut entity_hitbox) in entities_hitbox.iter_mut() {
        match &mut *entity_hitbox {
            EntityHitBox::Aabb2d(aabb2d) => {
                let half_size = aabb2d.half_size();
                aabb2d.min = hitbox_trans.translation.xy() - half_size;
                aabb2d.max = hitbox_trans.translation.xy() + half_size;

                gizmos.rect_2d(aabb2d.center(), aabb2d.half_size() * 2., GREEN);
            }
            EntityHitBox::BoundingCircle(bounding_circle) => {
                bounding_circle.center = hitbox_trans.translation.xy();
                gizmos.circle_2d(bounding_circle.center(), bounding_circle.radius(), GREEN);
            }
        }
    }
}
pub fn update_projectiles_volumes(
    mut gizmos: Gizmos,
    mut projectiles: Query<(&mut Transform, &mut Projectile)>,
) {
    for (hitbox_trans, mut projectile) in projectiles.iter_mut() {
        let half_size = projectile.hitbox.half_size();
        let hitbox = projectile.hitbox;

        let offset = projectile.velocity.normalize_or_zero() * projectile.offset;
        projectile.hitbox.min = hitbox_trans.translation.xy() - half_size + offset;
        projectile.hitbox.max = hitbox_trans.translation.xy() + half_size + offset;

        gizmos.rect_2d(hitbox.center(), hitbox.half_size() * 2., GREEN);
    }
}
