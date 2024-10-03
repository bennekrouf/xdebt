

pub fn is_better_match(current_best: &str, new_version: &str) -> bool {
    let current_best_parts: Vec<&str> = current_best.split('.').collect();
    let new_version_parts: Vec<&str> = new_version.split('.').collect();

    for (best_part, new_part) in current_best_parts.iter().zip(new_version_parts.iter()) {
        if new_part == &"x" {
            continue; // Skip wildcard parts in new_version
        }
        if best_part < new_part {
            return true; // new_version is better if it's numerically greater
        }
        if best_part > new_part {
            return false; // current_best is still better if it's numerically greater
        }
    }

    // If we haven't found a difference yet, prefer the new version if it has more components
    new_version_parts.len() > current_best_parts.len()
}

