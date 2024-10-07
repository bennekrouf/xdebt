
use chrono::Utc;
use tracing::debug;
use crate::models::RoadmapEntry;
use crate::kpi::utils::is_valid_timeframe::is_valid_timeframe;
use crate::kpi::is_lower_version::is_lower_version;
use crate::kpi::utils::compare_versions::compare_versions;
use crate::models::AppConfig;

pub fn find_upgrade_suggestions(
    config: &AppConfig,
    entries: &mut Vec<RoadmapEntry>,
) -> (Option<(String, String)>, Option<(String, String)>) {
    let today = Utc::now().date_naive();
    debug!("Starting to find upgrade suggestions for today's date: {}", today);

    if let Some(sources_priorities) = &config.sources_priorities {
        // Custom sort by both sources_priorities and version cycle, making source_name comparisons case-insensitive
        entries.sort_by(|a, b| {
            let source_a = a.source_name.clone().unwrap_or_default().to_lowercase();
            let source_b = b.source_name.clone().unwrap_or_default().to_lowercase();

            let priority_a = sources_priorities.iter().position(|p| p.to_lowercase() == source_a).unwrap_or(sources_priorities.len());
            let priority_b = sources_priorities.iter().position(|p| p.to_lowercase() == source_b).unwrap_or(sources_priorities.len());


            if priority_a == priority_b {
                // If priorities are equal, fallback to comparing versions
                compare_versions(&a.cycle, &b.cycle)
            } else {
                // Compare based on priority
                priority_a.cmp(&priority_b)
            }
        });
    } else {
        // Fallback: sort only by version cycle if no priorities are provided
        entries.sort_by(|a, b| compare_versions(&a.cycle, &b.cycle));
    }

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

            // Update latest suggestion, taking priority into account (if available)
            if latest_suggestion.is_none() ||
                is_lower_version(&latest_suggestion.unwrap().cycle, &entry.cycle) ||
                config.sources_priorities.as_ref()
                    .map(|priorities| priorities.iter().position(|p| p.to_lowercase() == entry.source_name.clone().unwrap_or_default().to_lowercase())
                        < priorities.iter().position(|p| p.to_lowercase() == latest_suggestion.unwrap().source_name.clone().unwrap_or_default().to_lowercase()))
                    .unwrap_or(false) {
                debug!("Setting latest upgrade suggestion to {}", entry.cycle);
                latest_suggestion = Some(entry);
            }

            // Update oldest suggestion, taking priority into account (if available)
            if oldest_suggestion.is_none() ||
                is_lower_version(&entry.cycle, &oldest_suggestion.unwrap().cycle) ||
                config.sources_priorities.as_ref()
                    .map(|priorities| priorities.iter().position(|p| p.to_lowercase() == entry.source_name.clone().unwrap_or_default().to_lowercase())
                        < priorities.iter().position(|p| p.to_lowercase() == oldest_suggestion.unwrap().source_name.clone().unwrap_or_default().to_lowercase()))
                    .unwrap_or(false) {
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

