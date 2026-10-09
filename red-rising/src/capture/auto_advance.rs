use crate::capture::script::{ScriptStep, ScriptedSession};
use crate::decision::outcome::{DecisionOption, end_state_for_choice};
use crate::decision::ui::ChosenEndState;
use crate::game_state::GameState;
use bevy::prelude::{NextState, Res, ResMut, State};

/// Drives `GameState` transitions directly from the scripted session instead of
/// faking a UI button `Interaction`: capture mode renders to an off-screen
/// `RenderTarget::Image`, and bevy_ui's `ui_focus_system` only tracks cursor
/// `Interaction` for cameras targeting a real `Window` -- it unconditionally
/// resets `Interaction` back to `None` every frame for an off-screen camera,
/// so a faked click is clobbered before any button-handler system observes it.
pub fn auto_advance_game_state(
    session: Res<ScriptedSession>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut chosen_end_state: ResMut<ChosenEndState>,
) {
    let current = *current_state.get();
    if !script_has_moved_past(&session, current) {
        return;
    }

    let Some(next) = next_game_state(current) else {
        return;
    };

    if current == GameState::Decision {
        chosen_end_state.0 = Some(end_state_for_choice(DecisionOption::KeepDrilling));
    }
    next_state.set(next);
}

fn next_game_state(current: GameState) -> Option<GameState> {
    match current {
        GameState::CharacterSelect => Some(GameState::Drilling),
        GameState::CallEvent => Some(GameState::Decision),
        GameState::Decision => Some(GameState::End),
        GameState::Drilling | GameState::End => None,
    }
}

fn script_has_moved_past(session: &ScriptedSession, current_state: GameState) -> bool {
    match session.front() {
        Some(ScriptStep::WaitForState(next)) => *next != current_state,
        Some(ScriptStep::Exit) => true,
        _ => false,
    }
}
