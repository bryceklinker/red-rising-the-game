use bevy::prelude::Vec3;
use red_rising::camera::camera_position_for;

#[test]
fn when_player_is_at_origin_then_camera_is_at_offset() {
    let position = camera_position_for(Vec3::ZERO, Vec3::new(0.0, 3.0, 8.0));

    assert_eq!(position, Vec3::new(0.0, 3.0, 8.0));
}

#[test]
fn when_player_has_moved_diagonally_then_camera_follows_by_the_same_offset() {
    let player_position = Vec3::new(2.0, 0.0, -5.0);
    let offset = Vec3::new(0.0, 3.0, 8.0);

    let position = camera_position_for(player_position, offset);

    assert_eq!(position, Vec3::new(2.0, 3.0, 3.0));
}

#[test]
fn when_called_twice_with_the_same_inputs_then_result_is_the_same() {
    let player_position = Vec3::new(1.0, 2.0, 3.0);
    let offset = Vec3::new(0.0, 3.0, 8.0);

    let first = camera_position_for(player_position, offset);
    let second = camera_position_for(player_position, offset);

    assert_eq!(first, second);
}
