use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Versions {
    pub versions: Vec<ProductVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVersion {
    pub product: String,
    domain: Option<String>,
    chapter: Option<String>,
    pub records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub version: String,
    pub etat: String,
    start_date: Option<NaiveDate>,                // Date field
    pub end_date: Option<NaiveDate>,          // Optional Date field
    extended_end_date: Option<NaiveDate>, // Optional Date field
    comment: Option<String>,
    source_name: Option<String>,
    source_entity: Option<String>,
    pub updated_at: Option<NaiveDate>,                // Date field
}
