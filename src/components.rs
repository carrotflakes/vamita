use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Projectile;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component, Copy, Clone)]
pub struct EnemyAttributes {
    pub size: Vec2,
    pub speed: f32,
    pub damage: i32,
    pub score_value: u32,
    pub color: Color,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct Particle;
