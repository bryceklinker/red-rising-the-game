use bevy::prelude::{Button, Commands, Component, Entity, Node, Query, Text, With};

#[derive(Component)]
pub struct CallEventBanner;

#[derive(Component)]
pub struct AcknowledgeCallButton;

pub fn spawn_call_event_banner(mut commands: Commands) {
    commands
        .spawn((CallEventBanner, Node::default()))
        .with_children(|root| {
            root.spawn(Text::new(
                "Nero: \"Hold up -- I'm reading a gas pocket near your position.\"",
            ));
            root.spawn((AcknowledgeCallButton, Button, Node::default()))
                .with_children(|button| {
                    button.spawn(Text::new("Acknowledge"));
                });
        });
}

pub fn despawn_call_event_banner(
    mut commands: Commands,
    query: Query<Entity, With<CallEventBanner>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
