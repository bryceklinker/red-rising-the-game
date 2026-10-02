use crate::game_state::GameState;
use bevy::prelude::{
    Button, Changed, Commands, Component, Entity, Interaction, NextState, Node, Query, ResMut,
    Text, With,
};

#[derive(Component)]
pub struct CharacterSelectUi;

#[derive(Component)]
pub struct CharacterSelectButton;

pub fn spawn_character_select_ui(mut commands: Commands) {
    commands
        .spawn((CharacterSelectUi, Node::default()))
        .with_children(|root| {
            root.spawn((CharacterSelectButton, Button, Node::default()))
                .with_children(|button| {
                    button.spawn(Text::new("Darrow"));
                });
        });
}

pub fn handle_character_select_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<CharacterSelectButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Drilling);
        }
    }
}

pub fn despawn_character_select_ui(
    mut commands: Commands,
    query: Query<Entity, With<CharacterSelectUi>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
