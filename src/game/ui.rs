use bevy::prelude::*;

use super::powerup::PowerUpProgress;
use crate::MainState;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthBarRoot;
#[derive(Component)]
pub struct HealthBarFill {
    pub full_size: Vec2,
}

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct ScoreboardUi;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Single<Entity, With<Camera2d>>,
) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.entity(*camera).with_children(|parent| {
        parent.spawn((
            DespawnOnExit(MainState::Game),
            ScoreboardUi,
            Text2d("Score: ".to_string()),
            TextFont {
                font: font_handle.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(-300.0, 250.0, 1.0)),
            ScoreText,
            children![(
                TextSpan::default(),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            )],
        ));
    });
}

pub fn update_score_text(
    score: Res<Score>,
    progress: Res<PowerUpProgress>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text2d>)>,
    mut writer: TextUiWriter,
) {
    let (current, required) = progress.progress_to_next();
    *writer.text(*score_root, 1) = format!("{}\nXP: {}/{}", score.0, current, required);
}
