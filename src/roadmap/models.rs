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
    records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    version: String,
    etat: String,
    start_date: Option<NaiveDate>,                // Date field
    end_date: Option<NaiveDate>,          // Optional Date field
    extended_end_date: Option<NaiveDate>, // Optional Date field
    comment: Option<String>,
    source_name: Option<String>,
    source_entity: Option<String>,
    updated_at: Option<NaiveDate>,                // Date field
}
