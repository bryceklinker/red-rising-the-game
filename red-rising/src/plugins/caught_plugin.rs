use crate::caught::spawn_caught_banner;
use crate::game_state::GameState;
use bevy::prelude::{App, OnEnter};

pub fn caught_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Caught), spawn_caught_banner);
}
