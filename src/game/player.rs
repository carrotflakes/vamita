use bevy::input::ButtonInput;
use bevy::prelude::*;

use super::components::{Bomb, BombExplosion, Lifetime, Velocity};
use super::constants::{BOMB_EXPLOSION_DURATION, BOMB_FUSE};
use super::enemy::Enemy;
use super::powerup::PlayerUpgrades;
use super::resources::{BombSound, ShootSound};
use crate::MainState;
use crate::audio::{SEVolume, spawn_se};
use crate::game::components::{LevelEntity, Projectile};
use crate::game::constants::{ARENA_HALF_SIZE, PLAYER_SIZE};
use crate::game::ui::HealthText;

use rand::Rng;
use std::f32::consts::TAU;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerStats {
    pub health: i32,
    pub experience: u32,
}

#[derive(Resource)]
pub struct ShootTimer(pub Timer);

#[derive(Resource)]
pub struct BombTimer(pub Timer);

pub fn player_input(
    mut query: Query<&mut Velocity, With<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
    upgrades: Res<PlayerUpgrades>,
) {
    if let Ok(mut velocity) = query.single_mut() {
        let mut dir = Vec2::ZERO;
        if kb.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            dir.y += 1.0;
        }
        if kb.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            dir.y -= 1.0;
        }
        if kb.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            dir.x -= 1.0;
        }
        if kb.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            dir.x += 1.0;
        }
        let dir = dir.normalize_or_zero();
        velocity.0 = dir * upgrades.movement_speed();
    }
}

pub fn player_auto_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
    player_query: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    shoot_sound: Res<ShootSound>,
    se_volume: Res<SEVolume>,
    upgrades: Res<PlayerUpgrades>,
) {
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
            DespawnOnExit(MainState::Game),
            LevelEntity,
            Sprite {
                color: Color::srgba(1.0, 1.0, 0.0, 0.8),
                custom_size: Some(Vec2::new(12.0, 6.0)),
                ..default()
            },
            Transform::from_translation(transform.translation + Vec3::new(0.0, 0.0, 1.0))
                .with_rotation(Quat::from_rotation_z(dir.y.atan2(dir.x))),
            Projectile {
                damage: upgrades.projectile_damage(),
            },
            Velocity(dir * upgrades.projectile_speed()),
            Lifetime {
                timer: Timer::from_seconds(1.2, TimerMode::Once),
            },
        ));
        spawn_se(&mut commands, &*se_volume, &shoot_sound.0);
    }
}

pub fn player_place_bomb(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<BombTimer>,
    player_query: Query<&Transform, With<Player>>,
    upgrades: Res<PlayerUpgrades>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let Ok(transform) = player_query.single() else {
            return;
        };

        let position = transform.translation;
        commands.spawn((
            DespawnOnExit(MainState::Game),
            LevelEntity,
            Sprite::from_color(Color::srgba(1.0, 0.45, 0.1, 0.9), Vec2::splat(24.0)),
            Transform::from_translation(Vec3::new(position.x, position.y, 0.4)),
            Bomb {
                timer: Timer::from_seconds(BOMB_FUSE, TimerMode::Once),
                blink_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                radius: upgrades.bomb_explosion_radius(),
                visible: true,
            },
        ));
    }
}

pub fn update_bombs(
    mut commands: Commands,
    time: Res<Time>,
    mut bombs: Query<(Entity, &Transform, &mut Sprite, &mut Bomb)>,
    bomb_sound: Res<BombSound>,
    se_volume: Res<SEVolume>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, mut sprite, mut bomb) in &mut bombs {
        if bomb.blink_timer.tick(time.delta()).just_finished() {
            bomb.visible = !bomb.visible;
            sprite.color.set_alpha(if bomb.visible { 0.9 } else { 0.4 });
        }

        if bomb.timer.tick(time.delta()).just_finished() {
            let position = transform.translation;
            commands.entity(entity).despawn();
            commands.spawn((
                DespawnOnExit(MainState::Game),
                LevelEntity,
                Mesh2d(meshes.add(Circle {
                    radius: bomb.radius,
                })),
                MeshMaterial2d(materials.add(Color::srgba(1.0, 0.6, 0.2, 0.35))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.5)),
                BombExplosion {
                    radius: bomb.radius,
                },
                Lifetime {
                    timer: Timer::from_seconds(BOMB_EXPLOSION_DURATION, TimerMode::Once),
                },
            ));
            spawn_se(&mut commands, &*se_volume, &bomb_sound.0);
        }
    }
}

pub fn constrain_to_arena(mut query: Query<&mut Transform, With<Player>>) {
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

pub fn spawn_player(commands: &mut Commands, handle_font: &Handle<Font>) {
    commands.spawn((
        DespawnOnExit(MainState::Game),
        LevelEntity,
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), PLAYER_SIZE),
        Transform::default(),
        Player,
        Velocity(Vec2::ZERO),
        children!((
            Text2d("HP: ".to_string()),
            TextFont {
                font: handle_font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 28.0, 1.0)),
            HealthText,
            children!(TextSpan::default())
        )),
    ));
}

pub fn update_health_text(
    player_stats: Res<PlayerStats>,
    health_root: Single<Entity, (With<HealthText>, With<Text2d>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*health_root, 1) = player_stats.health.to_string();
}
