use crate::drilling::rig::DrillRig;
use crate::game_state::GameState;
use bevy::prelude::{NextState, Query, ResMut, Transform, With};

const DEPTH_THRESHOLD: f32 = 10.0;

pub fn call_trigger_state(depth: f32, threshold: f32) -> bool {
    depth >= threshold
}

pub fn check_drill_depth(
    rig: Query<&Transform, With<DrillRig>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(transform) = rig.single() else {
        return;
    };
    let depth = -transform.translation.y;
    if call_trigger_state(depth, DEPTH_THRESHOLD) {
        next_state.set(GameState::CallEvent);
    }
}
