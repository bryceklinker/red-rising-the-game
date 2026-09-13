use bevy::prelude::{ButtonInput, Commands, Component, KeyCode, Query, Res, Transform, With, Vec3};

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
    let Ok(mut transform) = query.single_mut() else { return };
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    transform.translation += direction.normalize_or_zero() * SPEED;
}