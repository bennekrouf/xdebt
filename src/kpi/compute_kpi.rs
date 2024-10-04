
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

    if let Some(roadmap) = &mut analysis.roadmap {
        let (oldest_suggestion, latest_suggestion) = find_upgrade_suggestions(&mut roadmap.entries, today);

        if let Some(matching_entry) = roadmap.entries.iter().find(|entry| version_matches(&cycle, &entry.cycle)) {
            debug!("Version matches: {} with {}", cycle, &matching_entry.cycle);

            if is_valid_timeframe(&matching_entry.release_date, &matching_entry.eol, &matching_entry.extended_end_date, today) {
                let mut reason = format!("Version {} is valid as of {}.", cycle, today);

                // Append upgrade suggestions if present, with source name in parentheses
                if let Some((latest, source_name)) = latest_suggestion {
                    reason.push_str(&format!(" Latest valid version is {} (source: {})", latest, source_name));
                }
                if let Some((oldest, source_name)) = oldest_suggestion {
                    reason.push_str(&format!(" Minimum valid version is {} (source: {})", oldest, source_name));
                }

                return Some(KPIResult {
                    product: analysis.dependency_version.product.clone(),
                    cycle: cycle.clone(),
                    status: KPIStatus::Compliant,
                    reason,
                });
            } else {
                let reason = format!(
                    "Version {} is outdated as of {}. Consider upgrading to {}.",
                    cycle,
                    matching_entry.eol.unwrap_or(today).to_string(),
                    latest_suggestion.map(|(latest, source_name)| format!("{} (source: {})", latest, source_name)).unwrap_or("a higher cycle".to_string())
                );

                return Some(KPIResult {
                    product: analysis.dependency_version.product.clone(),
                    cycle: cycle.clone(),
                    status: KPIStatus::Outdated,
                    reason,
                });
            }
        }

        // Handle case where no direct match is found
        if let Some((latest, source_name)) = latest_suggestion {
            let reason = format!(
                "No direct match. Version {} is outdated as of {}. Consider upgrading to {} (source: {}).",
                cycle,
                today,
                latest,
                source_name
            );

            return Some(KPIResult {
                product: analysis.dependency_version.product.clone(),
                cycle: cycle.clone(),
                status: KPIStatus::Outdated,
                reason,
            });
        }
    }

    info!("Returning no suggestion for: {:?}", analysis.dependency_version.product);
    None
}

