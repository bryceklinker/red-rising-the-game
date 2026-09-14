use bevy::prelude::{KeyCode, Vec3};
use rstest::rstest;
use red_rising::movement::input::get_direction_from_keys;

#[test]
fn when_no_keys_are_pressed_then_direction_is_zero() {
    let direction = get_direction_from_keys(&[]);

    assert_eq!(direction, Vec3::ZERO);
}

#[rstest]
#[case::forward(KeyCode::KeyW, 1.0)]
#[case::backward(KeyCode::KeyS, -1.0)]
fn when_key_is_pressed_then_y_direction_is_adjusted(#[case] key: KeyCode, #[case] expected_y: f32) {
    let direction = get_direction_from_keys(&[key]);

    assert_eq!(direction.y, expected_y);
}

#[rstest]
#[case::right(KeyCode::KeyD, 1.0)]
#[case::right(KeyCode::KeyA, -1.0)]
fn when_key_is_pressed_then_x_direction_is_adjusted(#[case] key: KeyCode, #[case] expected_x: f32) {
    let direction = get_direction_from_keys(&[key]);

    assert_eq!(direction.x, expected_x);
}