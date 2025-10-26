use bevy::prelude::*;

use crate::game::player::Player;

use super::components::{Lifetime, Projectile, Velocity};
use super::constants::{ENEMY_REPULSION_RADIUS_FACTOR, ENEMY_REPULSION_STRENGTH};
use super::enemy::{Enemy, EnemyAttributes};

pub fn update_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * delta;
    }
}

pub fn enemy_seek_player(
    mut enemies: Query<(&Transform, &mut Velocity, &EnemyAttributes), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_pos = player_transform.translation.xy();
    for (transform, mut velocity, attributes) in &mut enemies {
        let dir = (player_pos - transform.translation.xy()).normalize_or_zero();
        velocity.0 = dir * attributes.speed;
    }

    // Enemy repulsion
    {
        let mut combinations = enemies.iter_combinations_mut();
        while let Some(
            [
                (transform_a, mut velocity_a, attributes_a),
                (transform_b, mut velocity_b, attributes_b),
            ],
        ) = combinations.fetch_next()
        {
            let offset = transform_b.translation.xy() - transform_a.translation.xy();
            let distance_sq = offset.length_squared();
            let radius_a = enemy_radius(attributes_a);
            let radius_b = enemy_radius(attributes_b);
            let min_distance = (radius_a + radius_b) * ENEMY_REPULSION_RADIUS_FACTOR;

            let distance = distance_sq.sqrt();
            if distance >= min_distance {
                continue;
            }

            let normal = if distance > f32::EPSILON {
                offset / distance
            } else {
                Vec2::X
            };
            let effective_distance = distance.max(0.001);
            let overlap_ratio = (min_distance - effective_distance) / min_distance;
            let repulsion_speed = overlap_ratio
                * ENEMY_REPULSION_STRENGTH
                * attributes_a.speed.min(attributes_b.speed);
            if repulsion_speed <= 0.0 {
                continue;
            }

            let impulse = normal * repulsion_speed;
            velocity_a.0 -= impulse;
            velocity_b.0 += impulse;
        }
    }
}

fn enemy_radius(attributes: &EnemyAttributes) -> f32 {
    attributes.size.x.max(attributes.size.y) * 0.5
}

pub fn update_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * delta;
        transform.rotation = Quat::from_rotation_z(velocity.y.atan2(velocity.x));
    }
}

pub fn decay_lifetimes(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[allow(dead_code)]
pub fn center_camera_on_player(
    player_transform: Single<&Transform, With<Player>>,
    mut camera_transform: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_translation = player_transform.translation;
    camera_transform.translation.x = player_translation.x;
    camera_transform.translation.y = player_translation.y;
}
