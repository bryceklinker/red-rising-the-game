use avian3d::prelude::{Gravity, PhysicsPlugins};
use bevy::MinimalPlugins;
use bevy::asset::AssetPlugin;
use bevy::prelude::{App, ButtonInput, KeyCode, Startup, Transform, Update, Vec3};
use bevy::time::TimeUpdateStrategy;
use bevy::transform::TransformPlugin;
use red_rising::drilling::rig::{DrillRig, drive_drill_rig, spawn_drill_rig};
use red_rising::drilling::world::spawn_drilling_ground;
use std::time::Duration;

const MARS_GRAVITY: f32 = 3.71;
const DEPTH_THRESHOLD: f32 = 10.0;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        PhysicsPlugins::default(),
        AssetPlugin::default(),
        bevy::mesh::MeshPlugin,
        bevy::scene::ScenePlugin,
    ));
    app.insert_resource(Gravity(Vec3::NEG_Y * MARS_GRAVITY));
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
        1.0 / 60.0,
    )));
    app.add_systems(Startup, (spawn_drill_rig, spawn_drilling_ground));
    app.add_systems(Update, drive_drill_rig);
    app.finish();
    app
}

#[test]
fn when_w_is_held_then_the_rig_descends_past_the_depth_threshold_without_getting_stuck_on_the_ground()
 {
    let mut app = setup_testing_app();
    app.update();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);

    for _ in 0..600 {
        app.update();
    }

    let mut query = app.world_mut().query::<(&DrillRig, &Transform)>();
    let (_, transform) = query.iter(app.world()).next().unwrap();
    assert!(
        -transform.translation.y >= DEPTH_THRESHOLD,
        "rig only reached y = {} (depth {})",
        transform.translation.y,
        -transform.translation.y
    );
}
