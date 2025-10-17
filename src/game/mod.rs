mod combat;
mod components;
mod constants;
mod enemy;
mod events;
mod experience;
mod movement;
mod pause;
mod player;
mod powerup;
mod resources;
mod ui;

use bevy::prelude::*;
use combat::handle_collisions;
use constants::ENEMY_SPAWN_INTERVAL;
use events::{EnemyKilled, PlayerHit};
use experience::experience_orb_behavior;
use movement::{decay_lifetimes, enemy_seek_player, update_projectiles, update_velocity};
use pause::{pause_button_visuals, pause_menu_actions};
use player::player_input;
use powerup::{PlayerUpgrades, PowerUpProgress, handle_powerup_selection, powerup_button_visuals};
use resources::{BombSound, DefeatSound, ExperienceOrbSound, HitSelfSound, HitSound, ShootSound};

use crate::{
    Difficulty, MainState,
    audio::BGM,
    game::{
        combat::EnemySpawnTimer,
        components::LevelEntity,
        enemy::EnemyCatalog,
        player::{BombTimer, PlayerStats, ShootTimer, spawn_player},
        ui::Score,
    },
};

#[derive(States, Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    SelectingPowerUp,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_systems(
            OnEnter(MainState::Game),
            (setup, ui::setup, experience::setup),
        )
        .insert_resource(EnemyCatalog::new())
        .add_message::<PlayerHit>()
        .add_message::<EnemyKilled>()
        .add_systems(
            Update,
            (
                pause::pause_input,
                player_input,
                ui::update_score_text,
                player::update_health_bar,
            )
                .chain()
                .run_if(in_state(MainState::Game).and(in_state(GameState::Playing))),
        )
        .add_systems(
            Update,
            (pause::pause_input, pause_button_visuals, pause_menu_actions)
                .chain()
                .run_if(in_state(MainState::Game).and(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (powerup_button_visuals, handle_powerup_selection)
                .run_if(in_state(MainState::Game).and(in_state(GameState::SelectingPowerUp))),
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
                enemy::update_enemy_hit_flash,
                enemy::spawn_enemies,
                player::player_auto_fire,
                decay_lifetimes,
                powerup::spawn_menu_when_ready,
            )
                .chain()
                .run_if(in_state(MainState::Game).and(in_state(GameState::Playing))),
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
    let defeat_sound_handle = asset_server.load("sounds/defeat.wav");

    commands.insert_resource(HitSound(hit_sound_handle));
    commands.insert_resource(HitSelfSound(hit_self_sound_handle));
    commands.insert_resource(ShootSound(shoot_sound_handle));
    commands.insert_resource(ExperienceOrbSound(exp_sound_handle));
    commands.insert_resource(BombSound(bomb_sound_handle));
    commands.insert_resource(DefeatSound(defeat_sound_handle));

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

    reset_game(&mut commands, None, *difficulty);
}

pub fn reset_game(
    commands: &mut Commands,
    level_entity_query: Option<&Query<Entity, With<LevelEntity>>>,
    difficulty: Difficulty,
) {
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        difficulty.enemy_spawn_interval(ENEMY_SPAWN_INTERVAL),
        TimerMode::Repeating,
    )));
    let upgrades = PlayerUpgrades::default();
    let fire_rate = upgrades.fire_rate_duration();
    let bomb_interval = upgrades.bomb_interval_duration();
    commands.insert_resource(upgrades);
    commands.insert_resource(ShootTimer(Timer::from_seconds(
        fire_rate,
        TimerMode::Repeating,
    )));
    commands.insert_resource(BombTimer(Timer::from_seconds(
        bomb_interval,
        TimerMode::Repeating,
    )));
    commands.insert_resource(PowerUpProgress::default());
    commands.insert_resource(PlayerStats { experience: 0 });
    commands.insert_resource(Score::default());

    if let Some(level_entity_query) = level_entity_query {
        for entity in level_entity_query.iter() {
            commands.entity(entity).despawn();
        }
    }

    spawn_player(commands, &difficulty);

    commands.set_state(GameState::Playing);
}
