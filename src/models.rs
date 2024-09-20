use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapList {
    pub roadmap_list: Vec<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roadmap {
    pub dependency: String,
    pub domain: Option<String>,
    pub chapter: Option<String>,
    pub entries: Vec<RoadmapEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapEntry {
    pub version: String,
    start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    extended_end_date: Option<NaiveDate>,
    comment: Option<String>,
    source_name: Option<String>,
    source_entity: Option<String>,
    pub updated_at: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyVersion {
    pub version_number: String,
    pub dependency_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub repository_name: String,
    pub dependency_version: DependencyVersion,
    pub roadmap: Option<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult<'a> {
    pub repository_name: &'a str,
    pub dependency_name: String,
    pub version_number: String,
    pub compliance_status: String,
    pub maintenance_action: String,
}

