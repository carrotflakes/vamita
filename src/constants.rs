use bevy::math::Vec2;

pub const PLAYER_SPEED: f32 = 320.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const PROJECTILE_SPEED: f32 = 500.0;
pub const PROJECTILE_SIZE: Vec2 = Vec2::new(12.0, 12.0);
pub const FIRE_RATE: f32 = 0.5;
pub const ENEMY_SPAWN_INTERVAL: f32 = 0.25;
pub const ARENA_HALF_SIZE: f32 = 400.0;
pub const PLAYER_MAX_HEALTH: i32 = 5;
pub const ENEMY_DEATH_PARTICLES: usize = 20;
pub const ENEMY_DEATH_PARTICLE_LIFETIME: f32 = 0.35;
pub const ENEMY_DEATH_PARTICLE_SPEED: f32 = 180.0;
pub const ENEMY_DEATH_PARTICLE_SIZE: Vec2 = Vec2::splat(3.0);
