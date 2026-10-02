use crate::drilling::input::drill_input_to_velocity;
use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    ButtonInput, Commands, Component, Cylinder, Entity, KeyCode, Mesh3d, MeshMaterial3d, Query,
    Res, ResMut, Transform, With,
};

const DRILL_SPEED: f32 = 2.0;
const DRILL_RIG_RADIUS: f32 = 0.5;
const DRILL_RIG_HEIGHT: f32 = 1.0;

#[derive(Component)]
pub struct DrillRig;

pub fn spawn_drill_rig(mut commands: Commands) {
    commands.spawn((
        DrillRig,
        RigidBody::Dynamic,
        Collider::cylinder(DRILL_RIG_RADIUS, DRILL_RIG_HEIGHT),
        LinearVelocity::default(),
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}

pub fn spawn_drill_rig_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rig: Query<Entity, With<DrillRig>>,
) {
    let Ok(entity) = rig.single() else {
        return;
    };
    commands.entity(entity).insert((
        Mesh3d(meshes.add(Cylinder::new(DRILL_RIG_RADIUS, DRILL_RIG_HEIGHT))),
        MeshMaterial3d(materials.add(Color::srgb(0.6, 0.4, 0.1))),
    ));
}

pub fn despawn_drill_rig(mut commands: Commands, rig: Query<Entity, With<DrillRig>>) {
    for entity in &rig {
        commands.entity(entity).despawn();
    }
}

pub fn drive_drill_rig(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, bevy::prelude::With<DrillRig>>,
) {
    let pressed_keys: Vec<KeyCode> = keyboard.get_pressed().copied().collect();
    let direction = drill_input_to_velocity(&pressed_keys);
    for mut velocity in &mut query {
        velocity.0 = direction * DRILL_SPEED;
    }
}
