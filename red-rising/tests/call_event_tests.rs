use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, NextState};
use bevy::state::app::StatesPlugin;
use red_rising::call_event::CallEventBanner;
use red_rising::game_state::GameState;
use red_rising::plugins::call_event_plugin::call_event_plugin;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.add_plugins(call_event_plugin);
    app
}

#[test]
fn when_entering_call_event_then_banner_is_spawned() {
    let mut app = setup_testing_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::CallEvent);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&CallEventBanner>();
    assert_eq!(query.iter(app.world()).count(), 1);
}

#[test]
fn when_leaving_call_event_then_banner_is_despawned() {
    let mut app = setup_testing_app();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::CallEvent);
    app.update();
    app.update();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Decision);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&CallEventBanner>();
    assert_eq!(query.iter(app.world()).count(), 0);
}
