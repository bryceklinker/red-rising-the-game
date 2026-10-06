use red_rising::decision::outcome::{DecisionOption, EndState, end_state_for_choice};

#[test]
fn when_keep_drilling_is_chosen_then_end_state_reflects_it() {
    assert_eq!(
        end_state_for_choice(DecisionOption::KeepDrilling),
        EndState::KeptDrilling
    );
}

#[test]
fn when_get_out_and_check_is_chosen_then_end_state_reflects_it() {
    assert_eq!(
        end_state_for_choice(DecisionOption::GetOutAndCheck),
        EndState::GotOutAndChecked
    );
}

#[test]
fn when_wait_for_the_team_is_chosen_then_end_state_reflects_it() {
    assert_eq!(
        end_state_for_choice(DecisionOption::WaitForTheTeam),
        EndState::WaitedForTheTeam
    );
}
