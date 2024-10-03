
use chrono::Utc;
use tracing::{info, debug};

use crate::models::{KPIResult, Analysis, KPIStatus, RoadmapEntry};
use crate::kpi::utils::sanitize_version::sanitize_version;
use crate::kpi::utils::version_matches::version_matches;
use crate::kpi::utils::is_valid_timeframe::is_valid_timeframe;
use crate::kpi::find_upgrade_suggestion::find_upgrade_suggestion;
use crate::kpi::is_lower_version::is_lower_version;
use crate::kpi::utils::is_better_match::is_better_match;


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
                    let mut reason = if let Some(valid_until) = entry.eol {
                        format!("Valid until {}.", valid_until)
                    } else {
                        format!("Version {} is valid as of {}.", cycle, today)
                    };

                    // If there is an upgrade suggestion, append it to the reason
                    if let Some(upgrade_entry) = upgrade_suggestion {
                        reason.push_str(&format!(
                            " Consider upgrading to {} for better support.",
                            upgrade_entry.cycle
                        ));
                    }

                    return Some(KPIResult {
                        product: analysis.dependency_version.product.clone(),
                        cycle: cycle.clone(),
                        status: KPIStatus::Compliant,
                        reason,
                    });
                } else {
                    // If EOL has passed, version is outdated
                    return Some(KPIResult {
                        product: analysis.dependency_version.product.clone(),
                        cycle: cycle.clone(),
                        status: KPIStatus::Outdated,
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
                status: KPIStatus::Outdated,
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

