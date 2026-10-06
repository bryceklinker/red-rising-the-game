use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, NextState, State, Transform, Update};
use bevy::state::app::StatesPlugin;
use red_rising::drilling::depth::check_drill_depth;
use red_rising::drilling::rig::DrillRig;
use red_rising::game_state::GameState;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.add_systems(Update, check_drill_depth);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();
    app
}

#[test]
fn when_rig_is_shallow_then_state_stays_drilling() {
    let mut app = setup_testing_app();
    app.world_mut()
        .spawn((DrillRig, Transform::from_xyz(0.0, -2.0, 0.0)));

    app.update();
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Drilling);
}

#[test]
fn when_rig_reaches_the_depth_threshold_then_state_transitions_to_call_event() {
    let mut app = setup_testing_app();
    app.world_mut()
        .spawn((DrillRig, Transform::from_xyz(0.0, -15.0, 0.0)));

    app.update();
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::CallEvent);
}
