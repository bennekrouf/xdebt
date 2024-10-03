
use chrono::NaiveDate;
use crate::models::RoadmapEntry;
use crate::kpi::utils::is_valid_timeframe::is_valid_timeframe;
use crate::kpi::is_lower_version::is_lower_version;
use crate::kpi::utils::compare_versions::compare_versions;

pub fn find_upgrade_suggestion(entries: &mut Vec<RoadmapEntry>, today: NaiveDate) -> Option<String> {
    // Ensure entries are sorted in ascending order (lower versions first)
    entries.sort_by(|a, b| compare_versions(&a.cycle, &b.cycle));

    let mut upgrade_suggestion: Option<&RoadmapEntry> = None;

    for entry in entries.iter().rev() { // Iterate from highest to lowest
        // Ensure that we only consider versions that have been released
        if let Some(start) = entry.release_date {
            if today < start {
                continue; // Skip versions not yet released
            }
        }

        // Check if the entry is within the valid timeframe
        if is_valid_timeframe(&entry.release_date, &entry.eol, &entry.extended_end_date, today) {
            // Always pick the highest version (latest entry)
            if upgrade_suggestion.is_none() || is_lower_version(&upgrade_suggestion.unwrap().cycle, &entry.cycle) {
                upgrade_suggestion = Some(entry);
            }
        }
    }

    upgrade_suggestion.map(|entry| entry.cycle.clone())
}

