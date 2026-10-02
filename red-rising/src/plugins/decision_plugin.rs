use crate::decision::ui::{
    ChosenEndState, despawn_decision_ui, handle_decision_button, spawn_decision_ui,
};
use crate::game_state::GameState;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, in_state};

pub fn decision_plugin(app: &mut App) {
    app.init_resource::<ChosenEndState>();
    app.add_systems(OnEnter(GameState::Decision), spawn_decision_ui);
    app.add_systems(
        Update,
        handle_decision_button.run_if(in_state(GameState::Decision)),
    );
    app.add_systems(OnExit(GameState::Decision), despawn_decision_ui);
}
