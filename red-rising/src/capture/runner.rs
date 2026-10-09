use crate::capture::script::{ScriptStep, ScriptedSession};
use crate::capture::{CAPTURE_OUTPUT_DIR, CaptureTarget};
use crate::game_state::GameState;
use bevy::app::AppExit;
use bevy::log::info;
use bevy::prelude::{Commands, MessageWriter, Res, ResMut, State};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

pub fn advance_scripted_session(
    mut session: ResMut<ScriptedSession>,
    current_state: Res<State<GameState>>,
    capture_target: Option<Res<CaptureTarget>>,
    mut app_exit: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    let Some(step) = session.front().copied() else {
        return;
    };

    match step {
        ScriptStep::AdvanceTicks(remaining) => advance_ticks(&mut session, remaining),
        ScriptStep::WaitForState(expected) => {
            wait_for_state(&mut session, &current_state, expected);
        }
        ScriptStep::Capture(name) => {
            capture_frame(&mut session, &mut commands, capture_target, name)
        }
        ScriptStep::Exit => exit_session(&mut session, &mut app_exit),
    }
}

fn advance_ticks(session: &mut ScriptedSession, remaining: u32) {
    session.pop_front();
    if remaining > 1 {
        session.push_front(ScriptStep::AdvanceTicks(remaining - 1));
    }
}

fn wait_for_state(
    session: &mut ScriptedSession,
    current_state: &State<GameState>,
    expected: GameState,
) {
    if *current_state.get() == expected {
        session.pop_front();
    }
}

fn capture_frame(
    session: &mut ScriptedSession,
    commands: &mut Commands,
    capture_target: Option<Res<CaptureTarget>>,
    name: &'static str,
) {
    session.pop_front();
    let Some(target) = capture_target else {
        return;
    };
    let path = format!("{CAPTURE_OUTPUT_DIR}/{name}.png");
    info!("capture: requesting screenshot '{name}' -> {path}");
    commands
        .spawn(Screenshot::image(target.0.clone()))
        .observe(save_to_disk(path));
}

fn exit_session(session: &mut ScriptedSession, app_exit: &mut MessageWriter<AppExit>) {
    session.pop_front();
    app_exit.write(AppExit::Success);
}
