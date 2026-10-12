use crate::game_state::GameState;
use crate::vale::reaction::{
    ChosenValeReaction, despawn_vale_reaction_ui, handle_vale_reaction_button,
    spawn_vale_reaction_ui,
};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, in_state};

pub fn vale_reaction_plugin(app: &mut App) {
    app.init_resource::<ChosenValeReaction>();
    app.add_systems(OnEnter(GameState::ValeReaction), spawn_vale_reaction_ui);
    app.add_systems(
        Update,
        handle_vale_reaction_button.run_if(in_state(GameState::ValeReaction)),
    );
    app.add_systems(OnExit(GameState::ValeReaction), despawn_vale_reaction_ui);
}
