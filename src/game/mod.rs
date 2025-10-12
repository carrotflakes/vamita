mod combat;
mod components;
mod constants;
mod events;
mod experience;
mod input;
mod movement;
mod resources;
mod state;
mod ui;

use bevy::prelude::*;
use combat::{handle_collisions, player_auto_fire, spawn_enemies};
use components::{Player, Velocity};
use constants::{ENEMY_SPAWN_INTERVAL, FIRE_RATE, PLAYER_MAX_HEALTH, PLAYER_SIZE};
use events::{EnemyKilled, PlayerHit};
use experience::experience_orb_behavior;
use input::{pause_input, player_input};
use movement::{
    constrain_to_arena, decay_lifetimes, enemy_seek_player, update_projectiles, update_velocity,
};
use resources::{
    EnemyCatalog, EnemySpawnTimer, ExperienceOrbSound, HitSelfSound, HitSound, PauseState,
    PlayerStats, Score, ShootSound, ShootTimer,
};
use state::pause_menu_actions;

use crate::{BGM, Difficulty, MainState, game::ui::HealthText};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MainState::Game), (setup, ui::setup))
        .insert_resource(EnemyCatalog::new())
        .add_message::<PlayerHit>()
        .add_message::<EnemyKilled>()
        .add_systems(
            Update,
            (
                pause_input,
                player_input,
                pause_menu_actions,
                ui::update_score_text,
                update_health_text,
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
                constrain_to_arena,
                handle_collisions,
                spawn_enemies,
                player_auto_fire,
                decay_lifetimes,
            )
                .chain()
                .run_if(in_state(MainState::Game)),
        );
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, difficulty: Res<Difficulty>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    let hit_sound_handle = asset_server.load("sounds/hit.wav");
    let hit_self_sound_handle = asset_server.load("sounds/hit_self.wav");
    let shoot_sound_handle = asset_server.load("sounds/shoot.wav");
    let exp_sound_handle = asset_server.load("sounds/exp.wav");

    commands.insert_resource(HitSound(hit_sound_handle));
    commands.insert_resource(HitSelfSound(hit_self_sound_handle));
    commands.insert_resource(ShootSound(shoot_sound_handle));
    commands.insert_resource(ExperienceOrbSound(exp_sound_handle));

    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        difficulty.enemy_spawn_interval(ENEMY_SPAWN_INTERVAL),
        TimerMode::Repeating,
    )));
    commands.insert_resource(ShootTimer(Timer::from_seconds(
        FIRE_RATE,
        TimerMode::Repeating,
    )));
    commands.insert_resource(PlayerStats {
        health: difficulty.player_max_health(PLAYER_MAX_HEALTH),
        experience: 0,
    });
    commands.insert_resource(Score::default());
    commands.insert_resource(PauseState::default());

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

    spawn_player(&mut commands, &font_handle);
}

fn spawn_player(commands: &mut Commands, handle_font: &Handle<Font>) {
    commands.spawn((
        DespawnOnExit(MainState::Game),
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), PLAYER_SIZE),
        Transform::default(),
        Player,
        Velocity(Vec2::ZERO),
        children!((
            Text2d("HP: ".to_string()),
            TextFont {
                font: handle_font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 28.0, 1.0)),
            HealthText,
            children!(TextSpan::default())
        )),
    ));
}

fn update_health_text(
    player_stats: Res<PlayerStats>,
    health_root: Single<Entity, (With<HealthText>, With<Text2d>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*health_root, 1) = player_stats.health.to_string();
}
