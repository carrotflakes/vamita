use bevy::prelude::*;
use bevy_pkv::PkvStore;

pub fn plugin(app: &mut App) {
    app.insert_resource(PkvStore::new("carrotflakes", "vamita"));
}
