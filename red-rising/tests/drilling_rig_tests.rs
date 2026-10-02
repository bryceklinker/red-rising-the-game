use avian3d::prelude::LinearVelocity;
use bevy::MinimalPlugins;
use bevy::prelude::{App, ButtonInput, KeyCode, Update};
use red_rising::drilling::rig::{DrillRig, drive_drill_rig};

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.add_systems(Update, drive_drill_rig);
    app
}

#[test]
fn when_d_is_pressed_then_rig_velocity_moves_right() {
    let mut app = setup_testing_app();
    app.world_mut().spawn((DrillRig, LinearVelocity::default()));

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    app.update();

    let mut query = app.world_mut().query::<&LinearVelocity>();
    let velocity = query.single(app.world()).unwrap();
    assert!(velocity.x > 0.0);
    assert_eq!(velocity.y, 0.0);
}

#[test]
fn when_w_is_pressed_then_rig_velocity_drills_deeper() {
    let mut app = setup_testing_app();
    app.world_mut().spawn((DrillRig, LinearVelocity::default()));

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    app.update();

    let mut query = app.world_mut().query::<&LinearVelocity>();
    let velocity = query.single(app.world()).unwrap();
    assert!(velocity.y < 0.0);
}
