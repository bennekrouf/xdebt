use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapList {
    pub versions: Vec<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roadmap {
    pub product: String,
    domain: Option<String>,
    chapter: Option<String>,
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
pub struct Analysis {
    pub current: String,
    pub roadmap: Roadmap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub product: String,
    pub current_version: String,
    pub compliance_status: String,
    pub last_updated: String,
    pub maintenance_action: String,
}

