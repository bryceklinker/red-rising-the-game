use crate::capture::script::{ScriptStep, ScriptedSession};
use crate::capture::{CAPTURE_OUTPUT_DIR, CaptureTarget};
use crate::game_state::GameState;
use bevy::app::AppExit;
use bevy::log::info;
use bevy::prelude::{Commands, MessageWriter, Res, ResMut, Resource, State};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

/// Bounded at 30 ticks/sec (see `ScheduleRunnerPlugin` in capture_plugin.rs),
/// so this is a ~10 second budget for any single state transition to land.
pub const WAIT_FOR_STATE_TIMEOUT_TICKS: u32 = 300;

#[derive(Resource, Default)]
pub struct ScriptWatchdog(u32);

impl ScriptWatchdog {
    pub fn ticks_waited(&self) -> u32 {
        self.0
    }
}

pub fn advance_scripted_session(
    mut session: ResMut<ScriptedSession>,
    current_state: Res<State<GameState>>,
    capture_target: Option<Res<CaptureTarget>>,
    mut app_exit: MessageWriter<AppExit>,
    mut commands: Commands,
    mut watchdog: ResMut<ScriptWatchdog>,
) {
    let Some(step) = session.front().copied() else {
        return;
    };

    match step {
        ScriptStep::AdvanceTicks(remaining) => advance_ticks(&mut session, remaining),
        ScriptStep::WaitForState(expected) => {
            wait_for_state(&mut session, &current_state, expected, &mut watchdog);
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
    watchdog: &mut ScriptWatchdog,
) {
    if *current_state.get() == expected {
        session.pop_front();
        watchdog.0 = 0;
        return;
    }

    watchdog.0 += 1;
    if watchdog.0 > WAIT_FOR_STATE_TIMEOUT_TICKS {
        panic!(
            "capture watchdog: WaitForState(GameState::{expected:?}) did not resolve within {WAIT_FOR_STATE_TIMEOUT_TICKS} ticks (current state: {:?}) -- scripted capture session is stuck",
            current_state.get()
        );
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
