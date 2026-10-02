use crate::drilling::depth::check_drill_depth;
use crate::drilling::rig::{
    despawn_drill_rig, drive_drill_rig, spawn_drill_rig, spawn_drill_rig_mesh,
};
use crate::drilling::world::{
    despawn_drilling_ground, spawn_drilling_ground, spawn_drilling_ground_mesh,
};
use crate::game_state::GameState;
use avian3d::prelude::{Gravity, PhysicsPlugins};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, OnEnter, OnExit, Update, Vec3, in_state};

const MARS_GRAVITY: f32 = 3.71;

pub fn drilling_plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.insert_resource(Gravity(Vec3::NEG_Y * MARS_GRAVITY));
    app.add_systems(
        OnEnter(GameState::Drilling),
        (spawn_drill_rig, spawn_drilling_ground),
    );
    app.add_systems(
        OnEnter(GameState::Drilling),
        (
            spawn_drill_rig_mesh.after(spawn_drill_rig),
            spawn_drilling_ground_mesh.after(spawn_drilling_ground),
        ),
    );
    app.add_systems(
        Update,
        (
            drive_drill_rig.run_if(in_state(GameState::Drilling)),
            check_drill_depth.run_if(in_state(GameState::Drilling)),
        ),
    );
    app.add_systems(
        OnExit(GameState::Drilling),
        (despawn_drill_rig, despawn_drilling_ground),
    );
}
