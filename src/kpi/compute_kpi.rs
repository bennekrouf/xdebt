
use chrono::{NaiveDate, Utc};
use crate::kpi::compare_versions::compare_versions;
use crate::kpi::models::{KPIResult, Product};

pub fn compute_kpi(product: &Product) -> KPIResult {
    let current_version = &product.current;
    let today = Utc::now().date_naive();

    let mut compliance_status = "non-compliant".to_string();
    let mut maintenance_action = "Upgrade needed".to_string();

    for record in &product.roadmap.records {
        let record_version = &record.version;
        let etat = &record.etat;

        // Check if the current version is compliant with the record's version
        if compare_versions(current_version, record_version) {
            compliance_status = "compliant".to_string();

            // Check lifecycle status: parse end_date if it exists, otherwise skip comparison
            if etat == "out" || record.end_date.as_ref().map_or(false, |date_str| {
                NaiveDate::parse_from_str(&date_str.to_string(), "%Y-%m-%d")
                    .map(|date| date <= today)
                    .unwrap_or(false)  // If parsing fails, assume the version is out of date
            }) {
                maintenance_action = "Upgrade needed".to_string();
            } else {
                maintenance_action = "No action needed".to_string();
            }
        }
    }

    KPIResult {
        product: product.roadmap.product.clone(),
        current_version: current_version.clone(),
        compliance_status,
        last_updated: product.roadmap.records[0].updated_at.expect("Invalid date").to_string(),
        maintenance_action,
    }
}

