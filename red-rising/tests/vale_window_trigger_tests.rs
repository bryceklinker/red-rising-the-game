use red_rising::vale::window_trigger_state;

#[test]
fn when_distance_is_below_threshold_then_window_is_not_reached() {
    assert!(!window_trigger_state(5.0, 10.0));
}

#[test]
fn when_distance_meets_threshold_then_window_is_reached() {
    assert!(window_trigger_state(10.0, 10.0));
}

#[test]
fn when_distance_exceeds_threshold_then_window_is_reached() {
    assert!(window_trigger_state(15.0, 10.0));
}
