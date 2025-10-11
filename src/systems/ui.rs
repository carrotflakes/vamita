use bevy::prelude::*;

use crate::components::{HealthText, PauseOverlay, ScoreText};
use crate::resources::{PlayerStats, Score};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Text2d("Score: 0".to_string()),
        TextFont {
            font: font_handle.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(-200.0, 200.0, 1.0)),
        ScoreText,
    ));

    const MARGIN: Val = Val::Px(16.0);

    commands.spawn((
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
            Text::new("Paused\nEnter: Resume\nN: New Game".to_string()),
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
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text2d, With<ScoreText>>,
) {
    let label = if player_stats.health > 0 {
        format!("Score: {}", score.0)
    } else {
        format!("GAME OVER — Score: {}", score.0)
    };
    for mut text in &mut query {
        text.0 = label.clone();
    }
}

pub fn update_health_text(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text2d, With<HealthText>>,
) {
    for mut text in &mut query {
        text.0 = format!("HP: {}", player_stats.health.max(0));
    }
}
