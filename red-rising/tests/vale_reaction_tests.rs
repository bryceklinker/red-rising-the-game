use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, Entity, Interaction, NextState, State};
use bevy::state::app::StatesPlugin;
use red_rising::game_state::GameState;
use red_rising::plugins::vale_reaction_plugin::vale_reaction_plugin;
use red_rising::vale::reaction::{
    ChosenValeReaction, ValeReactionButton, ValeReactionChoice, ValeReactionUi,
};

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.add_plugins(vale_reaction_plugin);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::ValeReaction);
    app.update();
    app.update();
    app
}

fn button_entity_for(app: &mut App, choice: ValeReactionChoice) -> Entity {
    let mut query = app.world_mut().query::<(Entity, &ValeReactionButton)>();
    query
        .iter(app.world())
        .find(|(_, button)| button.0 == choice)
        .map(|(entity, _)| entity)
        .unwrap()
}

#[test]
fn when_entering_vale_reaction_then_three_buttons_are_spawned() {
    let mut app = setup_testing_app();

    let mut query = app.world_mut().query::<&ValeReactionButton>();
    assert_eq!(query.iter(app.world()).count(), 3);
}

#[test]
fn when_a_reaction_is_pressed_then_it_is_recorded_and_state_transitions_to_caught() {
    for choice in [
        ValeReactionChoice::Awe,
        ValeReactionChoice::Anger,
        ValeReactionChoice::Grief,
    ] {
        let mut app = setup_testing_app();
        let entity = button_entity_for(&mut app, choice);

        app.world_mut()
            .entity_mut(entity)
            .insert(Interaction::Pressed);
        app.update();
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Caught);
        let chosen = app.world().resource::<ChosenValeReaction>();
        assert_eq!(chosen.0, Some(choice));
    }
}

#[test]
fn when_leaving_vale_reaction_then_buttons_are_despawned() {
    let mut app = setup_testing_app();
    let entity = button_entity_for(&mut app, ValeReactionChoice::Awe);

    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&ValeReactionUi>();
    assert_eq!(query.iter(app.world()).count(), 0);
}
