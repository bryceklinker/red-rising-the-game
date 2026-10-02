use crate::audio::{despawn_drill_rumble, spawn_call_beep, spawn_drill_rumble};
use crate::game_state::GameState;
use bevy::prelude::{App, OnEnter, OnExit};

pub fn audio_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Drilling), spawn_drill_rumble);
    app.add_systems(OnExit(GameState::Drilling), despawn_drill_rumble);
    app.add_systems(OnEnter(GameState::CallEvent), spawn_call_beep);
}
