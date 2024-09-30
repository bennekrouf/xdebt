
use chrono::{NaiveDate, Utc};
use crate::models::{KPIResult, Analysis, KPIStatus, RoadmapEntry};

pub fn compute_kpi<'a>(analysis: &'a Analysis) -> Option<KPIResult> {
    let current_version = &analysis.dependency_version.current_version;
    let today = Utc::now().date_naive();

    if let Some(roadmap) = &analysis.roadmap {
        let mut upgrade_suggestion: Option<&RoadmapEntry> = None;

        for entry in &roadmap.entries {
            // Check if current version matches the roadmap entry's version pattern
            if version_matches(current_version, &entry.version) {
                // Check if the current version is within valid timeframe
                if is_valid_timeframe(&entry.start_date, &entry.end_date, &entry.extended_end_date, today) {
                    let valid_until = entry.end_date.unwrap_or(today);  // Use the end_date or today if null
                    return Some(KPIResult {
                        dependency_name: analysis.dependency_version.dependency_name.clone(),
                        current_version: current_version.clone(),
                        status: KPIStatus::Compliant(format!(
                            "Version {} is valid as of {}. Valid until {}.",
                            current_version, today, valid_until
                        )),
                    });
                } else {
                    // If end_date has passed, version is outdated
                    return Some(KPIResult {
                        dependency_name: analysis.dependency_version.dependency_name.clone(),
                        current_version: current_version.clone(),
                        status: KPIStatus::UpgradeNeeded(format!(
                            "Version {} is outdated as of {}. Consider upgrading to {}.",
                            current_version,
                            entry.end_date.unwrap_or(today).to_string(),
                            find_upgrade_suggestion(&roadmap.entries, today).unwrap_or("a higher version".to_string())
                        )),
                    });
                }
            }

            // Suggest upgrade if current version is outdated and this entry is newer
            if is_lower_version(current_version, &entry.version) {
                if upgrade_suggestion.is_none() || is_better_match(&upgrade_suggestion.unwrap().version, &entry.version) {
                    upgrade_suggestion = Some(entry);
                }
            }
        }

        // Handle the case where no direct match is found
        if let Some(upgrade_suggestion) = upgrade_suggestion {
            return Some(KPIResult {
                dependency_name: analysis.dependency_version.dependency_name.clone(),
                current_version: current_version.clone(),
                status: KPIStatus::UpgradeNeeded(format!(
                    "Version {} is outdated as of {}. Consider upgrading to {}.",
                    current_version,
                    today,
                    upgrade_suggestion.version
                )),
            });
        }
    }

    None
}

// Helper function to find the next available version to upgrade to
fn find_upgrade_suggestion(entries: &Vec<RoadmapEntry>, today: NaiveDate) -> Option<String> {
    let mut upgrade_suggestion: Option<&RoadmapEntry> = None;

    for entry in entries {
        // If the version starts in the future, suggest it for upgrade
        if let Some(start) = entry.start_date {
            if today < start {
                if upgrade_suggestion.is_none() || is_lower_version(&entry.version, &upgrade_suggestion.unwrap().version) {
                    upgrade_suggestion = Some(entry);
                }
            }
        }

        // Check if the version is within the valid timeframe
        if is_valid_timeframe(&entry.start_date, &entry.end_date, &entry.extended_end_date, today) {
            if upgrade_suggestion.is_none() || is_lower_version(&entry.version, &upgrade_suggestion.unwrap().version) {
                upgrade_suggestion = Some(entry);
            }
        }
    }

    upgrade_suggestion.map(|entry| entry.version.clone())
}

// Check if versions match (support wildcards like 'x')

fn version_matches(current_version: &str, roadmap_version: &str) -> bool {
    // Simplistic matching for wildcard versions like "1.x"
    if roadmap_version.ends_with(".x") {
        let base_version = roadmap_version.trim_end_matches(".x");
        return current_version.starts_with(base_version);
    }
    current_version == roadmap_version
}


// Check if timeframe is valid for the current date
fn is_valid_timeframe(start_date: &Option<NaiveDate>, end_date: &Option<NaiveDate>, extended_end_date: &Option<NaiveDate>, today: NaiveDate) -> bool {
    if let Some(start) = start_date {
        if today < *start {
            return false; // Today is before the start date
        }
    }

    if let Some(end) = end_date {
        if today > *end {
            // Check extended end date if provided
            if let Some(extended_end) = extended_end_date {
                if today > *extended_end {
                    return false; // Today is after extended end date
                }
            } else {
                return false; // Today is after end date and no extended date
            }
        }
    }

    true // The version is valid within the time range
}

// Compare versions to check if the roadmap version is lower than the current version
fn is_lower_version(current_version: &str, roadmap_version: &str) -> bool {
    let current_parts: Vec<&str> = current_version.split('.').collect();
    let roadmap_parts: Vec<&str> = roadmap_version.split('.').collect();

    for (current_part, roadmap_part) in current_parts.iter().zip(roadmap_parts.iter()) {
        if roadmap_part == &"x" {
            continue; // Wildcard match, consider it compatible
        }
        if current_part < roadmap_part {
            return false; // If roadmap part is greater, it's not lower
        }
        if current_part > roadmap_part {
            return true; // If current part is greater, it's lower
        }
    }

    false // If we get here, the versions are equal
}

// Compare two versions and return true if the new version is a better retro-compatible match
fn is_better_match(current_best: &str, new_version: &str) -> bool {
    let current_best_parts: Vec<&str> = current_best.split('.').collect();
    let new_version_parts: Vec<&str> = new_version.split('.').collect();

    for (best_part, new_part) in current_best_parts.iter().zip(new_version_parts.iter()) {
        if new_part == &"x" {
            continue; // Wildcard match
        }
        if best_part < new_part {
            return true; // New version is closer to current
        }
        if best_part > new_part {
            return false; // Current best is closer
        }
    }

    false // Versions are equal
}

