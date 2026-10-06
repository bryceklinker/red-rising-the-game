use crate::end::spawn_end_ui;
use crate::game_state::GameState;
use bevy::prelude::{App, OnEnter};

pub fn end_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::End), spawn_end_ui);
}
