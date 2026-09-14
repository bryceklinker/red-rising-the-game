use bevy::prelude::{App, Startup, Update};
use crate::player::{move_player, spawn_player};

pub fn player_movement_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, move_player);
}
