use bevy::prelude::*;
use red_rising::game_state::GameState;
use red_rising::plugins::audio_plugin::audio_plugin;
use red_rising::plugins::call_event_plugin::call_event_plugin;
use red_rising::plugins::camera_plugin::camera_plugin;
use red_rising::plugins::caught_plugin::caught_plugin;
use red_rising::plugins::character_select_plugin::character_select_plugin;
use red_rising::plugins::decision_plugin::decision_plugin;
use red_rising::plugins::diagnostics_plugin::diagnostics_plugin;
use red_rising::plugins::drilling_plugin::drilling_plugin;
use red_rising::plugins::laurel_snub_plugin::laurel_snub_plugin;
use red_rising::plugins::player_movement_plugin::player_movement_plugin;
use red_rising::plugins::vale_plugin::vale_plugin;
use red_rising::plugins::vale_reaction_plugin::vale_reaction_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(diagnostics_plugin)
        .init_state::<GameState>()
        .add_plugins(player_movement_plugin)
        .add_plugins(camera_plugin)
        .add_plugins(character_select_plugin)
        .add_plugins(drilling_plugin)
        .add_plugins(call_event_plugin)
        .add_plugins(decision_plugin)
        .add_plugins(laurel_snub_plugin)
        .add_plugins(vale_plugin)
        .add_plugins(vale_reaction_plugin)
        .add_plugins(caught_plugin)
        .add_plugins(audio_plugin)
        .run();
}
