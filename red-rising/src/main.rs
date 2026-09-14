use bevy::prelude::*;
use red_rising::plugins::player_movement_plugin::player_movement_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player_movement_plugin)
        .run();
}