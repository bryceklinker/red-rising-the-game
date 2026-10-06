use crate::diagnostics::format::format_diagnostic_line;
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::{
    ButtonInput, Commands, Component, KeyCode, Node, Query, Res, ResMut, Resource, Text,
    Visibility, With,
};

#[derive(Resource, Default)]
pub struct DiagnosticsOverlayVisible(pub bool);

#[derive(Component)]
pub struct DiagnosticsOverlayRoot;

#[derive(Component)]
pub struct DiagnosticsOverlayText;

pub fn toggle_diagnostics_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut visible: ResMut<DiagnosticsOverlayVisible>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        visible.0 = !visible.0;
    }
}

pub fn spawn_diagnostics_overlay(mut commands: Commands) {
    commands
        .spawn((DiagnosticsOverlayRoot, Node::default(), Visibility::Hidden))
        .with_children(|root| {
            root.spawn((DiagnosticsOverlayText, Text::new("")));
        });
}

pub fn update_diagnostics_overlay(
    diagnostics: Res<DiagnosticsStore>,
    mut text_query: Query<&mut Text, With<DiagnosticsOverlayText>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed());
    let entity_count = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|diagnostic| diagnostic.value());
    let cpu_usage = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)
        .and_then(|diagnostic| diagnostic.smoothed());
    let memory_usage = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE)
        .and_then(|diagnostic| diagnostic.smoothed());
    text.0 = [
        format_diagnostic_line("FPS", fps),
        format_diagnostic_line("Entities", entity_count),
        format_diagnostic_line("CPU", cpu_usage),
        format_diagnostic_line("Memory", memory_usage),
    ]
    .join("\n");
}

pub fn apply_diagnostics_overlay_visibility(
    visible: Res<DiagnosticsOverlayVisible>,
    mut query: Query<&mut Visibility, With<DiagnosticsOverlayRoot>>,
) {
    let Ok(mut visibility) = query.single_mut() else {
        return;
    };
    *visibility = if visible.0 {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}
