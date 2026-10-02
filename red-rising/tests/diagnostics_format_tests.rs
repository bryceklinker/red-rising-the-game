use red_rising::diagnostics::format::format_diagnostic_line;

#[test]
fn when_value_is_present_then_line_shows_it_formatted() {
    assert_eq!(format_diagnostic_line("FPS", Some(59.9512)), "FPS: 59.95");
}

#[test]
fn when_value_is_absent_then_line_shows_not_available() {
    assert_eq!(format_diagnostic_line("CPU", None), "CPU: N/A");
}
