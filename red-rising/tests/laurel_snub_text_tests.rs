use red_rising::decision::outcome::{DrillingOutcome, NarolReaction};
use red_rising::laurel_snub::laurel_snub_text_for;

#[test]
fn when_keep_drilling_outcome_then_text_mentions_the_burn_and_narols_anger() {
    let text = laurel_snub_text_for(DrillingOutcome {
        yield_kilos: 48,
        injured: true,
        narol_reaction: NarolReaction::AlarmedAndAngry,
    });

    assert_eq!(
        text,
        "Lambda pulls 48 kilos, Darrow's arm raw from the burn -- and the board still skips them for the Laurel. Narol is furious they risked it for nothing."
    );
}

#[test]
fn when_get_out_and_check_outcome_then_text_is_safe_and_narol_is_relieved() {
    let text = laurel_snub_text_for(DrillingOutcome {
        yield_kilos: 32,
        injured: false,
        narol_reaction: NarolReaction::Relieved,
    });

    assert_eq!(
        text,
        "Lambda pulls 32 kilos, clean and safe -- and the board still skips them for the Laurel. Narol is relieved they stopped to check, for all the good it did."
    );
}

#[test]
fn when_wait_for_the_team_outcome_then_text_is_safe_and_narol_is_approving_but_tired() {
    let text = laurel_snub_text_for(DrillingOutcome {
        yield_kilos: 20,
        injured: false,
        narol_reaction: NarolReaction::ApprovingButTired,
    });

    assert_eq!(
        text,
        "Lambda pulls 20 kilos, clean and safe -- and the board still skips them for the Laurel. Narol nods, approving but worn thin."
    );
}
