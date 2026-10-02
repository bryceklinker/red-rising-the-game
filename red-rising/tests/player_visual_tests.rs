use bevy::MinimalPlugins;
use bevy::asset::Assets;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{App, Mesh3d, MeshMaterial3d, Startup};
use red_rising::player::{spawn_player, spawn_player_mesh};

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.add_systems(Startup, (spawn_player, spawn_player_mesh).chain());
    app
}

#[test]
fn when_player_spawns_then_it_has_a_mesh_and_material() {
    let mut app = setup_testing_app();

    app.update();

    let world = app.world_mut();
    let mut query = world.query::<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>();
    assert_eq!(query.iter(world).count(), 1);
}
