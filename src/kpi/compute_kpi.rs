
use chrono::Utc;
use tracing::{info, debug};
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
                KPIResult {
                    product: analysis.dependency_version.product.clone(),
                    cycle: cycle.clone(),
                    status: KPIStatus::Outdated,
                    reason,
                    source: Some(source_name.clone()),  // Assign the source here
                }
            })
        },
        |matching_entry| {
            // Match case
            let timeframe_valid = is_valid_timeframe(&matching_entry.release_date, &matching_entry.eol, &matching_entry.extended_end_date, today);
            let reason = match (timeframe_valid, latest_suggestion.as_ref(), oldest_suggestion.as_ref()) {
                // Case: UpToDate
                (true, Some((latest, source_name)), _) if *latest == cycle => {
                    format!("Version {} is up to date as of {} (source: {}).", cycle, today, source_name)
                },
                // Case: Compliant with both latest and oldest suggestion
                (true, Some((latest, source_name)), Some((oldest, source_name_oldest))) => {
                    format!(
                        "Version {} is valid as of {}. Latest valid version is {}. Minimum valid version is {} (source: {}).",
                        cycle, today, latest, source_name, oldest,
                    )
                },
                // Case: Compliant with only latest suggestion
                (true, Some((latest, source_name)), None) => {
                    format!(
                        "Version {} is valid as of {}. Latest valid version is {}.",
                        cycle, today, latest,
                    )
                },
                // Case: Outdated with suggestion
                (false, Some((latest, source_name)), _) => {
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
            })
        }
    )
}

