use std::collections::HashSet;
use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use crate::components::{Enemy, EnemyAttributes, Lifetime, Particle, Player, Projectile, Velocity};
use crate::constants::{
    ARENA_HALF_SIZE, ENEMY_DEATH_PARTICLE_LIFETIME, ENEMY_DEATH_PARTICLE_SIZE,
    ENEMY_DEATH_PARTICLE_SPEED, ENEMY_DEATH_PARTICLES, PLAYER_SIZE, PROJECTILE_SIZE,
    PROJECTILE_SPEED,
};
use crate::events::{EnemyKilled, PlayerHit};
use crate::resources::{EnemyCatalog, EnemySpawnTimer, HitSound, PauseState, PlayerStats, Score, ShootSound, ShootTimer};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    enemy_catalog: Res<EnemyCatalog>,
    player_query: Query<&Transform, With<Player>>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        let Ok(player_transform) = player_query.single() else {
            return;
        };

        let mut rng = rand::rng();
        let prototype = enemy_catalog.random_prototype(&mut rng);
        let attributes = prototype.attributes;
        let spawn_side = rng.random_range(0..4);
        let offset = rng.random_range(-ARENA_HALF_SIZE..=ARENA_HALF_SIZE);
        let (x, y) = match spawn_side {
            0 => (-ARENA_HALF_SIZE - 40.0, offset),
            1 => (ARENA_HALF_SIZE + 40.0, offset),
            2 => (offset, -ARENA_HALF_SIZE - 40.0),
            _ => (offset, ARENA_HALF_SIZE + 40.0),
        };

        let target = player_transform.translation.xy();
        let dir = (target - Vec2::new(x, y)).normalize_or_zero();

        commands.spawn((
            Sprite {
                color: attributes.color,
                custom_size: Some(attributes.size),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 0.0)),
            Enemy,
            attributes,
            Velocity(dir * attributes.speed),
        ));
    }
}

pub fn player_auto_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
    player_query: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    shoot_sound: Res<ShootSound>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        let Ok(transform) = player_query.single() else {
            return;
        };

        let origin = transform.translation.xy();
        let dir = enemies
            .iter()
            .map(|enemy_transform| enemy_transform.translation.xy())
            .min_by(|a, b| {
                origin
                    .distance_squared(*a)
                    .partial_cmp(&origin.distance_squared(*b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|target| (target - origin).normalize_or_zero())
            .filter(|dir| dir.length_squared() > 0.0)
            .unwrap_or_else(|| {
                let mut rng = rand::rng();
                let angle = rng.random_range(0.0..TAU);
                Vec2::new(angle.cos(), angle.sin())
            });

        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 1.0, 0.0, 0.8),
                custom_size: Some(PROJECTILE_SIZE),
                ..default()
            },
            Transform::from_translation(transform.translation + Vec3::new(0.0, 0.0, 1.0)),
            Projectile,
            Velocity(dir * PROJECTILE_SPEED),
            Lifetime {
                timer: Timer::from_seconds(1.2, TimerMode::Once),
            },
        ));
        commands.spawn((
            AudioPlayer(shoot_sound.0.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut score: ResMut<Score>,
    mut player_hit_messages: MessageWriter<PlayerHit>,
    mut enemy_killed_messages: MessageWriter<EnemyKilled>,
    hit_sound: Res<HitSound>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<(Entity, &Transform, &EnemyAttributes), With<Enemy>>,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

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
            if collide(
                *enemy_pos,
                attributes.size,
                *projectile_pos,
                PROJECTILE_SIZE,
            ) {
                enemies_to_despawn.insert(*enemy_entity);
                projectiles_to_despawn.insert(*projectile_entity);
                score.0 += attributes.score_value;
                enemy_killed_messages.write(EnemyKilled);
                commands.spawn((
                    AudioPlayer(hit_sound.0.clone()),
                    PlaybackSettings::DESPAWN,
                ));
                spawn_enemy_death_particles(&mut commands, *enemy_pos, attributes.color);
                break;
            }
        }
    }

    // enemy-player collisions
    for (enemy_entity, enemy_pos, attributes) in &enemies_data {
        if enemies_to_despawn.contains(enemy_entity) {
            continue;
        }

        if collide(
            *enemy_pos,
            attributes.size,
            player_transform.translation.xy(),
            PLAYER_SIZE,
        ) {
            enemies_to_despawn.insert(*enemy_entity);
            spawn_enemy_death_particles(&mut commands, *enemy_pos, attributes.color);
            if player_stats.health > 0 {
                player_stats.health = (player_stats.health - attributes.damage).max(0);
                player_hit_messages.write(PlayerHit);
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
        commands.entity(player_entity).despawn();
    }
}

fn spawn_enemy_death_particles(commands: &mut Commands, position: Vec2, color: Color) {
    let mut rng = rand::rng();
    for _ in 0..ENEMY_DEATH_PARTICLES {
        let angle = rng.random_range(0.0f32..TAU);
        let speed =
            rng.random_range(ENEMY_DEATH_PARTICLE_SPEED * 0.6..ENEMY_DEATH_PARTICLE_SPEED);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        let lifetime = rng.random_range(
            ENEMY_DEATH_PARTICLE_LIFETIME * 0.7..ENEMY_DEATH_PARTICLE_LIFETIME,
        );

        commands.spawn((
            Sprite::from_color(color, ENEMY_DEATH_PARTICLE_SIZE),
            Transform::from_translation(Vec3::new(position.x, position.y, 5.0)),
            Velocity(velocity),
            Lifetime {
                timer: Timer::from_seconds(lifetime, TimerMode::Once),
            },
            Particle,
        ));
    }
}

fn collide(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    let collision_x = (a_pos.x - b_pos.x).abs() < (a_size.x + b_size.x) * 0.5;
    let collision_y = (a_pos.y - b_pos.y).abs() < (a_size.y + b_size.y) * 0.5;
    collision_x && collision_y
}
