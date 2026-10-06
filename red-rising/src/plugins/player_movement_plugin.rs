use crate::game_state::GameState;
use crate::player::{despawn_player, move_player, spawn_player};
use bevy::prelude::{App, OnExit, Startup, Update};

pub fn player_movement_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, move_player);
    app.add_systems(OnExit(GameState::CharacterSelect), despawn_player);
}
