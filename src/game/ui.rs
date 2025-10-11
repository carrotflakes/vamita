use bevy::prelude::*;

use crate::MainState;
use super::resources::Score;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct ScoreboardUi;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        DespawnOnExit(MainState::Game),
        ScoreboardUi,
        Text2d("Score: ".to_string()),
        TextFont {
            font: font_handle.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(-200.0, 200.0, 1.0)),
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

    const MARGIN: Val = Val::Px(16.0);

    commands.spawn((
        DespawnOnExit(MainState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..Default::default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        Visibility::Hidden,
        PauseOverlay,
        children!((
            Text::new("Paused\nEnter: Resume\nN: New Game\nQ: Back to Menu".to_string()),
            TextFont {
                font: font_handle.clone(),
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )),
    ));
}

pub fn update_score_text(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text2d>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.0.to_string();
}
