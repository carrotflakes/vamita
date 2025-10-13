use std::collections::HashSet;
use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use super::components::{
    BombExplosion, Enemy, EnemyAttributes, ExperienceOrb, Lifetime, Particle, Projectile, Velocity,
};
use super::constants::{
    ENEMY_DEATH_PARTICLE_LIFETIME, ENEMY_DEATH_PARTICLE_SIZE, ENEMY_DEATH_PARTICLE_SPEED,
    ENEMY_DEATH_PARTICLES, EXPERIENCE_ORB_INITIAL_SPEED_MAX, EXPERIENCE_ORB_INITIAL_SPEED_MIN,
    EXPERIENCE_ORB_SIZE,
};
use super::events::{EnemyKilled, PlayerHit};
use super::resources::{DefeatSound, HitSelfSound, HitSound};
use crate::MainState;
use crate::audio::{SEVolume, spawn_se};
use crate::game::components::LevelEntity;
use crate::game::player::{Player, PlayerStats};
use crate::game::ui::Score;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

pub fn handle_collisions(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut score: ResMut<Score>,
    mut player_hit_messages: MessageWriter<PlayerHit>,
    mut enemy_killed_messages: MessageWriter<EnemyKilled>,
    hit_sound: Res<HitSound>,
    hit_self_sound: Res<HitSelfSound>,
    se_volume: Res<SEVolume>,
    defeat_sound: Res<DefeatSound>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<(Entity, &Transform, &EnemyAttributes), With<Enemy>>,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    bomb_explosions: Query<(Entity, &Transform, &BombExplosion)>,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    let enemies_data: Vec<(Entity, Vec2, EnemyAttributes)> = enemies
        .iter()
        .map(|(entity, transform, attributes)| (entity, transform.translation.xy(), *attributes))
        .collect();
    let projectiles_data: Vec<(Entity, Vec2)> = projectiles
        .iter()
        .map(|(entity, transform)| (entity, transform.translation.xy()))
        .collect();
    let bomb_explosions_data: Vec<(Entity, Vec2, f32)> = bomb_explosions
        .iter()
        .map(|(entity, transform, explosion)| {
            (entity, transform.translation.xy(), explosion.radius)
        })
        .collect();

    let mut enemies_to_despawn: HashSet<Entity> = HashSet::new();
    let mut projectiles_to_despawn: HashSet<Entity> = HashSet::new();

    // enemy-projectile collisions
    for (enemy_entity, enemy_pos, attributes) in &enemies_data {
        if enemies_to_despawn.contains(enemy_entity) {
            continue;
        }

        for (projectile_entity, projectile_pos) in &projectiles_data {
            if projectiles_to_despawn.contains(projectile_entity) {
                continue;
            }
            if collide(*enemy_pos, *projectile_pos, 12.0) {
                enemies_to_despawn.insert(*enemy_entity);
                projectiles_to_despawn.insert(*projectile_entity);
                score.0 += attributes.score_value;
                enemy_killed_messages.write(EnemyKilled);
                spawn_se(&mut commands, &*se_volume, &hit_sound.0);
                spawn_enemy_death_particles(&mut commands, *enemy_pos, attributes.color);
                spawn_experience_orb(&mut commands, *enemy_pos, attributes.xp_value);
                break;
            }
        }

        for (_, explosion_pos, radius) in &bomb_explosions_data {
            if collide(*enemy_pos, *explosion_pos, *radius) {
                enemies_to_despawn.insert(*enemy_entity);
                score.0 += attributes.score_value;
                enemy_killed_messages.write(EnemyKilled);
                spawn_se(&mut commands, &*se_volume, &hit_sound.0);
                spawn_enemy_death_particles(&mut commands, *enemy_pos, attributes.color);
                spawn_experience_orb(&mut commands, *enemy_pos, attributes.xp_value);
                break;
            }
        }
    }

    // enemy-player collisions
    for (enemy_entity, enemy_pos, attributes) in &enemies_data {
        if enemies_to_despawn.contains(enemy_entity) {
            continue;
        }

        if collide(*enemy_pos, player_transform.translation.xy(), 12.0) {
            enemies_to_despawn.insert(*enemy_entity);
            spawn_enemy_death_particles(&mut commands, *enemy_pos, attributes.color);
            if player_stats.health > 0 {
                player_stats.health = (player_stats.health - attributes.damage).max(0);
                player_hit_messages.write(PlayerHit);
                spawn_se(&mut commands, &*se_volume, &hit_self_sound.0);
            }
        }
    }

    for entity in enemies_to_despawn.into_iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectiles_to_despawn.into_iter() {
        commands.entity(entity).despawn();
    }

    if player_stats.health <= 0 {
        spawn_se(&mut commands, &*se_volume, &defeat_sound.0);
        commands.entity(player_entity).despawn();
    }
}

fn spawn_enemy_death_particles(commands: &mut Commands, position: Vec2, color: Color) {
    let mut rng = rand::rng();
    commands.spawn_batch(
        (0..ENEMY_DEATH_PARTICLES)
            .map(|_| {
                let angle = rng.random_range(0.0f32..TAU);
                let speed =
                    rng.random_range(ENEMY_DEATH_PARTICLE_SPEED * 0.6..ENEMY_DEATH_PARTICLE_SPEED);
                let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
                let lifetime = rng.random_range(
                    ENEMY_DEATH_PARTICLE_LIFETIME * 0.7..ENEMY_DEATH_PARTICLE_LIFETIME,
                );
                (
                    DespawnOnExit(MainState::Game),
                    LevelEntity,
                    Sprite::from_color(color, ENEMY_DEATH_PARTICLE_SIZE),
                    Transform::from_translation(Vec3::new(position.x, position.y, 5.0)),
                    Velocity(velocity),
                    Lifetime {
                        timer: Timer::from_seconds(lifetime, TimerMode::Once),
                    },
                    Particle,
                )
            })
            .collect::<Vec<_>>(),
    );
}

fn spawn_experience_orb(commands: &mut Commands, position: Vec2, value: u32) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0f32..TAU);
    let speed =
        rng.random_range(EXPERIENCE_ORB_INITIAL_SPEED_MIN..EXPERIENCE_ORB_INITIAL_SPEED_MAX);
    let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
    commands.spawn((
        DespawnOnExit(MainState::Game),
        LevelEntity,
        Sprite {
            color: Color::srgba(0.3, 0.9, 0.5, 0.95),
            custom_size: Some(EXPERIENCE_ORB_SIZE),
            ..default()
        },
        Transform::from_translation(Vec3::new(position.x, position.y, 0.8)),
        ExperienceOrb {
            value,
            magnetized: false,
        },
        Velocity(velocity),
    ));
}

fn collide(a_pos: Vec2, b_pos: Vec2, dist: f32) -> bool {
    a_pos.distance(b_pos) < dist
}
