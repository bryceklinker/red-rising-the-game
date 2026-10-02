use crate::call_event::{
    despawn_call_event_banner, handle_acknowledge_call_button, spawn_call_event_banner,
};
use crate::game_state::GameState;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, in_state};

pub fn call_event_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::CallEvent), spawn_call_event_banner);
    app.add_systems(
        Update,
        handle_acknowledge_call_button.run_if(in_state(GameState::CallEvent)),
    );
    app.add_systems(OnExit(GameState::CallEvent), despawn_call_event_banner);
}
