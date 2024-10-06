
// use chrono::Duration;
use chrono::Utc;
use tracing::debug;
use crate::models::{KPIResult, Analysis, KPIStatus};
use crate::kpi::utils::sanitize_version::sanitize_version;
use crate::kpi::utils::version_matches::version_matches;
use crate::kpi::utils::is_valid_timeframe::is_valid_timeframe;
use crate::kpi::find_upgrade_suggestions::find_upgrade_suggestions;

pub fn compute_kpi<'a>(analysis: &'a mut Analysis) -> Option<KPIResult> {
    let cycle = sanitize_version(&analysis.dependency_version.cycle);
    let today = Utc::now().date_naive();
    debug!("Analyzing KPI for {:?}", analysis.dependency_version);

    let (oldest_suggestion, latest_suggestion) = analysis.roadmap.as_mut()
        .map(|roadmap| find_upgrade_suggestions(&mut roadmap.entries, today))
        .unwrap_or((None, None));

    analysis.roadmap.as_ref().and_then(|roadmap| {
        roadmap.entries.iter().find(|entry| version_matches(&cycle, &entry.cycle))
    }).map_or_else(
        || {
            // No match case
            latest_suggestion.clone().map(|(latest, source_name)| {
                let reason = format!(
                    "No direct match. Version {} is outdated as of {}. Consider upgrading to {}.",
                    cycle, today, latest,
                );

                // Adding days calculation (no EOL, so days is 0 here)
                let days = 0;

                KPIResult {
                    product: analysis.dependency_version.product.clone(),
                    cycle: cycle.clone(),
                    status: KPIStatus::Outdated,
                    reason,
                    source: Some(source_name.clone()),  // Assign the source here
                    days: Some(days), // Set days to 0 since no specific EOL here
                }
            })
        },
        |matching_entry| {
            // Match case
            let timeframe_valid = is_valid_timeframe(&matching_entry.release_date, &matching_entry.eol, &matching_entry.extended_end_date, today);

            // Compute days difference for outdated cases
            let days = if let Some(eol_date) = matching_entry.eol {
                eol_date.signed_duration_since(today).num_days()
            } else {
                0 // Default to 0 if no EOL date is present
            };

            let reason = match (timeframe_valid, latest_suggestion.as_ref(), oldest_suggestion.as_ref()) {
                // Case: UpToDate
                (true, Some((latest, source_name)), _) if *latest == cycle => {
                    format!("Version {} is up to date as of {} (source: {}).", cycle, today, source_name)
                },
                // Case: Compliant with both latest and oldest suggestion
                (true, Some((latest, _)), Some((oldest, _))) => {
                    format!(
                        "Version {} is valid as of {}. Latest valid version is {}. Minimum valid version is {}.",
                        cycle, today, latest, oldest,
                    )
                },
                // Case: Compliant with only latest suggestion
                (true, Some((latest, _)), None) => {
                    format!(
                        "Version {} is valid as of {}. Latest valid version is {}.",
                        cycle, today, latest,
                    )
                },
                // Case: Outdated with suggestion
                (false, Some((latest, _)), _) => {
                    format!(
                        "Version {} is outdated as of {}. Consider upgrading to {}.",
                        cycle, matching_entry.eol.unwrap_or(today), latest,
                    )
                },
                // Case: Outdated without suggestion
                (false, None, _) => {
                    format!(
                        "Version {} is outdated as of {}. No upgrade suggestion available.",
                        cycle, matching_entry.eol.unwrap_or(today)
                    )
                },
                // Case: Compliant without any suggestion
                (true, None, _) => {
                    format!(
                        "Version {} is valid as of {}.",
                        cycle, today
                    )
                }
            };

            Some(KPIResult {
                product: analysis.dependency_version.product.clone(),
                cycle: cycle.clone(),
                status: if timeframe_valid && latest_suggestion.as_ref().map_or(false, |(latest, _)| latest == &cycle) {
                    KPIStatus::UpToDate
                } else if timeframe_valid {
                    KPIStatus::Compliant
                } else {
                    KPIStatus::Outdated
                },
                reason,
                source: latest_suggestion.as_ref().map(|(_, source_name)| source_name.clone()), // Set the source when available
                days: if !timeframe_valid { Some(days) } else { None }, // Set days only for outdated cases
            })
        }
    )
}

