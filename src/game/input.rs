use bevy::input::ButtonInput;
use bevy::prelude::*;

use super::components::{PauseOverlay, Player, Velocity};
use super::resources::PauseState;

pub fn player_input(
    mut query: Query<&mut Velocity, With<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
    pause_state: Res<PauseState>,
) {
    if pause_state.paused {
        return;
    }

    if let Ok(mut velocity) = query.single_mut() {
        let mut dir = Vec2::ZERO;
        if kb.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            dir.y += 1.0;
        }
        if kb.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            dir.y -= 1.0;
        }
        if kb.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            dir.x -= 1.0;
        }
        if kb.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            dir.x += 1.0;
        }
        let dir = dir.normalize_or_zero();
        velocity.0 = dir * super::constants::PLAYER_SPEED;
    }
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
