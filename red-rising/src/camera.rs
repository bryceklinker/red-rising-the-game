use crate::player::Player;
use bevy::prelude::{
    Camera3d, Commands, Component, PointLight, Query, Transform, Vec3, With, Without, default,
};

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 3.0, 8.0);

#[derive(Component)]
pub struct MainCamera;

pub fn camera_position_for(player_position: Vec3, offset: Vec3) -> Vec3 {
    player_position + offset
}

pub fn spawn_scene(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_translation(camera_position_for(Vec3::ZERO, CAMERA_OFFSET))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn follow_camera(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };
    camera_transform.translation = camera_position_for(player_transform.translation, CAMERA_OFFSET);
}
