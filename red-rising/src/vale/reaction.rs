use crate::game_state::GameState;
use bevy::prelude::{
    Button, Changed, Commands, Component, Entity, Interaction, NextState, Node, Query, ResMut,
    Resource, Text, With,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValeReactionChoice {
    Awe,
    Anger,
    Grief,
}

#[derive(Resource, Default)]
pub struct ChosenValeReaction(pub Option<ValeReactionChoice>);

#[derive(Component)]
pub struct ValeReactionUi;

#[derive(Component)]
pub struct ValeReactionButton(pub ValeReactionChoice);

const VALE_REACTION_OPTIONS: [(ValeReactionChoice, &str); 3] = [
    (ValeReactionChoice::Awe, "Stare in awe"),
    (ValeReactionChoice::Anger, "Grit your teeth in anger"),
    (ValeReactionChoice::Grief, "Think of Eo, and grieve"),
];

pub fn spawn_vale_reaction_ui(mut commands: Commands) {
    commands
        .spawn((ValeReactionUi, Node::default()))
        .with_children(|root| {
            for (choice, label) in VALE_REACTION_OPTIONS {
                root.spawn((ValeReactionButton(choice), Button, Node::default()))
                    .with_children(|button| {
                        button.spawn(Text::new(label));
                    });
            }
        });
}

pub fn handle_vale_reaction_button(
    interactions: Query<(&Interaction, &ValeReactionButton), Changed<Interaction>>,
    mut chosen: ResMut<ChosenValeReaction>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &interactions {
        if *interaction == Interaction::Pressed {
            chosen.0 = Some(button.0);
            next_state.set(GameState::Caught);
        }
    }
}

pub fn despawn_vale_reaction_ui(
    mut commands: Commands,
    query: Query<Entity, With<ValeReactionUi>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
