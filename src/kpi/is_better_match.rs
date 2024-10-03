
pub fn is_better_match(current_best: &str, new_version: &str) -> bool {
    let current_best_parts: Vec<&str> = current_best.split('.').collect();
    let new_version_parts: Vec<&str> = new_version.split('.').collect();

    for (best_part, new_part) in current_best_parts.iter().zip(new_version_parts.iter()) {
        if new_part == &"x" {
            continue;
        }
        if best_part < new_part {
            return true;
        }
        if best_part > new_part {
            return false;
        }
    }

    false
}
