use crate::camera::spawn_scene;
use bevy::prelude::{App, Startup};

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene);
}
