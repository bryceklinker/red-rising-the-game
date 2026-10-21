use crate::movement::input::get_direction_from_keys;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    ButtonInput, Capsule3d, Commands, Component, Entity, KeyCode, Mesh3d, MeshMaterial3d, Query,
    Res, ResMut, Sphere, Transform, Vec3, With, default,
};

const SPEED: f32 = 1.0;

// Proportions keep the same overall footprint (radius 0.4, height 1.8) as the
// old placeholder capsule so movement/camera framing doesn't shift.
const HEAD_RADIUS: f32 = 0.18;
const TORSO_RADIUS: f32 = 0.22;
const TORSO_LENGTH: f32 = 0.16;
const LIMB_RADIUS: f32 = 0.11;
const ARM_LENGTH: f32 = 0.33;
const LEG_RADIUS: f32 = 0.13;
const LEG_LENGTH: f32 = 0.54;
const ARM_OFFSET_X: f32 = 0.35;
const LEG_OFFSET_X: f32 = 0.12;
const HEAD_CENTER_Y: f32 = 0.7;
const TORSO_CENTER_Y: f32 = 0.2;
const ARM_CENTER_Y: f32 = 0.225;
const LEG_CENTER_Y: f32 = -0.5;
const KNEE_CENTER_Y: f32 = LEG_CENTER_Y + 0.1;

const DUST_PATCH_RADIUS: f32 = 0.05;
const DUST_PATCH_SCALE: Vec3 = Vec3::new(1.0, 1.0, 0.3);

const SKIN_COLOR: Color = Color::srgb(0.72, 0.42, 0.32);
// Cheap Red-caste fry suit: a worn, utilitarian work-suit rather than bare limbs.
const SUIT_COLOR: Color = Color::srgb_u8(46, 16, 8);
// StandardMaterial has no texture-free multi-layer/decal blending, so patchy
// Mars dirt is approximated with small flattened primitives in a lighter dust
// tone laid on the suit's surface, rather than a uniform tint over the whole suit.
const DUST_COLOR: Color = Color::srgb_u8(120, 70, 45);

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Transform::default()));
}

pub fn spawn_player_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Query<Entity, With<Player>>,
) {
    let Ok(entity) = player.single() else {
        return;
    };

    let skin = materials.add(StandardMaterial {
        base_color: SKIN_COLOR,
        perceptual_roughness: 0.9,
        ..default()
    });
    let suit = materials.add(StandardMaterial {
        base_color: SUIT_COLOR,
        perceptual_roughness: 0.95,
        ..default()
    });
    let dust = materials.add(StandardMaterial {
        base_color: DUST_COLOR,
        perceptual_roughness: 1.0,
        ..default()
    });

    let head_mesh = meshes.add(Sphere::new(HEAD_RADIUS));
    let torso_mesh = meshes.add(Capsule3d::new(TORSO_RADIUS, TORSO_LENGTH));
    let arm_mesh = meshes.add(Capsule3d::new(LIMB_RADIUS, ARM_LENGTH));
    let leg_mesh = meshes.add(Capsule3d::new(LEG_RADIUS, LEG_LENGTH));
    let dust_patch_mesh = meshes.add(Sphere::new(DUST_PATCH_RADIUS));

    commands.entity(entity).with_children(|body| {
        body.spawn((
            Mesh3d(head_mesh),
            MeshMaterial3d(skin),
            Transform::from_xyz(0.0, HEAD_CENTER_Y, 0.0),
        ));
        body.spawn((
            Mesh3d(torso_mesh),
            MeshMaterial3d(suit.clone()),
            Transform::from_xyz(0.0, TORSO_CENTER_Y, 0.0),
        ));
        body.spawn((
            Mesh3d(arm_mesh.clone()),
            MeshMaterial3d(suit.clone()),
            Transform::from_xyz(-ARM_OFFSET_X, ARM_CENTER_Y, 0.0),
        ));
        body.spawn((
            Mesh3d(arm_mesh),
            MeshMaterial3d(suit.clone()),
            Transform::from_xyz(ARM_OFFSET_X, ARM_CENTER_Y, 0.0),
        ));
        body.spawn((
            Mesh3d(leg_mesh.clone()),
            MeshMaterial3d(suit.clone()),
            Transform::from_xyz(-LEG_OFFSET_X, LEG_CENTER_Y, 0.0),
        ));
        body.spawn((
            Mesh3d(leg_mesh),
            MeshMaterial3d(suit),
            Transform::from_xyz(LEG_OFFSET_X, LEG_CENTER_Y, 0.0),
        ));
        body.spawn((
            Mesh3d(dust_patch_mesh.clone()),
            MeshMaterial3d(dust.clone()),
            Transform::from_xyz(0.0, TORSO_CENTER_Y + 0.02, TORSO_RADIUS - 0.03)
                .with_scale(DUST_PATCH_SCALE),
        ));
        body.spawn((
            Mesh3d(dust_patch_mesh.clone()),
            MeshMaterial3d(dust.clone()),
            Transform::from_xyz(-LEG_OFFSET_X, KNEE_CENTER_Y, LEG_RADIUS - 0.02)
                .with_scale(DUST_PATCH_SCALE),
        ));
        body.spawn((
            Mesh3d(dust_patch_mesh),
            MeshMaterial3d(dust),
            Transform::from_xyz(LEG_OFFSET_X, KNEE_CENTER_Y, LEG_RADIUS - 0.02)
                .with_scale(DUST_PATCH_SCALE),
        ));
    });
}

pub fn despawn_player(mut commands: Commands, player: Query<Entity, With<Player>>) {
    for entity in &player {
        commands.entity(entity).despawn();
    }
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
