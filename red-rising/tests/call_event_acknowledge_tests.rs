use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, Entity, Interaction, NextState, State, With};
use bevy::state::app::StatesPlugin;
use red_rising::call_event::AcknowledgeCallButton;
use red_rising::game_state::GameState;
use red_rising::plugins::call_event_plugin::call_event_plugin;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.add_plugins(call_event_plugin);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::CallEvent);
    app.update();
    app.update();
    app
}

#[test]
fn when_acknowledge_button_is_pressed_then_state_transitions_to_decision() {
    let mut app = setup_testing_app();
    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<AcknowledgeCallButton>>()
        .single(app.world())
        .unwrap();

    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Decision);
}
