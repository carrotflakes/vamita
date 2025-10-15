use std::collections::HashSet;
use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use super::components::{BombExplosion, Lifetime, Particle, Projectile, Velocity};
use super::constants::{
    ENEMY_DEATH_PARTICLE_LIFETIME, ENEMY_DEATH_PARTICLE_SIZE, ENEMY_DEATH_PARTICLE_SPEED,
    ENEMY_DEATH_PARTICLES,
};
use super::enemy::{
    ENEMY_HIT_FLASH_COLOR, ENEMY_HIT_FLASH_DURATION, Enemy, EnemyAttributes, EnemyHitFlash,
};
use super::events::{EnemyKilled, PlayerHit};
use super::resources::{DefeatSound, HitSelfSound, HitSound};
use crate::MainState;
use crate::audio::{SEVolume, spawn_se};
use crate::game::components::{Health, LevelEntity};
use crate::game::experience::{OrbMesh, spawn_experience_orb};
use crate::game::player::Player;
use crate::game::ui::Score;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

pub fn handle_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_hit_messages: MessageWriter<PlayerHit>,
    mut enemy_killed_messages: MessageWriter<EnemyKilled>,
    hit_sound: Res<HitSound>,
    hit_self_sound: Res<HitSelfSound>,
    se_volume: Res<SEVolume>,
    defeat_sound: Res<DefeatSound>,
    orb_mesh: Res<OrbMesh>,
    mut player: Single<(Entity, &mut Health, &Transform), (With<Player>, Without<Enemy>)>,
    mut enemies: Query<
        (
            Entity,
            &mut Health,
            &mut Sprite,
            &Transform,
            &EnemyAttributes,
        ),
        (With<Enemy>, Without<Player>),
    >,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    bomb_explosions: Query<(Entity, &Transform, &BombExplosion)>,
) {
    let (player_entity, player_health, player_transform) = &mut *player;

    let bomb_explosions_data: Vec<(Entity, Vec2, f32)> = bomb_explosions
        .iter()
        .map(|(entity, transform, explosion)| {
            (entity, transform.translation.xy(), explosion.radius)
        })
        .collect();

    let mut enemies_to_despawn: HashSet<Entity> = HashSet::new();
    let mut projectiles_to_despawn: HashSet<Entity> = HashSet::new();

    for (enemy_entity, mut health, mut sprite, transform, attributes) in &mut enemies {
        if enemies_to_despawn.contains(&enemy_entity) {
            continue;
        }

        let enemy_pos = transform.translation.xy();
        let mut enemy_hit = false;

        for (projectile_entity, projectile, transform) in &projectiles {
            if projectiles_to_despawn.contains(&projectile_entity) {
                continue;
            }
            let projectile_pos = transform.translation.xy();
            if collide(enemy_pos, projectile_pos, 12.0) {
                projectiles_to_despawn.insert(projectile_entity);
                health.current -= projectile.damage;
                enemy_hit = true;
            }
        }

        for (_, explosion_pos, radius) in &bomb_explosions_data {
            if collide(enemy_pos, *explosion_pos, *radius) {
                health.current -= 1;
                enemy_hit = true;
                // Only apply damage from one explosion per frame
                break;
            }
        }

        if enemy_hit {
            if health.current <= 0 {
                enemies_to_despawn.insert(enemy_entity);
                score.0 += attributes.score_value;
                enemy_killed_messages.write(EnemyKilled);
                spawn_se(&mut commands, &*se_volume, &hit_sound.0);
                spawn_enemy_death_particles(&mut commands, enemy_pos, attributes.color);
                spawn_experience_orb(&mut commands, &orb_mesh, enemy_pos, attributes.xp_value);
            } else {
                sprite.color = ENEMY_HIT_FLASH_COLOR;
                commands.entity(enemy_entity).insert(EnemyHitFlash {
                    timer: Timer::from_seconds(ENEMY_HIT_FLASH_DURATION, TimerMode::Once),
                });
            }
        }

        if enemies_to_despawn.contains(&enemy_entity) {
            continue;
        }

        if collide(enemy_pos, player_transform.translation.xy(), 12.0) {
            enemies_to_despawn.insert(enemy_entity);
            spawn_enemy_death_particles(&mut commands, enemy_pos, attributes.color);
            if player_health.current > 0 {
                player_health.current -= attributes.damage;
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

    if player_health.current <= 0 {
        spawn_se(&mut commands, &*se_volume, &defeat_sound.0);
        commands.entity(*player_entity).despawn();
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

fn collide(a_pos: Vec2, b_pos: Vec2, dist: f32) -> bool {
    a_pos.distance(b_pos) < dist
}
