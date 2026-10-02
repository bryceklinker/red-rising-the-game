use crate::call_event::{despawn_call_event_banner, spawn_call_event_banner};
use crate::game_state::GameState;
use bevy::prelude::{App, OnEnter, OnExit};

pub fn call_event_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::CallEvent), spawn_call_event_banner);
    app.add_systems(OnExit(GameState::CallEvent), despawn_call_event_banner);
}
