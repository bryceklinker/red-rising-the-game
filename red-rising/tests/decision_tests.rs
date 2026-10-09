use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, Entity, Interaction, NextState, State};
use bevy::state::app::StatesPlugin;
use red_rising::decision::outcome::{DecisionOption, DrillingOutcome, NarolReaction};
use red_rising::decision::ui::{ChosenDrillingOutcome, DecisionButton, DecisionUi};
use red_rising::game_state::GameState;
use red_rising::plugins::decision_plugin::decision_plugin;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.add_plugins(decision_plugin);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Decision);
    app.update();
    app.update();
    app
}

fn button_entity_for(app: &mut App, option: DecisionOption) -> Entity {
    let mut query = app.world_mut().query::<(Entity, &DecisionButton)>();
    query
        .iter(app.world())
        .find(|(_, button)| button.0 == option)
        .map(|(entity, _)| entity)
        .unwrap()
}

#[test]
fn when_entering_decision_then_three_buttons_are_spawned() {
    let mut app = setup_testing_app();

    let mut query = app.world_mut().query::<&DecisionButton>();
    assert_eq!(query.iter(app.world()).count(), 3);
}

#[test]
fn when_get_out_and_check_is_pressed_then_state_transitions_to_laurel_snub_with_that_outcome() {
    let mut app = setup_testing_app();
    let entity = button_entity_for(&mut app, DecisionOption::GetOutAndCheck);

    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::LaurelSnub);
    let chosen = app.world().resource::<ChosenDrillingOutcome>();
    assert_eq!(
        chosen.0,
        Some(DrillingOutcome {
            yield_kilos: 32,
            injured: false,
            narol_reaction: NarolReaction::Relieved,
        })
    );
}

#[test]
fn when_leaving_decision_then_buttons_are_despawned() {
    let mut app = setup_testing_app();
    let entity = button_entity_for(&mut app, DecisionOption::KeepDrilling);

    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&DecisionUi>();
    assert_eq!(query.iter(app.world()).count(), 0);
}
