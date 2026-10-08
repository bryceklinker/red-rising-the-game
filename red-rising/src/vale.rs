pub mod reaction;

use crate::game_state::GameState;
use crate::player::Player;
use bevy::prelude::{NextState, Query, ResMut, Transform, With};

const WINDOW_DISTANCE_THRESHOLD: f32 = 10.0;

pub fn window_trigger_state(distance_traveled: f32, threshold: f32) -> bool {
    distance_traveled >= threshold
}

pub fn check_vale_window_reached(
    player: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(transform) = player.single() else {
        return;
    };
    if window_trigger_state(transform.translation.y, WINDOW_DISTANCE_THRESHOLD) {
        next_state.set(GameState::ValeReaction);
    }
}
