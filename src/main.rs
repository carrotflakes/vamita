mod audio;
mod game;
mod menu;
mod splash;

use bevy::prelude::*;

#[derive(States, Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum MainState {
    #[default]
    Splash,
    Menu,
    Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

impl Difficulty {
    pub fn enemy_spawn_interval(self, base: f32) -> f32 {
        match self {
            Difficulty::Easy => base * 1.5,
            Difficulty::Normal => base,
            Difficulty::Hard => base * 0.75,
        }
    }

    pub fn player_max_health(self, base: i32) -> i32 {
        match self {
            Difficulty::Easy => base + 1,
            Difficulty::Normal => base,
            Difficulty::Hard => (base / 2).max(1),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin { ..default() }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .insert_resource(DisplayQuality::High)
        .insert_resource(Difficulty::default())
        .init_state::<MainState>()
        .add_systems(Startup, setup)
        .add_plugins((audio::plugin, splash::plugin, menu::plugin, game::plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
