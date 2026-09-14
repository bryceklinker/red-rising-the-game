use bevy::math::Vec3;
use bevy::prelude::KeyCode;

pub fn get_direction_from_keys(pressed_keys: &[KeyCode]) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if pressed_keys.contains(&KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if pressed_keys.contains(&KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if pressed_keys.contains(&KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if pressed_keys.contains(&KeyCode::KeyD) {
        direction.x += 1.0;
    }
    return direction.normalize_or_zero();
}