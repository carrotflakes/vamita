use bevy::prelude::*;

/// Entities with this component will be despawned on game reset.
#[derive(Component)]
pub struct LevelEntity;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Bomb {
    pub timer: Timer,
    pub blink_timer: Timer,
    pub radius: f32,
    pub visible: bool,
}

#[derive(Component)]
pub struct BombExplosion {
    pub radius: f32,
}

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
    pub xp_value: u32,
    pub color: Color,
}

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct ExperienceOrb {
    pub value: u32,
    pub magnetized: bool,
}
