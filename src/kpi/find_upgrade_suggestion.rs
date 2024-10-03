
use chrono::NaiveDate;
use crate::models::RoadmapEntry;
use crate::kpi::is_valid_timeframe::is_valid_timeframe;
use crate::kpi::is_lower_version::is_lower_version;
use crate::kpi::compare_versions::compare_versions;

pub fn find_upgrade_suggestion(entries: &mut Vec<RoadmapEntry>, today: NaiveDate) -> Option<String> {
    entries.sort_by(|a, b| compare_versions(&a.cycle, &b.cycle));

    let mut upgrade_suggestion: Option<&RoadmapEntry> = None;

    for entry in entries {
        if let Some(start) = entry.release_date {
            if today < start {
                if upgrade_suggestion.is_none() || is_lower_version(&upgrade_suggestion.unwrap().cycle, &entry.cycle) {
                    upgrade_suggestion = Some(entry);
                }
            }
        }

        if is_valid_timeframe(&entry.release_date, &entry.eol, &entry.extended_end_date, today) {
            if upgrade_suggestion.is_none() || is_lower_version(&upgrade_suggestion.unwrap().cycle, &entry.cycle) {
                upgrade_suggestion = Some(entry);
            }
        }
    }

    upgrade_suggestion.map(|entry| entry.cycle.clone())
}
