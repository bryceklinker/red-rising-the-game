use bevy::prelude::{KeyCode, Vec3};
use red_rising::drilling::input::drill_input_to_velocity;

#[test]
fn when_no_keys_are_pressed_then_direction_is_zero() {
    let direction = drill_input_to_velocity(&[]);

    assert_eq!(direction, Vec3::ZERO);
}

#[test]
fn when_w_is_pressed_then_rig_drills_deeper() {
    let direction = drill_input_to_velocity(&[KeyCode::KeyW]);

    assert!(direction.y < 0.0);
}

#[test]
fn when_s_is_pressed_then_rig_retracts_upward() {
    let direction = drill_input_to_velocity(&[KeyCode::KeyS]);

    assert!(direction.y > 0.0);
}

#[test]
fn when_a_is_pressed_then_rig_strafes_left() {
    let direction = drill_input_to_velocity(&[KeyCode::KeyA]);

    assert!(direction.x < 0.0);
}

#[test]
fn when_d_is_pressed_then_rig_strafes_right() {
    let direction = drill_input_to_velocity(&[KeyCode::KeyD]);

    assert!(direction.x > 0.0);
}

#[test]
fn when_opposing_keys_are_pressed_then_they_cancel_out() {
    let direction = drill_input_to_velocity(&[KeyCode::KeyW, KeyCode::KeyS]);

    assert_eq!(direction, Vec3::ZERO);
}
