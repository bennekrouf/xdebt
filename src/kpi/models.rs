
use serde::{Deserialize, Serialize};
// use chrono::{NaiveDate, Utc};

use crate::roadmap::models::ProductVersion;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub current: String,
    pub roadmap: ProductVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub product: String,
    pub current_version: String,
    pub compliance_status: String,
    pub last_updated: String,
    pub maintenance_action: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductCatalog {
    #[serde(rename = "spring-framework")]
    spring_framework: Product,
}
