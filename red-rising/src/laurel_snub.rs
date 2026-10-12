use crate::decision::outcome::{DrillingOutcome, NarolReaction};
use crate::decision::ui::ChosenDrillingOutcome;
use crate::game_state::GameState;
use bevy::prelude::{
    Button, Changed, Commands, Component, Entity, Interaction, NextState, Node, Query, Res, ResMut,
    Text, With,
};

#[derive(Component)]
pub struct LaurelSnubBanner;

#[derive(Component)]
pub struct AcknowledgeLaurelSnubButton;

const NO_OUTCOME_RECORDED_MESSAGE: &str = "Lambda clan is denied the Laurel. Narol has nothing to say about numbers that were never taken.";

pub fn laurel_snub_text_for(outcome: DrillingOutcome) -> String {
    let yield_clause = if outcome.injured {
        format!(
            "Lambda pulls {} kilos, Darrow's arm raw from the burn",
            outcome.yield_kilos
        )
    } else {
        format!("Lambda pulls {} kilos, clean and safe", outcome.yield_kilos)
    };
    let reaction_clause = match outcome.narol_reaction {
        NarolReaction::AlarmedAndAngry => "Narol is furious they risked it for nothing.",
        NarolReaction::Relieved => {
            "Narol is relieved they stopped to check, for all the good it did."
        }
        NarolReaction::ApprovingButTired => "Narol nods, approving but worn thin.",
    };
    format!("{yield_clause} -- and the board still skips them for the Laurel. {reaction_clause}")
}

pub fn spawn_laurel_snub_banner(mut commands: Commands, chosen: Res<ChosenDrillingOutcome>) {
    let message = chosen
        .0
        .map(laurel_snub_text_for)
        .unwrap_or_else(|| NO_OUTCOME_RECORDED_MESSAGE.to_string());
    commands
        .spawn((LaurelSnubBanner, Node::default()))
        .with_children(|root| {
            root.spawn(Text::new(message));
            root.spawn((AcknowledgeLaurelSnubButton, Button, Node::default()))
                .with_children(|button| {
                    button.spawn(Text::new("Continue"));
                });
        });
}

pub fn handle_acknowledge_laurel_snub_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<AcknowledgeLaurelSnubButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Vale);
        }
    }
}

pub fn despawn_laurel_snub_banner(
    mut commands: Commands,
    query: Query<Entity, With<LaurelSnubBanner>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
