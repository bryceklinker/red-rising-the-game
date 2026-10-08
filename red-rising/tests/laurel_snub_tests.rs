use bevy::MinimalPlugins;
use bevy::prelude::{App, AppExtStates, Entity, Interaction, NextState, State, Text, With};
use bevy::state::app::StatesPlugin;
use red_rising::decision::outcome::{DrillingOutcome, NarolReaction};
use red_rising::decision::ui::ChosenDrillingOutcome;
use red_rising::game_state::GameState;
use red_rising::laurel_snub::{AcknowledgeLaurelSnubButton, laurel_snub_text_for};
use red_rising::plugins::laurel_snub_plugin::laurel_snub_plugin;

fn setup_testing_app(outcome: DrillingOutcome) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.insert_resource(ChosenDrillingOutcome(Some(outcome)));
    app.add_plugins(laurel_snub_plugin);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::LaurelSnub);
    app.update();
    app.update();
    app
}

fn acknowledge(app: &mut App) {
    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<AcknowledgeLaurelSnubButton>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(entity)
        .insert(Interaction::Pressed);
    app.update();
    app.update();
}

fn outcomes() -> [DrillingOutcome; 3] {
    [
        DrillingOutcome {
            yield_kilos: 48,
            injured: true,
            narol_reaction: NarolReaction::AlarmedAndAngry,
        },
        DrillingOutcome {
            yield_kilos: 32,
            injured: false,
            narol_reaction: NarolReaction::Relieved,
        },
        DrillingOutcome {
            yield_kilos: 20,
            injured: false,
            narol_reaction: NarolReaction::ApprovingButTired,
        },
    ]
}

#[test]
fn when_entering_laurel_snub_then_banner_text_matches_the_outcome_for_every_variant() {
    for outcome in outcomes() {
        let mut app = setup_testing_app(outcome);

        let mut text_query = app.world_mut().query::<&Text>();
        let text = text_query.iter(app.world()).next().unwrap();
        assert_eq!(text.0, laurel_snub_text_for(outcome));
    }
}

#[test]
fn when_acknowledged_then_every_outcome_variant_transitions_to_vale() {
    for outcome in outcomes() {
        let mut app = setup_testing_app(outcome);

        acknowledge(&mut app);

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Vale);
    }
}
