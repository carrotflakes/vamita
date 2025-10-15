use bevy::prelude::*;

/// Entities with this component will be despawned on game reset.
#[derive(Component)]
pub struct LevelEntity;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self { current: 1, max: 1 }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
}

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

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct ExperienceOrb {
    pub value: u32,
    pub magnetized: bool,
}
