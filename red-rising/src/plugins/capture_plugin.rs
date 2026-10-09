use crate::call_event::AcknowledgeCallButton;
use crate::camera::{MainCamera, spawn_scene};
use crate::capture::runner::advance_scripted_session;
use crate::capture::script::{ScriptStep, ScriptedSession};
use crate::capture::{CAPTURE_OUTPUT_DIR, CaptureTarget, prologue_script};
use crate::character_select::CharacterSelectButton;
use crate::decision::ui::DecisionButton;
use crate::game_state::GameState;
use bevy::app::ScheduleRunnerPlugin;
use bevy::camera::{Camera, ClearColorConfig, RenderTarget};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::log::info;
use bevy::prelude::{
    App, Assets, ButtonInput, Camera2d, Commands, Component, Entity, Image, Interaction,
    IsDefaultUiCamera, KeyCode, Query, Res, ResMut, Startup, State, Update, With, default,
    in_state,
};
use bevy::render::render_resource::{TextureFormat, TextureUsages};
use std::time::Duration;

const CAPTURE_WIDTH: u32 = 1280;
const CAPTURE_HEIGHT: u32 = 720;

pub fn capture_plugin(app: &mut App) {
    app.add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        1.0 / 30.0,
    )));
    app.insert_resource(ScriptedSession::new(prologue_script()));
    app.add_systems(Startup, setup_capture_render_target.after(spawn_scene));
    app.add_systems(
        Update,
        (
            advance_scripted_session,
            auto_press::<CharacterSelectButton>.run_if(in_state(GameState::CharacterSelect)),
            auto_drill.run_if(in_state(GameState::Drilling)),
            auto_press::<AcknowledgeCallButton>.run_if(in_state(GameState::CallEvent)),
            auto_press::<DecisionButton>.run_if(in_state(GameState::Decision)),
        ),
    );
}

fn setup_capture_render_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    camera: Query<Entity, With<MainCamera>>,
) {
    std::fs::create_dir_all(CAPTURE_OUTPUT_DIR).expect("failed to create capture output directory");
    let handle = images.add(capture_render_target_image());
    let camera_entity = camera.single().expect("MainCamera must exist by Startup");

    info!("capture: redirecting MainCamera {camera_entity:?} to off-screen render target");
    commands
        .entity(camera_entity)
        .insert(RenderTarget::Image(handle.clone().into()));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderTarget::Image(handle.clone().into()),
        IsDefaultUiCamera,
    ));

    commands.insert_resource(CaptureTarget(handle));
}

fn capture_render_target_image() -> Image {
    let mut image = Image::new_target_texture(
        CAPTURE_WIDTH,
        CAPTURE_HEIGHT,
        TextureFormat::Rgba8UnormSrgb,
        None,
    );
    image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    image
}

fn auto_press<B: Component>(
    mut buttons: Query<&mut Interaction, With<B>>,
    session: Res<ScriptedSession>,
    current_state: Res<State<GameState>>,
) {
    if !script_has_moved_past(&session, *current_state.get()) {
        return;
    }
    for mut interaction in &mut buttons {
        *interaction = Interaction::Pressed;
    }
}

fn script_has_moved_past(session: &ScriptedSession, current_state: GameState) -> bool {
    match session.front() {
        Some(ScriptStep::WaitForState(next)) => *next != current_state,
        Some(ScriptStep::Exit) => true,
        _ => false,
    }
}

fn auto_drill(mut keyboard: ResMut<ButtonInput<KeyCode>>) {
    keyboard.press(KeyCode::KeyW);
}
