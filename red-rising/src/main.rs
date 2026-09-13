use bevy::prelude::*;

pub mod plugins;
pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}