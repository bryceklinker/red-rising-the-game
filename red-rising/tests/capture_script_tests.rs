use red_rising::capture::script::{ScriptStep, ScriptedSession};
use red_rising::game_state::GameState;

#[test]
fn when_session_is_constructed_then_steps_pop_in_order() {
    let mut session = ScriptedSession::new(vec![
        ScriptStep::AdvanceTicks(3),
        ScriptStep::WaitForState(GameState::Drilling),
        ScriptStep::Capture("drilling"),
        ScriptStep::Exit,
    ]);

    assert_eq!(session.pop_front(), Some(ScriptStep::AdvanceTicks(3)));
    assert_eq!(
        session.pop_front(),
        Some(ScriptStep::WaitForState(GameState::Drilling))
    );
    assert_eq!(session.pop_front(), Some(ScriptStep::Capture("drilling")));
    assert_eq!(session.pop_front(), Some(ScriptStep::Exit));
    assert_eq!(session.pop_front(), None);
}
