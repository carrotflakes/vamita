use bevy::input::ButtonInput;
use bevy::prelude::*;

use super::components::{Enemy, Player, Projectile, Velocity, ExperienceOrb};
use super::constants::PLAYER_MAX_HEALTH;
use super::resources::{EnemySpawnTimer, PauseState, PlayerStats, Score, ShootTimer};

use crate::MainState;
use crate::game::spawn_player;
use crate::game::ui::PauseOverlay;

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
    exp_orb_query: Query<Entity, With<ExperienceOrb>>,
    asset_server: Res<AssetServer>,
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
            &mut score,
            &mut player_stats,
            &mut enemy_spawn,
            &mut shoot_timer,
            &mut player_query,
            &enemy_query,
            &projectile_query,
            &exp_orb_query,
            &asset_server,
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

fn reset_game(
    commands: &mut Commands,
    score: &mut ResMut<Score>,
    player_stats: &mut ResMut<PlayerStats>,
    enemy_spawn: &mut ResMut<EnemySpawnTimer>,
    shoot_timer: &mut ResMut<ShootTimer>,
    player_query: &mut Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    enemy_query: &Query<Entity, With<Enemy>>,
    projectile_query: &Query<Entity, With<Projectile>>,
    exp_orb_query: &Query<Entity, With<ExperienceOrb>>,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    score.0 = 0;
    player_stats.health = PLAYER_MAX_HEALTH;
    player_stats.experience = 0;
    enemy_spawn.0.reset();
    shoot_timer.0.reset();

    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in exp_orb_query.iter() {
        commands.entity(entity).despawn();
    }

    if let Some((_entity, mut transform, mut velocity)) = player_query.iter_mut().next() {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
        velocity.0 = Vec2::ZERO;
    } else {
        spawn_player(commands, &font);
    }
}
