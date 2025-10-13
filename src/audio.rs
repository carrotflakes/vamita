use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct BGMVolume(pub u32);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct SEVolume(pub u32);

#[derive(Component)]
pub struct BGM;

pub fn plugin(app: &mut App) {
    app.insert_resource(BGMVolume(5))
        .insert_resource(SEVolume(9))
        .add_systems(Update, update_bgm_volume);
}

pub fn spawn_se(commands: &mut Commands, se_volume: &SEVolume, sound: &Handle<AudioSource>) {
    commands.spawn((
        AudioPlayer(sound.clone()),
        PlaybackSettings::DESPAWN.with_volume(volume_from_setting(se_volume.0)),
    ));
}

pub fn update_bgm_volume(
    mut music_controller: Query<&mut AudioSink, With<BGM>>,
    volume: Res<BGMVolume>,
) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };
    sink.set_volume(volume_from_setting(volume.0));
}

pub fn volume_from_setting(level: u32) -> bevy::audio::Volume {
    let v = if level == 0 {
        0.0
    } else {
        0.05f32.powf(1.0 - (level as f32) / 9.0)
    };
    bevy::audio::Volume::Linear(v)
}
