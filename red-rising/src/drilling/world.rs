use avian3d::prelude::{Collider, RigidBody};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    Commands, Component, Cuboid, Entity, Mesh3d, MeshMaterial3d, Query, ResMut, Transform, With,
};

const GROUND_SIZE: f32 = 20.0;
const GROUND_THICKNESS: f32 = 1.0;

#[derive(Component)]
pub struct DrillingGround;

pub fn spawn_drilling_ground(mut commands: Commands) {
    commands.spawn((
        DrillingGround,
        RigidBody::Static,
        Collider::cuboid(GROUND_SIZE, GROUND_THICKNESS, GROUND_SIZE),
        Transform::from_xyz(0.0, -GROUND_THICKNESS / 2.0, 0.0),
    ));
}

pub fn spawn_drilling_ground_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ground: Query<Entity, With<DrillingGround>>,
) {
    let Ok(entity) = ground.single() else {
        return;
    };
    commands.entity(entity).insert((
        Mesh3d(meshes.add(Cuboid::new(GROUND_SIZE, GROUND_THICKNESS, GROUND_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.2, 0.15))),
    ));
}

pub fn despawn_drilling_ground(
    mut commands: Commands,
    ground: Query<Entity, With<DrillingGround>>,
) {
    for entity in &ground {
        commands.entity(entity).despawn();
    }
}
