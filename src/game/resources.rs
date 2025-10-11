use bevy::prelude::*;
use rand::Rng;
use rand::prelude::*;

use super::components::EnemyAttributes;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct ShootTimer(pub Timer);

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct PlayerStats {
    pub health: i32,
}

#[derive(Resource, Default)]
pub struct PauseState {
    pub paused: bool,
}

#[derive(Resource, Clone)]
pub struct HitSound(pub Handle<AudioSource>);

#[derive(Resource, Clone)]
pub struct ShootSound(pub Handle<AudioSource>);

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
                    size: Vec2::new(24.0, 24.0),
                    speed: 120.0,
                    damage: 1,
                    score_value: 1,
                    color: Color::srgb(0.9, 0.3, 0.3),
                },
                weight: 1.0,
            },
            EnemyPrototype {
                attributes: EnemyAttributes {
                    size: Vec2::new(18.0, 18.0),
                    speed: 210.0,
                    damage: 1,
                    score_value: 2,
                    color: Color::srgb(0.95, 0.6, 0.2),
                },
                weight: 0.6,
            },
            EnemyPrototype {
                attributes: EnemyAttributes {
                    size: Vec2::new(36.0, 36.0),
                    speed: 80.0,
                    damage: 2,
                    score_value: 3,
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
