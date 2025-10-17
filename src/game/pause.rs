use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::game::components::LevelEntity;
use crate::game::ui::PauseOverlay;
use crate::game::{GameState, reset_game};
use crate::{Difficulty, MainState};

const OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);
const PANEL_COLOR: Color = Color::srgba(0.08, 0.08, 0.12, 0.95);
const BUTTON_COLOR: Color = Color::srgba(0.18, 0.18, 0.28, 0.95);
const BUTTON_HOVER_COLOR: Color = Color::srgba(0.28, 0.28, 0.38, 0.95);
const BUTTON_PRESSED_COLOR: Color = Color::srgba(0.35, 0.65, 0.35, 1.0);

#[derive(Component)]
pub struct PauseButton {
    action: PauseAction,
}

#[derive(Clone, Copy)]
enum PauseAction {
    Resume,
    Restart,
    QuitToMenu,
}

pub fn pause_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kb: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut set_game_state: ResMut<NextState<GameState>>,
    overlay: Query<Entity, With<PauseOverlay>>,
) {
    if kb.just_pressed(KeyCode::Escape) {
        let next_state = if *game_state == GameState::Playing {
            GameState::Paused
        } else {
            GameState::Playing
        };
        set_game_state.set(next_state);

        if next_state == GameState::Paused {
            spawn_pause(commands, asset_server);
        } else {
            close_pause_overlay(&mut commands, &overlay);
        }
    }
}

pub fn spawn_pause(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            DespawnOnExit(MainState::Game),
            PauseOverlay,
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(OVERLAY_COLOR),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(440.0),
                    row_gap: Val::Px(20.0),
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(28.0)),
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                BackgroundColor(PANEL_COLOR),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("Paused".to_string()),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 44.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                panel
                    .spawn((Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|list| {
                        list.spawn(pause_button_bundle(
                            font_handle.clone(),
                            "Resume",
                            "Enter / Esc",
                            PauseAction::Resume,
                        ));
                        list.spawn(pause_button_bundle(
                            font_handle.clone(),
                            "New Game",
                            "N",
                            PauseAction::Restart,
                        ));
                        list.spawn(pause_button_bundle(
                            font_handle.clone(),
                            "Back to Menu",
                            "Q",
                            PauseAction::QuitToMenu,
                        ));
                    });
            });
        });
}

pub fn pause_button_visuals(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<PauseButton>),
    >,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Pressed => BUTTON_PRESSED_COLOR.into(),
            Interaction::Hovered => BUTTON_HOVER_COLOR.into(),
            Interaction::None => BUTTON_COLOR.into(),
        };
    }
}

pub fn pause_menu_actions(
    kb: Res<ButtonInput<KeyCode>>,
    mut button_interactions: Query<
        (&Interaction, &PauseButton),
        (Changed<Interaction>, With<Button>),
    >,
    overlay: Query<Entity, With<PauseOverlay>>,
    mut commands: Commands,
    level_entity_query: Query<Entity, With<LevelEntity>>,
    difficulty: Res<Difficulty>,
    mut main_state: ResMut<NextState<MainState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut resume_requested = kb.just_pressed(KeyCode::Enter);
    let mut restart_requested = kb.just_pressed(KeyCode::KeyN);
    let mut menu_requested = kb.just_pressed(KeyCode::KeyQ);

    for (interaction, button) in &mut button_interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button.action {
            PauseAction::Resume => resume_requested = true,
            PauseAction::Restart => restart_requested = true,
            PauseAction::QuitToMenu => menu_requested = true,
        }
    }

    if resume_requested {
        game_state.set(GameState::Playing);
        close_pause_overlay(&mut commands, &overlay);
        return;
    }

    if restart_requested {
        reset_game(&mut commands, Some(&level_entity_query), *difficulty);
        game_state.set(GameState::Playing);
        close_pause_overlay(&mut commands, &overlay);
        return;
    }

    if menu_requested {
        main_state.set(MainState::Menu);
        close_pause_overlay(&mut commands, &overlay);
    }
}

fn pause_button_bundle(
    font: Handle<Font>,
    label: &str,
    hint: &str,
    action: PauseAction,
) -> impl Bundle {
    (
        Button,
        PauseButton { action },
        Node {
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(18.0), Val::Px(14.0)),
            row_gap: Val::Px(4.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        BackgroundColor(BUTTON_COLOR),
        children![
            (
                Text::new(label.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::new(hint.to_string()),
                TextFont {
                    font,
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.8, 0.85, 1.0)),
            ),
        ],
    )
}

fn close_pause_overlay(commands: &mut Commands, overlay: &Query<Entity, With<PauseOverlay>>) {
    for entity in overlay.iter() {
        commands.entity(entity).despawn();
    }
}
