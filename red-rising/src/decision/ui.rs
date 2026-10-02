use crate::decision::outcome::{DecisionOption, EndState, end_state_for_choice};
use crate::game_state::GameState;
use bevy::prelude::{
    Button, Changed, Commands, Component, Entity, Interaction, NextState, Node, Query, ResMut,
    Resource, Text, With,
};

#[derive(Component)]
pub struct DecisionUi;

#[derive(Component)]
pub struct DecisionButton(pub DecisionOption);

#[derive(Resource, Default)]
pub struct ChosenEndState(pub Option<EndState>);

const DECISION_OPTIONS: [(DecisionOption, &str); 3] = [
    (DecisionOption::KeepDrilling, "Keep drilling"),
    (DecisionOption::GetOutAndCheck, "Get out and check"),
    (DecisionOption::WaitForTheTeam, "Wait for the team"),
];

pub fn spawn_decision_ui(mut commands: Commands) {
    commands
        .spawn((DecisionUi, Node::default()))
        .with_children(|root| {
            for (option, label) in DECISION_OPTIONS {
                root.spawn((DecisionButton(option), Button, Node::default()))
                    .with_children(|button| {
                        button.spawn(Text::new(label));
                    });
            }
        });
}

pub fn handle_decision_button(
    interactions: Query<(&Interaction, &DecisionButton), Changed<Interaction>>,
    mut chosen: ResMut<ChosenEndState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &interactions {
        if *interaction == Interaction::Pressed {
            chosen.0 = Some(end_state_for_choice(button.0));
            next_state.set(GameState::End);
        }
    }
}

pub fn despawn_decision_ui(mut commands: Commands, query: Query<Entity, With<DecisionUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
