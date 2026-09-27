use bevy::prelude::{Camera3d, Commands, PointLight, Transform, Vec3, default};

pub fn spawn_scene(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
