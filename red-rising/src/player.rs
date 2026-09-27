use crate::movement::input::get_direction_from_keys;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    ButtonInput, Capsule3d, Commands, Component, KeyCode, Mesh3d, MeshMaterial3d, Query, Res,
    ResMut, Transform, With,
};

const SPEED: f32 = 1.0;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        Transform::default(),
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
    ));
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
