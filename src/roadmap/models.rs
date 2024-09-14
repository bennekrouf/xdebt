use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Versions {
    pub versions: Vec<ProductVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVersion {
    pub product: String,
    domain: String,
    records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    chapter: String,
    version: String,
    etat: String,
    start_date: NaiveDate,                // Date field
    end_date: Option<NaiveDate>,          // Optional Date field
    extended_end_date: Option<NaiveDate>, // Optional Date field
    comment: String,
    source_name: String,
    source_entity: String,
    updated_at: NaiveDate,                // Date field
}
