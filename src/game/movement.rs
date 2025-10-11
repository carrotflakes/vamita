use bevy::prelude::*;

use super::components::{Enemy, EnemyAttributes, Lifetime, Player, Projectile, Velocity};
use super::constants::ARENA_HALF_SIZE;
use super::resources::PauseState;

pub fn update_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    let delta = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * delta;
    }
}

pub fn enemy_seek_player(
    mut enemies: Query<(&Transform, &mut Velocity, &EnemyAttributes), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_pos = player_transform.translation.xy();
    for (transform, mut velocity, attributes) in &mut enemies {
        let dir = (player_pos - transform.translation.xy()).normalize_or_zero();
        velocity.0 = dir * attributes.speed;
    }
}

pub fn update_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    let delta = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * delta;
    }
}

pub fn decay_lifetimes(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn constrain_to_arena(
    mut query: Query<&mut Transform, With<Player>>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    for mut transform in &mut query {
        transform.translation.x = transform
            .translation
            .x
            .clamp(-ARENA_HALF_SIZE, ARENA_HALF_SIZE);
        transform.translation.y = transform
            .translation
            .y
            .clamp(-ARENA_HALF_SIZE, ARENA_HALF_SIZE);
    }
}
