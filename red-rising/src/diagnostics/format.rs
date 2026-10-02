pub fn format_diagnostic_line(label: &str, value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{label}: {value:.2}"),
        None => format!("{label}: N/A"),
    }
}
