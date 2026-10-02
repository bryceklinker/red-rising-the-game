use crate::character_select::{
    despawn_character_select_ui, handle_character_select_button, spawn_character_select_ui,
};
use crate::game_state::GameState;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, in_state};

pub fn character_select_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::CharacterSelect),
        spawn_character_select_ui,
    );
    app.add_systems(
        Update,
        handle_character_select_button.run_if(in_state(GameState::CharacterSelect)),
    );
    app.add_systems(
        OnExit(GameState::CharacterSelect),
        despawn_character_select_ui,
    );
}
