use crate::movement::input::get_direction_from_keys;
use bevy::prelude::{ButtonInput, Commands, Component, KeyCode, Query, Res, Transform, With};

const SPEED: f32 = 1.0;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Transform::default()));
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    let pressed_keys: Vec<KeyCode> = keyboard.get_pressed().copied().collect();
    let direction = get_direction_from_keys(&pressed_keys);
    transform.translation += direction * SPEED;
}
