use crate::decision::outcome::EndState;
use crate::decision::ui::ChosenEndState;
use bevy::prelude::{Commands, Component, Node, Res, Text};

#[derive(Component)]
pub struct EndUi;

pub fn end_message_for(end_state: EndState) -> &'static str {
    match end_state {
        EndState::KeptDrilling => "Darrow keeps drilling, gas pocket be damned.",
        EndState::GotOutAndChecked => "Darrow pulls the team out to check the reading.",
        EndState::WaitedForTheTeam => "Darrow waits for the team before going further.",
    }
}

const NO_CHOICE_MESSAGE: &str = "The demo has ended.";

pub fn spawn_end_ui(mut commands: Commands, chosen: Res<ChosenEndState>) {
    let message = chosen.0.map(end_message_for).unwrap_or(NO_CHOICE_MESSAGE);
    commands
        .spawn((EndUi, Node::default()))
        .with_children(|root| {
            root.spawn(Text::new(message));
        });
}
