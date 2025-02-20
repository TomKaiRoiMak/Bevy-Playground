use bevy::{
    math::bounding::IntersectsVolume, prelude::*,
};

use crate::{DefaultMelee, Enemy, EntityHitBox, Player, Projectile, IMMUNITY_TIMER};



pub fn default_melee_attack(
    player_query: Single<(&DefaultMelee, &Transform), With<Player>>,
    mut enemies: Query<
        (&mut Enemy, &mut Transform, &EntityHitBox, Entity),
        (With<Enemy>, Without<Player>),
    >,
    mut commands: Commands,
) {
    
    let (player, player_trans) = *player_query;
    if player.is_active {
        let (player_hitbox, player_damage) = (player.hitbox, player.base_damage);

        for (mut enemy, mut enemy_trans, enemy_hitbox, entity) in enemies.iter_mut() {
            let hit_entity = match enemy_hitbox {
                EntityHitBox::Aabb2d(aabb2d) => player_hitbox.intersects(aabb2d),
                EntityHitBox::BoundingCircle(bounding_circle) => {
                    player_hitbox.intersects(bounding_circle)
                }
            };
            if hit_entity {
                if enemy.immunity_timer <= 0. {
                    enemy.base_hp -= player_damage;
                    enemy.immunity_timer = IMMUNITY_TIMER;

                    if enemy.base_hp <= 0. {
                        commands.entity(entity).despawn();
                    }

                    let direction = (enemy_trans.translation - player_trans.translation)
                        .xy()
                        .normalize();
                    enemy_trans.translation += direction.extend(0.) * 40.;
                }
            }
        }
    }
}

pub fn projectiles_damage(
    mut commands: Commands,
    mut projectiles_query: Query<(&mut Projectile, Entity)>,
    mut enemies_query: Query<
        (&mut Enemy, &mut Transform, &EntityHitBox, Entity),
        (With<Enemy>, Without<Player>),
    >,
    player: Single<(&Transform, &Player)>,
    
) {
    let (player_trans, player) = *player;

    for (mut projectile, projectile_entity) in projectiles_query.iter_mut() {
        for (mut enemy, mut enemy_trans, enemy_hitbox, entity) in enemies_query.iter_mut() {
            let hit_entity = match enemy_hitbox {
                EntityHitBox::Aabb2d(aabb2d) => projectile.hitbox.intersects(aabb2d),
                EntityHitBox::BoundingCircle(bounding_circle) => {
                    projectile.hitbox.intersects(bounding_circle)
                }
            };

            if projectile.piercing <= 0 {
                commands.entity(projectile_entity).despawn();
                return;
            }

            if hit_entity {
                if enemy.immunity_timer <= 0.0 {
                    projectile.piercing -= 1;
                    enemy.base_hp -= projectile.base_damage * player.damage_multiplier;
                    enemy.immunity_timer = IMMUNITY_TIMER;

                    if enemy.base_hp <= 0.0 {
                        commands.entity(entity).despawn();

                        
                    }

                    let direction = (enemy_trans.translation - player_trans.translation)
                        .xy()
                        .normalize();
                    enemy_trans.translation += direction.extend(0.0) * 20.0;
                }
            }
        }
    }
}
