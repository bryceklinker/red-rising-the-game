use red_rising::decision::outcome::EndState;
use red_rising::end::end_message_for;

#[test]
fn when_kept_drilling_then_message_reflects_it() {
    assert_eq!(
        end_message_for(EndState::KeptDrilling),
        "Darrow keeps drilling, gas pocket be damned."
    );
}

#[test]
fn when_got_out_and_checked_then_message_reflects_it() {
    assert_eq!(
        end_message_for(EndState::GotOutAndChecked),
        "Darrow pulls the team out to check the reading."
    );
}

#[test]
fn when_waited_for_the_team_then_message_reflects_it() {
    assert_eq!(
        end_message_for(EndState::WaitedForTheTeam),
        "Darrow waits for the team before going further."
    );
}
