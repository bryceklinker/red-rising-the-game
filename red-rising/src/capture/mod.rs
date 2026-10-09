pub mod auto_advance;
pub mod runner;
pub mod script;

use crate::game_state::GameState;
use bevy::prelude::{Handle, Image, Resource};
use script::ScriptStep;

pub const CAPTURE_OUTPUT_DIR: &str = "target/capture";

#[derive(Resource, Clone)]
pub struct CaptureTarget(pub Handle<Image>);

pub fn prologue_script() -> Vec<ScriptStep> {
    vec![
        ScriptStep::AdvanceTicks(2),
        ScriptStep::WaitForState(GameState::CharacterSelect),
        ScriptStep::Capture("character_select"),
        ScriptStep::WaitForState(GameState::Drilling),
        ScriptStep::Capture("drilling"),
        ScriptStep::WaitForState(GameState::CallEvent),
        ScriptStep::Capture("call_event"),
        ScriptStep::WaitForState(GameState::Decision),
        ScriptStep::Capture("decision"),
        ScriptStep::WaitForState(GameState::End),
        ScriptStep::Capture("end"),
        ScriptStep::AdvanceTicks(30),
        ScriptStep::Exit,
    ]
}
