
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

                let days = 0; // No EOL, set days to 0 in no match case

                KPIResult {
                    product: analysis.dependency_version.product.clone(),
                    cycle: cycle.clone(),
                    status: KPIStatus::Outdated,
                    reason,
                    source: Some(source_name.clone()),
                    validity: Some(days.to_string()),
                }
            })
        },
        |matching_entry| {
            let timeframe_valid = is_valid_timeframe(
                &matching_entry.release_date, 
                &matching_entry.eol, 
                &matching_entry.extended_end_date, 
                today
            );

            // Calculate the number of days difference until the `eol` or `extended_end_date`
            let valid_until = matching_entry.eol.or(matching_entry.extended_end_date).unwrap_or(today);
            let days = valid_until.signed_duration_since(today).num_days().max(0); // Positive number of days

            let reason = match (timeframe_valid, latest_suggestion.as_ref(), oldest_suggestion.as_ref()) {
                // Case: UpToDate - no newer versions available
                (true, None, _) => {
                    debug!("Version {} is up to date until {} (source: roadmap).", cycle, valid_until);
                    format!("Version {} is valid until {}.", cycle, valid_until)
                },
                // Case: Compliant with both latest and oldest suggestion
                (true, Some((latest, _)), Some((oldest, _))) => {
                    format!(
                        "Version {} is valid until {}. Latest valid version is {}. Minimum valid version is {}.",
                        cycle, valid_until, latest, oldest,
                    )
                },
                // Case: Compliant with only latest suggestion
                (true, Some((latest, _)), None) => {
                    format!(
                        "Version {} is valid until {}. Latest valid version is {}.",
                        cycle, valid_until, latest,
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
                }
            };

            Some(KPIResult {
                product: analysis.dependency_version.product.clone(),
                cycle: cycle.clone(),
                status: if timeframe_valid && latest_suggestion.is_none() {
                    // Return UpToDate if the version is valid and there are no newer versions
                    KPIStatus::UpToDate
                } else if timeframe_valid {
                    KPIStatus::Compliant
                } else {
                    KPIStatus::Outdated
                },
                reason,
                source: latest_suggestion.as_ref().map(|(_, source_name)| source_name.clone()), // Set the source if available
                validity: Some(days.to_string()), // Always set `days` for valid and outdated cases
            })
        }
    )
}

