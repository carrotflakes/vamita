pub mod combat;
pub mod input;
pub mod movement;
pub mod setup;
pub mod state;
pub mod ui;

pub use combat::{handle_collisions, player_auto_fire, spawn_enemies};
pub use input::{pause_input, player_input};
pub use movement::{
    constrain_to_arena, decay_lifetimes, enemy_seek_player, update_projectiles, update_velocity,
};
pub use setup::setup;
pub use state::pause_menu_actions;
