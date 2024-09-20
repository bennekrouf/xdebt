use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapList {
    pub roadmap_list: Vec<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roadmap {
    pub product: String,
    pub domain: Option<String>,
    pub chapter: Option<String>,
    pub entries: Vec<RoadmapEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapEntry {
    pub version: String,
    pub etat: String,
    start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    extended_end_date: Option<NaiveDate>,
    comment: Option<String>,
    source_name: Option<String>,
    source_entity: Option<String>,
    pub updated_at: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVersion {
    pub version_number: String,
    pub product_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub product_version: ProductVersion,
    pub roadmap: Option<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub product_name: String,
    pub version_number: String,
    pub compliance_status: String,
    pub maintenance_action: String,
}

