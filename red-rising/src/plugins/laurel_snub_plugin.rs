use crate::game_state::GameState;
use crate::laurel_snub::{
    despawn_laurel_snub_banner, handle_acknowledge_laurel_snub_button, spawn_laurel_snub_banner,
};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, in_state};

pub fn laurel_snub_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::LaurelSnub), spawn_laurel_snub_banner);
    app.add_systems(
        Update,
        handle_acknowledge_laurel_snub_button.run_if(in_state(GameState::LaurelSnub)),
    );
    app.add_systems(OnExit(GameState::LaurelSnub), despawn_laurel_snub_banner);
}
