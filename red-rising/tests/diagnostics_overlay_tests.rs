use bevy::MinimalPlugins;
use bevy::prelude::{App, ButtonInput, KeyCode, Update};
use red_rising::diagnostics::overlay::{DiagnosticsOverlayVisible, toggle_diagnostics_overlay};

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.init_resource::<DiagnosticsOverlayVisible>();
    app.add_systems(Update, toggle_diagnostics_overlay);
    app
}

// MinimalPlugins has no InputPlugin, so nothing clears `just_pressed` between
// frames the way a real app does in PreUpdate -- clear it manually here to
// emulate one real input frame.
fn press_f3_for_one_frame(app: &mut App) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::F3);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::F3);
}

#[test]
fn when_overlay_starts_then_it_is_hidden() {
    let app = setup_testing_app();

    assert!(!app.world().resource::<DiagnosticsOverlayVisible>().0);
}

#[test]
fn when_f3_is_pressed_then_overlay_becomes_visible() {
    let mut app = setup_testing_app();

    press_f3_for_one_frame(&mut app);

    assert!(app.world().resource::<DiagnosticsOverlayVisible>().0);
}

#[test]
fn when_f3_is_pressed_twice_then_overlay_toggles_back_off() {
    let mut app = setup_testing_app();

    press_f3_for_one_frame(&mut app);
    press_f3_for_one_frame(&mut app);

    assert!(!app.world().resource::<DiagnosticsOverlayVisible>().0);
}
