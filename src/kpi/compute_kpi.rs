use crate::kpi::compare_versions::compare_versions;
use crate::models::{Analysis, KPIResult, KPIStatus};
use chrono::Utc;
use std::cmp::Ordering;

pub fn compute_kpi<'a>(analysis: &'a Analysis) -> Option<KPIResult> {
    let current_version = &analysis.dependency_version.current_version;
    let today = Utc::now().date_naive();

    let mut kpi_status: KPIStatus = KPIStatus::NonCompliant("No roadmap available".to_string());

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

        let current_version_in_roadmap = sorted_entries
            .iter()
            .any(|record| record.version == *current_version);

        if current_version_in_roadmap {
            for record in &sorted_entries {
                let record_version = &record.version;

                if compare_versions(current_version, record_version) {
                    _ = KPIStatus::Compliant("Version is compliant.".to_string());

                    if let Some(end_date) = record.end_date {
                        kpi_status = if end_date <= today {
                            KPIStatus::UpgradeNeeded(format!("Upgrade needed by {}", end_date))
                        } else {
                            KPIStatus::NoActionNeeded("No action needed.".to_string())
                        };
                    } else {
                        kpi_status = KPIStatus::NoActionNeeded(
                            "No action needed (latest version).".to_string(),
                        );
                    }

                    break; // Exit the loop after finding the compliant version
                }
            }
        }

        if !current_version_in_roadmap {
            if let Some(record) = sorted_entries.first() {
                kpi_status = KPIStatus::NonCompliant(format!(
                    "Upgrade needed to version {} (latest version)",
                    record.version
                ));
                if let Some(end_date) = record.end_date {
                    kpi_status = KPIStatus::UpgradeNeeded(format!(
                        "Upgrade needed to version {} (latest version) by {}",
                        record.version, end_date
                    ));
                }
            }
        }

        // Return None if no action is needed
        if let KPIStatus::NoActionNeeded(_) = kpi_status {
            return None;
        }

        // Return the KPIResult only if kpi_status is not in NoActionNeeded state
        Some(KPIResult {
            dependency_name: roadmap.dependency.clone(),
            current_version: current_version.clone(),
            kpi_status,
        })
    } else {
        panic!("No roadmap available");
    }
}
