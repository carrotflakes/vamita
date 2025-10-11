use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};

use crate::components::{HealthText, Player, Velocity};
use crate::constants::{PLAYER_MAX_HEALTH, PLAYER_SIZE};
use crate::resources::{HitSound, ShootSound, UiAssets};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    let hit_sound_handle = asset_server.load("sounds/hit.wav");
    let shoot_sound_handle = asset_server.load("sounds/shoot.wav");
    let bgm_handle: Handle<AudioSource> = asset_server.load("sounds/vamita-0.mp3");

    commands.insert_resource(UiAssets {
        font: font_handle.clone(),
    });
    commands.insert_resource(HitSound(hit_sound_handle));
    commands.insert_resource(ShootSound(shoot_sound_handle));

    commands.spawn(Camera2d);

    commands.spawn((
        AudioPlayer(bgm_handle),
        PlaybackSettings::LOOP,
    ));

    spawn_player(&mut commands, &font_handle);
}

pub(crate) fn spawn_player(commands: &mut Commands, handle_font: &Handle<Font>) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), PLAYER_SIZE),
        Transform::default(),
        Player,
        Velocity(Vec2::ZERO),
        children!((
            Text2d(format!("HP: {}", PLAYER_MAX_HEALTH)),
            TextFont {
                font: handle_font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 28.0, 1.0)),
            HealthText,
        )),
    ));
}
