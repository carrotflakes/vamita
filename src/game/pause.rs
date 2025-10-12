use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::game::components::LevelEntity;
use crate::game::reset_game;
use crate::game::ui::PauseOverlay;
use crate::{Difficulty, MainState};

#[derive(Resource, Default)]
pub struct PauseState {
    pub paused: bool,
}

pub fn pause_input(
    kb: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    mut overlay: Query<&mut Visibility, With<PauseOverlay>>,
) {
    if kb.just_pressed(KeyCode::Escape) {
        pause_state.paused = !pause_state.paused;

        if let Some(mut visibility) = overlay.iter_mut().next() {
            *visibility = if pause_state.paused {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub fn pause_menu_actions(
    kb: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    mut overlay: Query<&mut Visibility, With<PauseOverlay>>,
    mut commands: Commands,
    level_entity_query: Query<Entity, With<LevelEntity>>,
    asset_server: Res<AssetServer>,
    difficulty: Res<Difficulty>,
    mut game_state: ResMut<NextState<MainState>>,
) {
    if !pause_state.paused {
        return;
    }

    if kb.just_pressed(KeyCode::Enter) || kb.just_pressed(KeyCode::KeyR) {
        pause_state.paused = false;
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
        pause_state.paused = false;
        if let Some(mut visibility) = overlay.iter_mut().next() {
            *visibility = Visibility::Hidden;
        }
    }

    if kb.just_pressed(KeyCode::KeyQ) {
        game_state.set(MainState::Menu);
    }
}
