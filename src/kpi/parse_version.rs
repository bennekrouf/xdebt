pub fn parse_version(version: &str) -> (u32, Option<u32>, Option<u32>) {
    let parts: Vec<&str> = version.split('.').collect();
    let major = parts[0].parse::<u32>().unwrap_or(0);
    let minor = if parts.len() > 1 {
        parts[1].parse::<u32>().ok()
    } else {
        None
    };
    let patch = if parts.len() > 2 {
        parts[2].parse::<u32>().ok()
    } else {
        None
    };
    (major, minor, patch)
}
