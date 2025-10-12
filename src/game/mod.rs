mod combat;
mod components;
mod constants;
mod enemy;
mod events;
mod experience;
mod movement;
mod pause;
mod player;
mod resources;
mod ui;

use bevy::prelude::*;
use combat::handle_collisions;
use constants::{BOMB_INTERVAL, ENEMY_SPAWN_INTERVAL, FIRE_RATE, PLAYER_MAX_HEALTH};
use events::{EnemyKilled, PlayerHit};
use experience::experience_orb_behavior;
use movement::{decay_lifetimes, enemy_seek_player, update_projectiles, update_velocity};
use pause::{PauseState, pause_menu_actions};
use player::player_input;
use resources::{BombSound, ExperienceOrbSound, HitSelfSound, HitSound, ShootSound};

use crate::{
    BGM, Difficulty, MainState,
    game::{
        combat::EnemySpawnTimer,
        components::LevelEntity,
        enemy::EnemyCatalog,
        player::{BombTimer, PlayerStats, ShootTimer, spawn_player},
        ui::Score,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MainState::Game), (setup, ui::setup))
        .insert_resource(EnemyCatalog::new())
        .add_message::<PlayerHit>()
        .add_message::<EnemyKilled>()
        .add_systems(
            Update,
            (
                pause::pause_input,
                player_input,
                pause_menu_actions,
                ui::update_score_text,
                player::update_health_text,
            )
                .chain()
                .run_if(in_state(MainState::Game)),
        )
        .add_systems(
            FixedUpdate,
            (
                experience_orb_behavior,
                update_velocity,
                enemy_seek_player,
                update_projectiles,
                player::constrain_to_arena,
                player::player_place_bomb,
                player::update_bombs,
                handle_collisions,
                enemy::spawn_enemies,
                player::player_auto_fire,
                decay_lifetimes,
            )
                .chain()
                .run_if(in_state(MainState::Game)),
        );
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, difficulty: Res<Difficulty>) {
    let hit_sound_handle = asset_server.load("sounds/hit.wav");
    let hit_self_sound_handle = asset_server.load("sounds/hit_self.wav");
    let shoot_sound_handle = asset_server.load("sounds/shoot.wav");
    let exp_sound_handle = asset_server.load("sounds/exp.wav");
    let bomb_sound_handle = asset_server.load("sounds/bomb.wav");

    commands.insert_resource(HitSound(hit_sound_handle));
    commands.insert_resource(HitSelfSound(hit_self_sound_handle));
    commands.insert_resource(ShootSound(shoot_sound_handle));
    commands.insert_resource(ExperienceOrbSound(exp_sound_handle));
    commands.insert_resource(BombSound(bomb_sound_handle));

    commands.spawn((
        DespawnOnExit(MainState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            // center children
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnGameScreen,
    ));

    let bgm_handle: Handle<AudioSource> = asset_server.load("sounds/vamita-0.mp3");
    commands.spawn((
        DespawnOnExit(MainState::Game),
        AudioPlayer(bgm_handle),
        BGM,
        PlaybackSettings::LOOP,
    ));

    reset_game(&mut commands, None, &asset_server, *difficulty);
}

pub fn reset_game(
    commands: &mut Commands,
    level_entity_query: Option<&Query<Entity, With<LevelEntity>>>,
    asset_server: &Res<AssetServer>,
    difficulty: Difficulty,
) {
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        difficulty.enemy_spawn_interval(ENEMY_SPAWN_INTERVAL),
        TimerMode::Repeating,
    )));
    commands.insert_resource(ShootTimer(Timer::from_seconds(
        FIRE_RATE,
        TimerMode::Repeating,
    )));
    commands.insert_resource(BombTimer(Timer::from_seconds(
        BOMB_INTERVAL,
        TimerMode::Repeating,
    )));
    commands.insert_resource(PlayerStats {
        health: difficulty.player_max_health(PLAYER_MAX_HEALTH),
        experience: 0,
    });
    commands.insert_resource(Score::default());
    commands.insert_resource(PauseState::default());

    if let Some(level_entity_query) = level_entity_query {
        for entity in level_entity_query.iter() {
            commands.entity(entity).despawn();
        }
    }

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    spawn_player(commands, &font);
}
