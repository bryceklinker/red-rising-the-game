#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DecisionOption {
    KeepDrilling,
    GetOutAndCheck,
    WaitForTheTeam,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NarolReaction {
    AlarmedAndAngry,
    Relieved,
    ApprovingButTired,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DrillingOutcome {
    pub yield_kilos: u32,
    pub injured: bool,
    pub narol_reaction: NarolReaction,
}

pub fn drilling_outcome_for(choice: DecisionOption) -> DrillingOutcome {
    match choice {
        DecisionOption::KeepDrilling => DrillingOutcome {
            yield_kilos: 48,
            injured: true,
            narol_reaction: NarolReaction::AlarmedAndAngry,
        },
        DecisionOption::GetOutAndCheck => DrillingOutcome {
            yield_kilos: 32,
            injured: false,
            narol_reaction: NarolReaction::Relieved,
        },
        DecisionOption::WaitForTheTeam => DrillingOutcome {
            yield_kilos: 20,
            injured: false,
            narol_reaction: NarolReaction::ApprovingButTired,
        },
    }
}
