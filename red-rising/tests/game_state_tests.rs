use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, NextState, State};
use bevy::state::app::StatesPlugin;
use red_rising::game_state::GameState;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app
}

#[test]
fn default_state_is_character_select() {
    let app = setup_testing_app();

    let state = app.world().resource::<State<GameState>>();

    assert_eq!(*state.get(), GameState::CharacterSelect);
}

#[test]
fn when_next_state_is_set_then_state_transitions_after_update() {
    let mut app = setup_testing_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Drilling);
}
