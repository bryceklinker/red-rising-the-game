use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, Interaction, Query, State, Update};
use bevy::state::app::StatesPlugin;
use red_rising::capture::auto_advance::auto_advance_game_state;
use red_rising::capture::script::{ScriptStep, ScriptedSession};
use red_rising::character_select::CharacterSelectButton;
use red_rising::decision::ui::ChosenDrillingOutcome;
use red_rising::game_state::GameState;
use red_rising::vale::reaction::ChosenValeReaction;

fn setup_testing_app(steps: Vec<ScriptStep>) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.init_resource::<ChosenDrillingOutcome>();
    app.init_resource::<ChosenValeReaction>();
    app.insert_resource(ScriptedSession::new(steps));
    app.add_systems(Update, auto_advance_game_state);
    app
}

fn reset_interaction_to_none(mut interactions: Query<&mut Interaction>) {
    for mut interaction in &mut interactions {
        *interaction = Interaction::None;
    }
}

#[test]
fn when_script_moves_past_character_select_then_game_state_advances_to_drilling_within_bounded_ticks()
 {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::Drilling)]);

    for _ in 0..5 {
        app.update();
    }

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Drilling
    );
}

#[test]
fn when_a_system_clobbers_interaction_every_frame_the_transition_still_completes() {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::Drilling)]);
    app.world_mut()
        .spawn((CharacterSelectButton, Interaction::None));
    app.add_systems(Update, reset_interaction_to_none);

    for _ in 0..5 {
        app.update();
    }

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Drilling
    );
}

#[test]
fn when_script_has_not_moved_past_current_state_then_game_state_does_not_advance() {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::CharacterSelect)]);

    app.update();

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::CharacterSelect
    );
}

#[test]
fn when_script_moves_past_decision_then_chosen_drilling_outcome_is_set() {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::LaurelSnub)]);
    app.world_mut()
        .resource_mut::<bevy::prelude::NextState<GameState>>()
        .set(GameState::Decision);
    app.update();

    for _ in 0..3 {
        app.update();
    }

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::LaurelSnub
    );
    assert!(app.world().resource::<ChosenDrillingOutcome>().0.is_some());
}

#[test]
fn when_script_moves_past_vale_reaction_then_chosen_vale_reaction_is_set() {
    let mut app = setup_testing_app(vec![ScriptStep::WaitForState(GameState::Caught)]);
    app.world_mut()
        .resource_mut::<bevy::prelude::NextState<GameState>>()
        .set(GameState::ValeReaction);
    app.update();

    for _ in 0..3 {
        app.update();
    }

    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Caught
    );
    assert!(app.world().resource::<ChosenValeReaction>().0.is_some());
}
