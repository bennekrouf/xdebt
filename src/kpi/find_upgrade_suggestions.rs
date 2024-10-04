
use chrono::NaiveDate;
use tracing::debug;
use crate::models::RoadmapEntry;
use crate::kpi::utils::is_valid_timeframe::is_valid_timeframe;
use crate::kpi::is_lower_version::is_lower_version;
use crate::kpi::utils::compare_versions::compare_versions;

pub fn find_upgrade_suggestions(entries: &mut Vec<RoadmapEntry>, today: NaiveDate) -> (Option<(String, String)>, Option<(String, String)>) {
    debug!("Starting to find upgrade suggestions for today's date: {}", today);

    entries.sort_by(|a, b| compare_versions(&a.cycle, &b.cycle));
    debug!("Entries sorted for comparison: {:?}", entries);

    let mut oldest_suggestion: Option<&RoadmapEntry> = None;
    let mut latest_suggestion: Option<&RoadmapEntry> = None;

    for entry in entries.iter().rev() {
        debug!("Analyzing roadmap entry: {:?}", entry);

        if let Some(start) = entry.release_date {
            if today < start {
                debug!("Skipping version {} as it is not released yet (release date: {})", entry.cycle, start);
                continue;
            }
        }

        if is_valid_timeframe(&entry.release_date, &entry.eol, &entry.extended_end_date, today) {
            debug!("Version {} is valid as of {}", entry.cycle, today);

            // Update latest suggestion
            if latest_suggestion.is_none() || is_lower_version(&latest_suggestion.unwrap().cycle, &entry.cycle) {
                debug!("Setting latest upgrade suggestion to {}", entry.cycle);
                latest_suggestion = Some(entry);
            }

            // Update oldest suggestion
            if oldest_suggestion.is_none() || is_lower_version(&entry.cycle, &oldest_suggestion.unwrap().cycle) {
                debug!("Setting oldest upgrade suggestion to {}", entry.cycle);
                oldest_suggestion = Some(entry);
            }
        } else {
            debug!("Version {} is not within valid timeframe (EOL: {:?})", entry.cycle, entry.eol);
        }
    }

    // Return the cycle and source_name for both the oldest and latest suggestions
    (
        oldest_suggestion.map(|e| (e.cycle.clone(), e.source_name.clone().unwrap_or_else(|| "?".to_string()))),
        latest_suggestion.map(|e| (e.cycle.clone(), e.source_name.clone().unwrap_or_else(|| "?".to_string())))
    )
}

