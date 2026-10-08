use crate::game_state::GameState;
use bevy::prelude::{
    Button, Changed, Commands, Component, Entity, Interaction, NextState, Node, Query, ResMut,
    Text, With,
};

#[derive(Component)]
pub struct CallEventBanner;

#[derive(Component)]
pub struct AcknowledgeCallButton;

pub fn spawn_call_event_banner(mut commands: Commands) {
    commands
        .spawn((CallEventBanner, Node::default()))
        .with_children(|root| {
            root.spawn(Text::new(
                "Narol: \"Hold up -- I'm reading a gas pocket near your position.\"",
            ));
            root.spawn((AcknowledgeCallButton, Button, Node::default()))
                .with_children(|button| {
                    button.spawn(Text::new("Acknowledge"));
                });
        });
}

pub fn handle_acknowledge_call_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<AcknowledgeCallButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Decision);
        }
    }
}

pub fn despawn_call_event_banner(
    mut commands: Commands,
    query: Query<Entity, With<CallEventBanner>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
