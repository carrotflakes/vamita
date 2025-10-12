use bevy::prelude::*;

use super::components::{ExperienceOrb, Player, Velocity};
use super::constants::{
    EXPERIENCE_ORB_IDLE_DAMPING, EXPERIENCE_ORB_MAGNET_ACCEL, EXPERIENCE_ORB_MAGNET_BASE_SPEED,
    EXPERIENCE_ORB_MAGNET_MAX_SPEED, EXPERIENCE_ORB_MAGNET_RADIUS, EXPERIENCE_ORB_SIZE,
    PLAYER_SIZE,
};
use super::resources::{ExperienceOrbSound, PauseState, PlayerStats};

pub fn experience_orb_behavior(
    mut commands: Commands,
    time: Res<Time>,
    mut orbs: Query<(Entity, &mut ExperienceOrb, &mut Velocity, &Transform)>,
    mut player_stats: ResMut<PlayerStats>,
    player_query: Query<&Transform, With<Player>>,
    pause_state: Res<PauseState>,
    exp_sound: Res<ExperienceOrbSound>,
) {
    if pause_state.paused {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.xy();
    let pickup_radius = (PLAYER_SIZE.x + EXPERIENCE_ORB_SIZE.x) * 0.5;

    for (entity, mut orb, mut velocity, transform) in &mut orbs {
        let orb_pos = transform.translation.xy();
        let to_player = player_pos - orb_pos;
        let distance = to_player.length();

        if !orb.magnetized && distance <= EXPERIENCE_ORB_MAGNET_RADIUS {
            orb.magnetized = true;
        }

        if distance <= pickup_radius {
            player_stats.experience = player_stats.experience.saturating_add(orb.value);
            commands.entity(entity).despawn();
            commands.spawn((AudioPlayer(exp_sound.0.clone()), PlaybackSettings::DESPAWN));
            continue;
        }

        if orb.magnetized {
            let dir = to_player.normalize_or_zero();
            let speed = (EXPERIENCE_ORB_MAGNET_BASE_SPEED + EXPERIENCE_ORB_MAGNET_ACCEL * distance)
                .min(EXPERIENCE_ORB_MAGNET_MAX_SPEED);
            velocity.0 = dir * speed;
        } else {
            let damping = (-EXPERIENCE_ORB_IDLE_DAMPING * time.delta_secs()).exp();
            velocity.0 *= damping;
        }
    }
}
