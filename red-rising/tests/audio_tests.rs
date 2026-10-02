use bevy::MinimalPlugins;
use bevy::asset::Assets;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackMode, PlaybackSettings};
use bevy::prelude::{App, AppExtStates, NextState};
use bevy::state::app::StatesPlugin;
use red_rising::audio::{CallBeepAudio, DrillRumbleAudio};
use red_rising::game_state::GameState;
use red_rising::plugins::audio_plugin::audio_plugin;

fn setup_testing_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.init_state::<GameState>();
    app.insert_resource(Assets::<AudioSource>::default());
    app.add_plugins(audio_plugin);
    app
}

#[test]
fn when_entering_drilling_then_drill_rumble_loops() {
    let mut app = setup_testing_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();
    app.update();

    let mut query = app
        .world_mut()
        .query::<(&DrillRumbleAudio, &AudioPlayer, &PlaybackSettings)>();
    let (_, _, settings) = query.iter(app.world()).next().unwrap();
    assert!(matches!(settings.mode, PlaybackMode::Loop));
}

#[test]
fn when_leaving_drilling_then_drill_rumble_is_despawned() {
    let mut app = setup_testing_app();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Drilling);
    app.update();
    app.update();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::CallEvent);
    app.update();
    app.update();

    let mut query = app.world_mut().query::<&DrillRumbleAudio>();
    assert_eq!(query.iter(app.world()).count(), 0);
}

#[test]
fn when_entering_call_event_then_beep_plays_once() {
    let mut app = setup_testing_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::CallEvent);
    app.update();
    app.update();

    let mut query = app
        .world_mut()
        .query::<(&CallBeepAudio, &AudioPlayer, &PlaybackSettings)>();
    let (_, _, settings) = query.iter(app.world()).next().unwrap();
    assert!(matches!(settings.mode, PlaybackMode::Despawn));
}
