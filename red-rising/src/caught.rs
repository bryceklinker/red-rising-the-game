use bevy::prelude::{Commands, Component, Node, Text};

#[derive(Component)]
pub struct CaughtBanner;

pub fn spawn_caught_banner(mut commands: Commands) {
    commands
        .spawn((CaughtBanner, Node::default()))
        .with_children(|root| {
            root.spawn(Text::new(
                "Grays close in from both ends of the access tunnel. There's nowhere left to run.",
            ));
        });
}
