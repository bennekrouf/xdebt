
pub fn sanitize_version(cycle: &str) -> String {
    cycle
        .replace("'", "")
        .replace(",", "")
        .chars()
        .filter(|c| c.is_numeric() || *c == '.')
        .collect::<String>()
        .trim()
        .to_string()
}
