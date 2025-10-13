use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct HitSound(pub Handle<AudioSource>);

#[derive(Resource, Clone)]
pub struct HitSelfSound(pub Handle<AudioSource>);

#[derive(Resource, Clone)]
pub struct ShootSound(pub Handle<AudioSource>);

#[derive(Resource, Clone)]
pub struct ExperienceOrbSound(pub Handle<AudioSource>);

#[derive(Resource, Clone)]
pub struct BombSound(pub Handle<AudioSource>);

#[derive(Resource, Clone)]
pub struct DefeatSound(pub Handle<AudioSource>);
