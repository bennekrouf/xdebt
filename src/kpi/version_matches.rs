

pub fn version_matches(cycle: &str, roadmap_version: &str) -> bool {
    if roadmap_version.ends_with(".x") {
        let base_version = roadmap_version.trim_end_matches(".x");
        return cycle.starts_with(base_version);
    }

    let roadmap_parts: Vec<&str> = roadmap_version.split('.').collect();
    let cycle_parts: Vec<&str> = cycle.split('.').collect();

    roadmap_parts.iter().zip(cycle_parts.iter()).all(|(rp, cp)| rp == cp)
}
