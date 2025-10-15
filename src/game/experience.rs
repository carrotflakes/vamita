use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use super::components::{ExperienceOrb, Velocity};
use super::constants::{
    EXPERIENCE_ORB_IDLE_DAMPING, EXPERIENCE_ORB_INITIAL_SPEED_MAX,
    EXPERIENCE_ORB_INITIAL_SPEED_MIN, EXPERIENCE_ORB_MAGNET_ACCEL,
    EXPERIENCE_ORB_MAGNET_BASE_SPEED, EXPERIENCE_ORB_MAGNET_MAX_SPEED,
    EXPERIENCE_ORB_MAGNET_RADIUS, EXPERIENCE_ORB_SIZE, PLAYER_SIZE,
};
use super::powerup::PowerUpProgress;
use super::resources::ExperienceOrbSound;
use crate::MainState;
use crate::audio::{SEVolume, spawn_se};
use crate::game::components::LevelEntity;
use crate::game::player::{Player, PlayerStats};

pub fn experience_orb_behavior(
    mut commands: Commands,
    time: Res<Time>,
    mut orbs: Query<(Entity, &mut ExperienceOrb, &mut Velocity, &Transform)>,
    mut player_stats: ResMut<PlayerStats>,
    mut powerup_progress: ResMut<PowerUpProgress>,
    player_query: Query<&Transform, With<Player>>,
    exp_sound: Res<ExperienceOrbSound>,
    se_volume: Res<SEVolume>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.xy();
    let pickup_radius = (PLAYER_SIZE.x + EXPERIENCE_ORB_SIZE) * 0.5;

    for (entity, mut orb, mut velocity, transform) in &mut orbs {
        let orb_pos = transform.translation.xy();
        let to_player = player_pos - orb_pos;
        let distance = to_player.length();

        if !orb.magnetized && distance <= EXPERIENCE_ORB_MAGNET_RADIUS {
            orb.magnetized = true;
        }

        if distance <= pickup_radius {
            player_stats.experience = player_stats.experience.saturating_add(orb.value);
            powerup_progress.add_experience(orb.value);
            commands.entity(entity).despawn();
            spawn_se(&mut commands, &*se_volume, &exp_sound.0);
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

#[derive(Resource)]
pub struct OrbMesh((Mesh2d, MeshMaterial2d<ColorMaterial>));

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let orb_mesh = OrbMesh((
        Mesh2d(meshes.add(RegularPolygon {
            circumcircle: Circle {
                radius: EXPERIENCE_ORB_SIZE,
            },
            sides: 3,
        })),
        MeshMaterial2d(materials.add(Color::srgba(0.3, 0.9, 0.5, 0.95))),
    ));
    commands.insert_resource(orb_mesh);
}

pub fn spawn_experience_orb(
    commands: &mut Commands,
    orb: &Res<OrbMesh>,
    position: Vec2,
    value: u32,
) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0f32..TAU);
    let speed =
        rng.random_range(EXPERIENCE_ORB_INITIAL_SPEED_MIN..EXPERIENCE_ORB_INITIAL_SPEED_MAX);
    let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
    commands.spawn((
        DespawnOnExit(MainState::Game),
        LevelEntity,
        orb.0.clone(),
        Transform::from_translation(Vec3::new(position.x, position.y, 0.8)),
        ExperienceOrb {
            value,
            magnetized: false,
        },
        Velocity(velocity),
    ));
}
