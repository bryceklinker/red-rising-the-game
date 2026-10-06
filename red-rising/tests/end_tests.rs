use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, NextState, Text};
use bevy::state::app::StatesPlugin;
use red_rising::decision::outcome::EndState;
use red_rising::decision::ui::ChosenEndState;
use red_rising::end::EndUi;
use red_rising::game_state::GameState;
use red_rising::plugins::end_plugin::end_plugin;

fn setup_testing_app(chosen: ChosenEndState) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.insert_resource(chosen);
    app.add_plugins(end_plugin);
    app
}

#[test]
fn when_entering_end_then_ui_reflects_the_chosen_outcome() {
    let mut app = setup_testing_app(ChosenEndState(Some(EndState::WaitedForTheTeam)));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::End);
    app.update();
    app.update();

    let mut query = app
        .world_mut()
        .query::<(&EndUi, &bevy::prelude::Children)>();
    assert_eq!(query.iter(app.world()).count(), 1);

    let mut text_query = app.world_mut().query::<&Text>();
    let text = text_query.iter(app.world()).next().unwrap();
    assert_eq!(text.0, "Darrow waits for the team before going further.");
}

#[test]
fn when_entering_end_with_no_choice_recorded_then_ui_shows_a_fallback_message() {
    let mut app = setup_testing_app(ChosenEndState(None));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::End);
    app.update();
    app.update();

    let mut text_query = app.world_mut().query::<&Text>();
    let text = text_query.iter(app.world()).next().unwrap();
    assert_eq!(text.0, "The demo has ended.");
}
