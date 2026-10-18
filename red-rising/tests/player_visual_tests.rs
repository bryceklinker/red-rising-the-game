use bevy::MinimalPlugins;
use bevy::asset::Assets;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{App, ChildOf, Mesh3d, MeshMaterial3d, Startup, With};
use red_rising::player::{Player, spawn_player, spawn_player_mesh};

const HUMANOID_PART_COUNT: usize = 6;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.add_systems(Startup, (spawn_player, spawn_player_mesh).chain());
    app
}

#[test]
fn when_player_spawns_then_it_has_a_composed_humanoid_mesh_and_material() {
    let mut app = setup_testing_app();

    app.update();

    let world = app.world_mut();
    let player_entity = world
        .query_filtered::<bevy::prelude::Entity, With<Player>>()
        .single(world)
        .unwrap();

    let mut parts = world.query::<(&Mesh3d, &MeshMaterial3d<StandardMaterial>, &ChildOf)>();
    let part_parents: Vec<_> = parts
        .iter(world)
        .map(|(_, _, child_of)| child_of.clone())
        .collect();
    assert_eq!(part_parents.len(), HUMANOID_PART_COUNT);
    assert!(
        part_parents
            .iter()
            .all(|child_of| child_of.parent() == player_entity)
    );
}
