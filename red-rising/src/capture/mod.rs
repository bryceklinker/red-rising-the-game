pub mod auto_advance;
pub mod runner;
pub mod script;

use crate::game_state::GameState;
use bevy::prelude::{Handle, Image, Resource};
use script::ScriptStep;

pub const CAPTURE_OUTPUT_DIR: &str = "target/capture";

#[derive(Resource, Clone)]
pub struct CaptureTarget(pub Handle<Image>);

/// Extra ticks held between a screen's `WaitForState` resolving and its
/// `Capture` firing, so any first-time GPU work that screen's render triggers
/// (texture upload, compiling a not-yet-used UI pipeline, etc.) has landed
/// before the screenshot readback. `WaitForState` is satisfied the instant
/// `current_state == expected`, which for the state machine's `#[default]`
/// (`GameState::CharacterSelect`) is true from tick zero -- so without this
/// buffer that screen gets none of the extra buffered frames every other
/// screen incidentally gains from its own OnExit/StateTransition/OnEnter
/// machinery. CI's llvmpipe software rasterizer is slow enough that a bare
/// `WaitForState` -> `Capture` can read back a frame before compositing
/// finishes, producing a flat `ClearColor` screenshot.
const CAPTURE_SETTLE_TICKS: u32 = 10;

fn wait_and_capture(state: GameState, name: &'static str) -> [ScriptStep; 3] {
    [
        ScriptStep::WaitForState(state),
        ScriptStep::AdvanceTicks(CAPTURE_SETTLE_TICKS),
        ScriptStep::Capture(name),
    ]
}

pub fn prologue_script() -> Vec<ScriptStep> {
    let mut steps = vec![ScriptStep::AdvanceTicks(2)];
    steps.extend(wait_and_capture(
        GameState::CharacterSelect,
        "character_select",
    ));
    steps.extend(wait_and_capture(GameState::Drilling, "drilling"));
    steps.extend(wait_and_capture(GameState::CallEvent, "call_event"));
    steps.extend(wait_and_capture(GameState::Decision, "decision"));
    steps.extend(wait_and_capture(GameState::LaurelSnub, "laurel_snub"));
    steps.extend(wait_and_capture(GameState::Vale, "vale"));
    steps.extend(wait_and_capture(GameState::ValeReaction, "vale_reaction"));
    steps.extend(wait_and_capture(GameState::Caught, "caught"));
    steps.push(ScriptStep::AdvanceTicks(30));
    steps.push(ScriptStep::Exit);
    steps
}
