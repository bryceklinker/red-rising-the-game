use bevy::MinimalPlugins;
use bevy::app::AppExit;
use bevy::prelude::{App, AppExtStates, Messages, NextState, Update};
use bevy::state::app::StatesPlugin;
use red_rising::capture::runner::advance_scripted_session;
use red_rising::capture::script::{ScriptStep, ScriptedSession};
use red_rising::game_state::GameState;

fn setup_testing_app(steps: Vec<ScriptStep>) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.insert_resource(ScriptedSession::new(steps));
    app.add_systems(Update, advance_scripted_session);
    app
}

#[test]
fn when_advance_ticks_runs_then_it_counts_down_once_per_update() {
    let mut app = setup_testing_app(vec![ScriptStep::AdvanceTicks(3), ScriptStep::Exit]);

    app.update();
    assert_eq!(
        app.world().resource::<ScriptedSession>().front(),
        Some(&ScriptStep::AdvanceTicks(2))
    );

    app.update();
    assert_eq!(
        app.world().resource::<ScriptedSession>().front(),
        Some(&ScriptStep::AdvanceTicks(1))
    );

    app.update();
    assert_eq!(
        app.world().resource::<ScriptedSession>().front(),
        Some(&ScriptStep::Exit)
    );
}

#[test]
fn when_waiting_for_a_different_state_then_the_step_does_not_advance() {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::Drilling)]);

    app.update();

    assert_eq!(
        app.world().resource::<ScriptedSession>().front(),
        Some(&ScriptStep::WaitForState(GameState::Drilling))
    );
}

#[test]
fn when_the_expected_state_is_reached_then_wait_for_state_advances() {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::Drilling)]);

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();
    app.update();

    assert_eq!(app.world().resource::<ScriptedSession>().front(), None);
}

#[test]
fn when_capture_step_runs_then_it_advances_to_the_next_step() {
    let mut app = setup_testing_app(vec![ScriptStep::Capture("drilling"), ScriptStep::Exit]);

    app.update();

    assert_eq!(
        app.world().resource::<ScriptedSession>().front(),
        Some(&ScriptStep::Exit)
    );
}

#[test]
fn when_exit_step_runs_then_an_app_exit_message_is_written() {
    let mut app = setup_testing_app(vec![ScriptStep::Exit]);

    app.update();

    assert_eq!(app.world().resource::<ScriptedSession>().front(), None);
    assert!(!app.world().resource::<Messages<AppExit>>().is_empty());
}
