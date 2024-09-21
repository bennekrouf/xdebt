
use chrono::Utc;
use crate::kpi::compare_versions::compare_versions;
use crate::models::{KPIResult, Analysis};

pub fn compute_kpi<'a>(analysis: &'a Analysis) -> KPIResult {
    let current_version = &analysis.dependency_version.version_number;
    // let repository_name = ;
    let today = Utc::now().date_naive();

    let mut compliance_status = "non-compliant".to_string();
    let mut maintenance_action = "Upgrade needed".to_string();

    // Use `.as_ref()` to borrow the `roadmap` rather than move it
    if let Some(roadmap) = analysis.roadmap.as_ref() {
        for record in &roadmap.entries {
            let record_version = &record.version;

            // Check if the current version is compliant with the record's version
            if compare_versions(current_version, record_version) {
                compliance_status = "compliant".to_string();

                // Check lifecycle status: parse end_date if it exists, otherwise skip comparison
                if record.end_date.as_ref().map_or(false, |date| *date <= today) {
                    maintenance_action = "Upgrade needed".to_string();
                } else {
                    maintenance_action = "No action needed".to_string();
                }
            }
        }

        KPIResult {
            dependency_name: roadmap.dependency.clone(),
            version_number: current_version.clone(),
            compliance_status,
            maintenance_action,
        }
    } else {
        panic!("No roadmap available");
    }
}

