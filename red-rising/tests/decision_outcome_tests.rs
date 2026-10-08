use red_rising::decision::outcome::{
    DecisionOption, DrillingOutcome, NarolReaction, drilling_outcome_for,
};

#[test]
fn when_keep_drilling_is_chosen_then_outcome_is_the_highest_yield_with_an_injury() {
    let outcome = drilling_outcome_for(DecisionOption::KeepDrilling);

    assert_eq!(
        outcome,
        DrillingOutcome {
            yield_kilos: 48,
            injured: true,
            narol_reaction: NarolReaction::AlarmedAndAngry,
        }
    );
}

#[test]
fn when_get_out_and_check_is_chosen_then_outcome_is_a_safe_moderate_yield() {
    let outcome = drilling_outcome_for(DecisionOption::GetOutAndCheck);

    assert_eq!(
        outcome,
        DrillingOutcome {
            yield_kilos: 32,
            injured: false,
            narol_reaction: NarolReaction::Relieved,
        }
    );
}

#[test]
fn when_wait_for_the_team_is_chosen_then_outcome_is_a_safe_lowest_yield() {
    let outcome = drilling_outcome_for(DecisionOption::WaitForTheTeam);

    assert_eq!(
        outcome,
        DrillingOutcome {
            yield_kilos: 20,
            injured: false,
            narol_reaction: NarolReaction::ApprovingButTired,
        }
    );
}
