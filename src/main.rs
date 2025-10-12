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

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

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

#[derive(Component)]
pub struct BGM;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin { ..default() }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .insert_resource(DisplayQuality::High)
        .insert_resource(Volume(5))
        .insert_resource(Difficulty::default())
        .init_state::<MainState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::plugin, menu::plugin, game::plugin))
        .add_systems(Update, update_bgm_volume)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update_bgm_volume(mut music_controller: Query<&mut AudioSink, With<BGM>>, volume: Res<Volume>) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };
    let v = if volume.0 == 0 {
        0.0
    } else {
        0.05f32.powf(1.0 - (volume.0 as f32) / 9.0)
    };
    sink.set_volume(bevy::audio::Volume::Linear(v));
}
