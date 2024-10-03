
use chrono::{NaiveDate, Utc};
use tracing::{info, debug};
use crate::models::{KPIResult, Analysis, KPIStatus, RoadmapEntry};

pub fn compute_kpi<'a>(analysis: &'a mut Analysis) -> Option<KPIResult> {
    let cycle = sanitize_version(&analysis.dependency_version.cycle);
    let today = Utc::now().date_naive();
    debug!("Analyzing KPI for {:?}", analysis.dependency_version);

    if let Some(roadmap) = &mut analysis.roadmap {
        let mut upgrade_suggestion: Option<&RoadmapEntry> = None;

        for entry in &roadmap.entries {
            debug!("Analyzing roadmap entry: {:?}", entry);

            // Check if current version matches the roadmap entry's version pattern
            if version_matches(&cycle, &entry.cycle) {
                debug!("Version matches: {} with {}", cycle, &entry.cycle);

                // Check if the current version is within valid timeframe
                if is_valid_timeframe(&entry.release_date, &entry.eol, &entry.extended_end_date, today) {
                    let valid_until = entry.eol.unwrap_or(today);  // Use eol or today if null
                    return Some(KPIResult {
                        product: analysis.dependency_version.product.clone(),
                        cycle: cycle.clone(),
                        status: KPIStatus::Compliant,  // Status without reason in enum
                        reason: format!(
                            "Version {} is valid as of {}. Valid until {}.",
                            cycle, today, valid_until
                        ),
                    });
                } else {
                    // If eol has passed, version is outdated
                    return Some(KPIResult {
                        product: analysis.dependency_version.product.clone(),
                        cycle: cycle.clone(),
                        status: KPIStatus::UpgradeNeeded,  // Status without reason in enum
                        reason: format!(
                            "Version {} is outdated as of {}. Consider upgrading to {}.",
                            cycle,
                            entry.eol.unwrap_or(today).to_string(),
                            find_upgrade_suggestion(&mut roadmap.entries, today).unwrap_or("a higher cycle".to_string())
                        ),
                    });
                }
            } else {
                debug!("No version match: {} and {} of {:?}", cycle, &entry.cycle, &entry);
            }

            // Suggest upgrade if current version is outdated and this entry is newer
            debug!("Checking if lower version: {} and {}", cycle, &entry.cycle);
            if is_lower_version(&cycle, &entry.cycle) {
                if upgrade_suggestion.is_none() || is_better_match(&upgrade_suggestion.unwrap().cycle, &entry.cycle) {
                    upgrade_suggestion = Some(entry);
                }
            }
        }

        // Handle case where no direct match is found
        if let Some(upgrade_suggestion) = upgrade_suggestion {
            return Some(KPIResult {
                product: analysis.dependency_version.product.clone(),
                cycle: cycle.clone(),
                status: KPIStatus::UpgradeNeeded,  // Status without reason in enum
                reason: format!(
                    "No direct match. Version {} is outdated as of {}. Consider upgrading to {}.",
                    cycle,
                    today,
                    upgrade_suggestion.cycle
                ),
            });
        }
    }

    info!("Returning no suggestion for: {:?}", analysis.dependency_version.product);
    None
}

// Helper function to find the next available version to upgrade to
fn find_upgrade_suggestion(entries: &mut Vec<RoadmapEntry>, today: NaiveDate) -> Option<String> {
    // Sort the entries by the cycle in ascending order
    entries.sort_by(|a, b| compare_versions(&a.cycle, &b.cycle));

    let mut upgrade_suggestion: Option<&RoadmapEntry> = None;

    for entry in entries {
        // If the version starts in the future, suggest it for upgrade
        if let Some(start) = entry.release_date {
            if today < start {
                // Suggest the lowest version that is higher than the current
                if upgrade_suggestion.is_none() || is_lower_version(&upgrade_suggestion.unwrap().cycle, &entry.cycle) {
                    upgrade_suggestion = Some(entry);
                }
            }
        }

        // Check if the version is within the valid timeframe
        if is_valid_timeframe(&entry.release_date, &entry.eol, &entry.extended_end_date, today) {
            // If this version is higher than the current suggestion, use it
            if upgrade_suggestion.is_none() || is_lower_version(&upgrade_suggestion.unwrap().cycle, &entry.cycle) {
                upgrade_suggestion = Some(entry);
            }
        }
    }

    // Return the cycle of the highest valid version found
    upgrade_suggestion.map(|entry| entry.cycle.clone())
}


fn version_matches(cycle: &str, roadmap_version: &str) -> bool {
    if roadmap_version.ends_with(".x") {
        let base_version = roadmap_version.trim_end_matches(".x");
        return cycle.starts_with(base_version);
    }

    // Allow matching major.minor versions (like 5.3 matches 5.3.31)
    let roadmap_parts: Vec<&str> = roadmap_version.split('.').collect();
    let cycle_parts: Vec<&str> = cycle.split('.').collect();

    // Check if the roadmap major.minor matches cycle major.minor
    roadmap_parts.iter().zip(cycle_parts.iter()).all(|(rp, cp)| rp == cp)
}

fn is_valid_timeframe(release_date: &Option<NaiveDate>, eol: &Option<NaiveDate>, extended_end_date: &Option<NaiveDate>, today: NaiveDate) -> bool {
    if let Some(start) = release_date {
        if today < *start {
            return false; // Today is before the start date
        }
    }

    if let Some(end) = eol {
        if today > *end {
            // Check extended end date if provided
            if let Some(extended_end) = extended_end_date {
                return today <= *extended_end;  // Valid if today is before the extended date
            } else {
                return false; // Today is after end date and no extended date
            }
        }
    }

    true // The version is valid within the time range
}



fn compare_versions(v1: &str, v2: &str) -> std::cmp::Ordering {
    let parts1 = v1.split('.').collect::<Vec<&str>>();
    let parts2 = v2.split('.').collect::<Vec<&str>>();

    for (p1, p2) in parts1.iter().zip(parts2.iter()) {
        if *p1 == "x" || *p2 == "x" {
            continue; // Skip comparison if either part is a wildcard
        }

        let num1 = p1.parse::<u32>().unwrap_or(0);
        let num2 = p2.parse::<u32>().unwrap_or(0);

        match num1.cmp(&num2) {
            std::cmp::Ordering::Equal => continue, // Keep comparing the next parts
            non_eq => return non_eq, // Return non-equal result
        }
    }

    // If all compared parts are equal but one version has more components, handle that
    parts1.len().cmp(&parts2.len())
}

fn is_lower_version(v1: &str, v2: &str) -> bool {
    compare_versions(v1, v2) == std::cmp::Ordering::Less
}


// Compare versions to check if the roadmap version is lower than the current version
// fn is_lower_version(cycle: &str, roadmap_version: &str) -> bool {
//     let current_parts: Vec<&str> = cycle.split('.').collect();
//     let roadmap_parts: Vec<&str> = roadmap_version.split('.').collect();
//
//     for (current_part, roadmap_part) in current_parts.iter().zip(roadmap_parts.iter()) {
//         if roadmap_part == &"x" {
//             continue; // Wildcard match, consider it compatible
//         }
//         if current_part < roadmap_part {
//             return false; // If roadmap part is greater, it's not lower
//         }
//         if current_part > roadmap_part {
//             return true; // If current part is greater, it's lower
//         }
//     }
//
//     false // If we get here, the versions are equal
// }

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

// Function to sanitize version strings by removing unwanted characters
fn sanitize_version(cycle: &str) -> String {
    cycle
        .replace("'", "") // Remove single quotes
        .replace(",", "")  // Remove commas if needed
        .chars()           // Iterate over characters
        .filter(|c| c.is_numeric() || *c == '.') // Keep only numeric and dot characters
        .collect::<String>()         // Collect the sanitized characters back into a string
        .trim()            // Trim whitespace if any
        .to_string()       // Convert to String}
}
