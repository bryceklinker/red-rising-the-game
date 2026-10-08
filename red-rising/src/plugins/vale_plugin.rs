use crate::game_state::GameState;
use crate::player::{despawn_player, spawn_player, spawn_player_mesh};
use crate::vale::check_vale_window_reached;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, in_state};

pub fn vale_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Vale), spawn_player);
    app.add_systems(
        OnEnter(GameState::Vale),
        spawn_player_mesh.after(spawn_player),
    );
    app.add_systems(
        Update,
        check_vale_window_reached.run_if(in_state(GameState::Vale)),
    );
    app.add_systems(OnExit(GameState::Vale), despawn_player);
}
