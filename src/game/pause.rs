use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::game::components::LevelEntity;
use crate::game::ui::PauseOverlay;
use crate::game::{GameState, reset_game};
use crate::{Difficulty, MainState};

pub fn pause_input(
    kb: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut overlay: Query<&mut Visibility, With<PauseOverlay>>,
) {
    if kb.just_pressed(KeyCode::Escape) {
        let next_state = if *game_state == GameState::Playing {
            GameState::Paused
        } else {
            GameState::Playing
        };
        set_game_state.set(next_state);

        if let Some(mut visibility) = overlay.iter_mut().next() {
            *visibility = if next_state == GameState::Paused {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub fn pause_menu_actions(
    kb: Res<ButtonInput<KeyCode>>,
    mut overlay: Query<&mut Visibility, With<PauseOverlay>>,
    mut commands: Commands,
    level_entity_query: Query<Entity, With<LevelEntity>>,
    asset_server: Res<AssetServer>,
    difficulty: Res<Difficulty>,
    mut main_state: ResMut<NextState<MainState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if kb.just_pressed(KeyCode::Enter) || kb.just_pressed(KeyCode::KeyR) {
        game_state.set(GameState::Playing);
        if let Some(mut visibility) = overlay.iter_mut().next() {
            *visibility = Visibility::Hidden;
        }
        return;
    }

    if kb.just_pressed(KeyCode::KeyN) {
        reset_game(
            &mut commands,
            Some(&level_entity_query),
            &asset_server,
            *difficulty,
        );
        game_state.set(GameState::Playing);
        if let Some(mut visibility) = overlay.iter_mut().next() {
            *visibility = Visibility::Hidden;
        }
    }

    if kb.just_pressed(KeyCode::KeyQ) {
        main_state.set(MainState::Menu);
    }
}
