use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, Entity, Interaction, State, With};
use bevy::state::app::StatesPlugin;
use red_rising::character_select::CharacterSelectButton;
use red_rising::game_state::GameState;
use red_rising::plugins::character_select_plugin::character_select_plugin;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.add_plugins(character_select_plugin);
    app.update();
    app
}

fn button_entity(app: &mut App) -> Entity {
    app.world_mut()
        .query_filtered::<Entity, With<CharacterSelectButton>>()
        .single(app.world())
        .unwrap()
}

#[test]
fn when_entering_character_select_then_darrow_button_is_spawned() {
    let mut app = setup_testing_app();

    let mut query = app.world_mut().query::<&CharacterSelectButton>();
    assert_eq!(query.iter(app.world()).count(), 1);
}

#[test]
fn when_darrow_button_is_pressed_then_state_transitions_to_drilling() {
    let mut app = setup_testing_app();
    let entity = button_entity(&mut app);

    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Drilling);
}

#[test]
fn when_leaving_character_select_then_button_is_despawned() {
    let mut app = setup_testing_app();
    let entity = button_entity(&mut app);

    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&CharacterSelectButton>();
    assert_eq!(query.iter(app.world()).count(), 0);
}
