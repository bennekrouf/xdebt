
use chrono::Utc;
use crate::kpi::compare_versions::compare_versions;
use crate::models::{KPIResult, Analysis};
use std::cmp::Ordering;

pub fn compute_kpi<'a>(analysis: &'a Analysis) -> KPIResult {
    let current_version = &analysis.dependency_version.current_version;
    let today = Utc::now().date_naive();

    let mut compliance_status = "non-compliant".to_string();
    let mut maintenance_action = "Upgrade needed".to_string();
    let mut found_compliant = false;

    // Use `.as_ref()` to borrow the `roadmap` rather than move it
    if let Some(roadmap) = analysis.roadmap.as_ref() {
        // Sort roadmap entries by version and start_date (newer versions and more recent start dates first)
        let mut sorted_entries = roadmap.entries.clone();
        sorted_entries.sort_by(|a, b| {
            let cmp = compare_versions(&b.version, &a.version);
            if cmp {
                b.start_date.cmp(&a.start_date)
            } else {
                Ordering::Equal
            }
        });

        // Check if the current version exists in the roadmap
        let current_version_in_roadmap = sorted_entries.iter().any(|record| {
            record.version == *current_version
        });

        if current_version_in_roadmap {
            // Iterate through sorted roadmap entries and check compliance
            for record in &sorted_entries {
                let record_version = &record.version;

                // If the current version is equal to or newer than the roadmap version, mark as compliant
                if compare_versions(current_version, record_version) {
                    compliance_status = "compliant".to_string();
                    found_compliant = true;

                    // Check if the version has an end date, if so set the maintenance action
                    if let Some(end_date) = record.end_date {
                        if end_date <= today {
                            maintenance_action = format!("Upgrade needed by {}", end_date);
                        } else {
                            maintenance_action = "No action needed".to_string();
                        }
                    } else {
                        maintenance_action = "No action needed (latest version)".to_string();
                    }

                    // Break after finding the most recent compliant version
                    break;
                }
            }
        }

        // If no compliant version was found, or the current version is outdated/not in the roadmap
        if !found_compliant || !current_version_in_roadmap {
            // The current version is not in the roadmap, mark as non-compliant and upgrade to latest roadmap version
            if let Some(record) = sorted_entries.first() {
                maintenance_action = format!(
                    "Upgrade needed to version {} (latest version)",
                    record.version
                );
                if let Some(end_date) = record.end_date {
                    maintenance_action.push_str(&format!(" by {}", end_date));
                }
            }
        }

        KPIResult {
            dependency_name: roadmap.dependency.clone(),
            current_version: current_version.clone(),
            compliance_status,
            maintenance_action,
        }
    } else {
        panic!("No roadmap available");
    }
}

