use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::components::{Enemy, PauseOverlay, Player, Projectile, Velocity};
use crate::constants::PLAYER_MAX_HEALTH;
use crate::resources::{
    EnemySpawnTimer, PauseState, PlayerStats, Score, ShootTimer, UiAssets,
};

use super::setup::spawn_player;

pub fn pause_menu_actions(
    kb: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    mut overlay: Query<&mut Visibility, With<PauseOverlay>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_stats: ResMut<PlayerStats>,
    mut enemy_spawn: ResMut<EnemySpawnTimer>,
    mut shoot_timer: ResMut<ShootTimer>,
    mut player_query: Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    assets: Res<UiAssets>,
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
            &mut score,
            &mut player_stats,
            &mut enemy_spawn,
            &mut shoot_timer,
            &mut player_query,
            &enemy_query,
            &projectile_query,
            &assets,
        );
        pause_state.paused = false;
        if let Some(mut visibility) = overlay.iter_mut().next() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn reset_game(
    commands: &mut Commands,
    score: &mut ResMut<Score>,
    player_stats: &mut ResMut<PlayerStats>,
    enemy_spawn: &mut ResMut<EnemySpawnTimer>,
    shoot_timer: &mut ResMut<ShootTimer>,
    player_query: &mut Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    enemy_query: &Query<Entity, With<Enemy>>,
    projectile_query: &Query<Entity, With<Projectile>>,
    assets: &UiAssets,
) {
    score.0 = 0;
    player_stats.health = PLAYER_MAX_HEALTH;
    enemy_spawn.0.reset();
    shoot_timer.0.reset();

    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }

    if let Some((_entity, mut transform, mut velocity)) = player_query.iter_mut().next() {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
        velocity.0 = Vec2::ZERO;
    } else {
        spawn_player(commands, &assets.font);
    }
}
