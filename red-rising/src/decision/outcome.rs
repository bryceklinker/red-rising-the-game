#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DecisionOption {
    KeepDrilling,
    GetOutAndCheck,
    WaitForTheTeam,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EndState {
    KeptDrilling,
    GotOutAndChecked,
    WaitedForTheTeam,
}

pub fn end_state_for_choice(choice: DecisionOption) -> EndState {
    match choice {
        DecisionOption::KeepDrilling => EndState::KeptDrilling,
        DecisionOption::GetOutAndCheck => EndState::GotOutAndChecked,
        DecisionOption::WaitForTheTeam => EndState::WaitedForTheTeam,
    }
}
