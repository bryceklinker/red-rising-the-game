use bevy::prelude::*;
use red_rising::game_state::GameState;
use red_rising::plugins::call_event_plugin::call_event_plugin;
use red_rising::plugins::camera_plugin::camera_plugin;
use red_rising::plugins::character_select_plugin::character_select_plugin;
use red_rising::plugins::drilling_plugin::drilling_plugin;
use red_rising::plugins::player_movement_plugin::player_movement_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(player_movement_plugin)
        .add_plugins(camera_plugin)
        .add_plugins(character_select_plugin)
        .add_plugins(drilling_plugin)
        .add_plugins(call_event_plugin)
        .run();
}
