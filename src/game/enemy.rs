use bevy::prelude::*;
use rand::Rng;
use rand::prelude::*;

use crate::MainState;
use crate::game::combat::EnemySpawnTimer;
use crate::game::components::LevelEntity;
use crate::game::components::Velocity;
use crate::game::constants::ARENA_HALF_SIZE;
use crate::game::player::Player;

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
}

pub const ENEMY_HIT_FLASH_DURATION: f32 = 0.08;
pub const ENEMY_HIT_FLASH_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct EnemyHitFlash {
    pub timer: Timer,
}

#[derive(Component, Copy, Clone)]
pub struct EnemyAttributes {
    pub health: i32,
    pub size: Vec2,
    pub speed: f32,
    pub damage: i32,
    pub score_value: u32,
    pub xp_value: u32,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct EnemyPrototype {
    pub attributes: EnemyAttributes,
    pub weight: f32,
}

#[derive(Resource)]
pub struct EnemyCatalog {
    prototypes: Vec<EnemyPrototype>,
    total_weight: f32,
}

impl EnemyCatalog {
    pub fn new() -> Self {
        let prototypes = vec![
            EnemyPrototype {
                attributes: EnemyAttributes {
                    health: 5,
                    size: Vec2::new(24.0, 24.0),
                    speed: 120.0,
                    damage: 1,
                    score_value: 1,
                    xp_value: 1,
                    color: Color::srgb(0.9, 0.3, 0.3),
                },
                weight: 1.0,
            },
            EnemyPrototype {
                attributes: EnemyAttributes {
                    health: 3,
                    size: Vec2::new(18.0, 18.0),
                    speed: 210.0,
                    damage: 1,
                    score_value: 2,
                    xp_value: 2,
                    color: Color::srgb(0.95, 0.6, 0.2),
                },
                weight: 0.6,
            },
            EnemyPrototype {
                attributes: EnemyAttributes {
                    health: 10,
                    size: Vec2::new(36.0, 36.0),
                    speed: 80.0,
                    damage: 2,
                    score_value: 3,
                    xp_value: 3,
                    color: Color::srgb(0.6, 0.1, 0.1),
                },
                weight: 0.3,
            },
        ];

        let total_weight = prototypes
            .iter()
            .map(|p| p.weight)
            .sum::<f32>()
            .max(f32::EPSILON);

        Self {
            prototypes,
            total_weight,
        }
    }

    pub fn random_prototype<'a>(&'a self, rng: &mut ThreadRng) -> &'a EnemyPrototype {
        let mut roll = rng.random_range(0.0..self.total_weight);
        for prototype in &self.prototypes {
            if roll <= prototype.weight {
                return prototype;
            }
            roll -= prototype.weight;
        }
        &self.prototypes[0]
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    enemy_catalog: Res<EnemyCatalog>,
    player_query: Query<&Transform, With<Player>>,
) {
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
            DespawnOnExit(MainState::Game),
            LevelEntity,
            Sprite {
                color: attributes.color,
                custom_size: Some(attributes.size),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 0.0)),
            Enemy {
                health: attributes.health,
            },
            attributes,
            Velocity(dir * attributes.speed),
        ));
    }
}

pub fn update_enemy_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &EnemyAttributes, &mut Sprite, &mut EnemyHitFlash)>,
) {
    for (entity, attributes, mut sprite, mut flash) in &mut query {
        flash.timer.tick(time.delta());
        if flash.timer.is_finished() {
            sprite.color = attributes.color;
            commands.entity(entity).remove::<EnemyHitFlash>();
        }
    }
}
