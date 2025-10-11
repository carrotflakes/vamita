mod components;
mod constants;
mod events;
mod resources;
mod systems;

use bevy::prelude::*;

use crate::constants::{ENEMY_SPAWN_INTERVAL, FIRE_RATE, PLAYER_MAX_HEALTH};
use crate::events::{EnemyKilled, PlayerHit};
use crate::resources::{
    EnemyCatalog, EnemySpawnTimer, PauseState, PlayerStats, Score, ShootTimer,
};
use crate::systems::{
    constrain_to_arena, decay_lifetimes, enemy_seek_player, handle_collisions, pause_input, pause_menu_actions, player_auto_fire, player_input, setup, spawn_enemies, ui, update_projectiles, update_velocity
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin { ..default() }))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            ENEMY_SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_resource(ShootTimer(Timer::from_seconds(
            FIRE_RATE,
            TimerMode::Repeating,
        )))
        .insert_resource(PlayerStats {
            health: PLAYER_MAX_HEALTH,
        })
        .insert_resource(Score::default())
        .insert_resource(EnemyCatalog::new())
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .insert_resource(PauseState::default())
        .add_message::<PlayerHit>()
        .add_message::<EnemyKilled>()
        .add_systems(Startup, (setup, ui::setup))
        .add_systems(
            Update,
            (
                pause_input,
                pause_menu_actions,
                player_input,
                update_velocity,
                enemy_seek_player,
                update_projectiles,
                constrain_to_arena,
                handle_collisions,
                spawn_enemies,
                player_auto_fire,
                ui::update_score_text,
                ui::update_health_text,
            )
                .chain(),
        )
        .add_systems(FixedUpdate, decay_lifetimes)
        .run();
}
