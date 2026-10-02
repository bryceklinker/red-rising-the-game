use bevy::MinimalPlugins;
use bevy::diagnostic::{
    DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::{App, Startup, Text, Update};
use red_rising::diagnostics::overlay::{
    DiagnosticsOverlayText, spawn_diagnostics_overlay, update_diagnostics_overlay,
};

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(DiagnosticsPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(EntityCountDiagnosticsPlugin::default());
    app.add_systems(Startup, spawn_diagnostics_overlay);
    app.add_systems(Update, update_diagnostics_overlay);
    app
}

#[test]
fn when_diagnostics_update_runs_then_overlay_text_has_fps_and_entity_lines() {
    let mut app = setup_testing_app();

    app.update();
    app.update();

    let mut text_query = app.world_mut().query::<(&DiagnosticsOverlayText, &Text)>();
    let (_, text) = text_query.iter(app.world()).next().unwrap();
    assert!(text.0.contains("FPS: "));
    assert!(text.0.contains("Entities: "));
}
