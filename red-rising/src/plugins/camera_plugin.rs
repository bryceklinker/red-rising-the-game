use crate::camera::{follow_camera, spawn_scene};
use crate::player::{spawn_player, spawn_player_mesh};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, Startup, Update};

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene);
    app.add_systems(Startup, spawn_player_mesh.after(spawn_player));
    app.add_systems(Update, follow_camera);
}
