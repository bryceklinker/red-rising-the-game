use red_rising::drilling::depth::call_trigger_state;

#[test]
fn when_depth_is_below_threshold_then_call_does_not_trigger() {
    assert!(!call_trigger_state(5.0, 10.0));
}

#[test]
fn when_depth_meets_threshold_then_call_triggers() {
    assert!(call_trigger_state(10.0, 10.0));
}

#[test]
fn when_depth_exceeds_threshold_then_call_triggers() {
    assert!(call_trigger_state(15.0, 10.0));
}
