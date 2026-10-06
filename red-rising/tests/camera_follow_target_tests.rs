use bevy::MinimalPlugins;
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{App, AppExtStates, ButtonInput, KeyCode, NextState, Transform, Vec3, With};
use bevy::state::app::StatesPlugin;
use red_rising::camera::MainCamera;
use red_rising::drilling::rig::DrillRig;
use red_rising::game_state::GameState;
use red_rising::player::Player;
use red_rising::plugins::camera_plugin::camera_plugin;
use red_rising::plugins::player_movement_plugin::player_movement_plugin;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.add_plugins(player_movement_plugin);
    app.add_plugins(camera_plugin);
    app.update();
    app
}

#[test]
fn when_entering_drilling_then_the_character_select_capsule_is_despawned() {
    let mut app = setup_testing_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&Player>();
    assert_eq!(query.iter(app.world()).count(), 0);
}

#[test]
fn when_a_drill_rig_exists_then_the_camera_follows_it_instead_of_the_player() {
    let mut app = setup_testing_app();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();
    app.update();

    app.world_mut()
        .spawn((DrillRig, Transform::from_xyz(3.0, -4.0, 0.0)));
    app.update();

    let mut query = app
        .world_mut()
        .query_filtered::<&Transform, With<MainCamera>>();
    let camera_transform = query.iter(app.world()).next().unwrap();
    assert_eq!(camera_transform.translation, Vec3::new(3.0, -1.0, 8.0));
}
