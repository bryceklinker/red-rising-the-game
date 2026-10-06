use crate::diagnostics::overlay::{
    DiagnosticsOverlayVisible, apply_diagnostics_overlay_visibility, spawn_diagnostics_overlay,
    toggle_diagnostics_overlay, update_diagnostics_overlay,
};
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::{App, Startup, Update};
use bevy::render::diagnostic::RenderDiagnosticsPlugin;

pub fn diagnostics_plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(EntityCountDiagnosticsPlugin::default());
    app.add_plugins(SystemInformationDiagnosticsPlugin);
    app.add_plugins(RenderDiagnosticsPlugin);
    app.init_resource::<DiagnosticsOverlayVisible>();
    app.add_systems(Startup, spawn_diagnostics_overlay);
    app.add_systems(
        Update,
        (
            toggle_diagnostics_overlay,
            update_diagnostics_overlay,
            apply_diagnostics_overlay_visibility,
        ),
    );
}
