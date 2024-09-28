
use chrono::Utc;
use crate::kpi::compare_versions::compare_versions;
use crate::models::{KPIResult, Analysis, KPIStatus};
use std::cmp::Ordering;

pub fn compute_kpi<'a>(analysis: &'a Analysis) -> Option<KPIResult> {
    let current_version = &analysis.dependency_version.current_version;
    let today = Utc::now().date_naive();

    if let Some(roadmap) = analysis.roadmap.as_ref() {
        let mut sorted_entries = roadmap.entries.clone();
        sorted_entries.sort_by(|a, b| {
            let cmp = compare_versions(&b.version, &a.version);
            if cmp {
                b.start_date.cmp(&a.start_date)
            } else {
                Ordering::Equal
            }
        });

        let current_version_in_roadmap = sorted_entries.iter().any(|record| record.version == *current_version);

        // Check for the latest version available
        let latest_version_needed = match sorted_entries.first() {
            Some(record) if compare_versions(&record.version, current_version) => Some(record.version.clone()),
            _ => None,
        };

        // Using `match` to determine the status based on the roadmap and version
        let status = match (current_version_in_roadmap, latest_version_needed) {
            // Case: Current version is in the roadmap
            (true, _) => {
                // Find the first compliant record and match its end date
                match sorted_entries.iter().find(|record| compare_versions(current_version, &record.version)) {
                    Some(record) => match record.end_date {
                        Some(end_date) if end_date <= today => {
                            KPIStatus::UpgradeNeeded(format!("Upgrade needed by {} for version {}", end_date, record.version))
                        },
                        _ => KPIStatus::NoActionNeeded(format!("No action needed for version {}", record.version)),
                    },
                    None => KPIStatus::Compliant("Version is compliant.".to_string()),
                }
            },

            // Case: Current version is not in the roadmap but a newer version exists
            (false, Some(latest_version)) => {
                KPIStatus::UpgradeNeeded(format!("Upgrade needed to version {} (latest version)", latest_version))
            },

            // Case: Current version is not in the roadmap and no newer version found
            (false, None) => match sorted_entries.first() {
                Some(record) => KPIStatus::NonCompliant(format!("Upgrade needed to version {} (latest version)", record.version)),
                None => KPIStatus::NonCompliant("No valid roadmap entry found.".to_string()),
            },
        };

        // Return the KPIResult or None if no action is needed
        match status {
            KPIStatus::NoActionNeeded(_) => None,
            _ => Some(KPIResult {
                dependency_name: roadmap.dependency.clone(),
                current_version: current_version.clone(),
                status,
            }),
        }
    } else {
        panic!("No roadmap available");
    }
}

