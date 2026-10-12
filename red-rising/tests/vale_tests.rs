use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, ButtonInput, KeyCode, NextState, Startup, State, Update};
use bevy::state::app::StatesPlugin;
use red_rising::game_state::GameState;
use red_rising::player::{move_player, spawn_player};
use red_rising::vale::check_vale_window_reached;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, (move_player, check_vale_window_reached));
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Vale);
    app.update();
    app
}

fn walk_forward(app: &mut App) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyW);
}

#[test]
fn when_the_player_has_not_reached_the_window_then_state_stays_vale() {
    let mut app = setup_testing_app();

    for _ in 0..3 {
        walk_forward(&mut app);
    }

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Vale);
}

#[test]
fn when_the_player_walks_far_enough_forward_then_state_transitions_to_vale_reaction() {
    let mut app = setup_testing_app();

    for _ in 0..15 {
        walk_forward(&mut app);
    }

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::ValeReaction);
}
