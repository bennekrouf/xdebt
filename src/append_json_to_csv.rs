
use std::env;
use std::error::Error;
// use std::fs::OpenOptions;
use std::path::Path;
use serde_json::Value;
use csv::WriterBuilder;

pub fn append_json_to_csv(project_name: &str, json_data: &Value) -> Result<(), Box<dyn Error>> {
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string()); // Default to "tmp" if not set

    // Create or open the CSV file for the project
    let file_path = format!("{}/{}.csv", &target_folder, project_name);
    let path = Path::new(&file_path);
    let mut writer = if path.exists() {
        WriterBuilder::new().has_headers(false).from_path(&file_path)?
    } else {
        WriterBuilder::new().from_path(&file_path)?
    };

    // Flatten the JSON data
    let repository = json_data.get("repository").and_then(Value::as_str).unwrap_or("");

    let binding_map = serde_json::Map::new();
    let versions = json_data.get("versions").and_then(Value::as_object).unwrap_or(&binding_map);

    let binding_vec = vec![];
    let references = json_data.get("references").and_then(Value::as_array).unwrap_or(&binding_vec);

    // Prepare the CSV row
    let mut row = vec![repository.to_string()];

    // Add versions to the row dynamically
    let mut versions_map = std::collections::HashMap::new();
    // if let versions_object = versions {
        for (key, value) in versions {
            if let Some(version_str) = value.as_str() {
                versions_map.insert(key.to_string(), version_str.to_string());
            }
        }
    // }

    // Add all versions to the row
    // Ensure consistent column order, e.g., alphabetically sorted
    let mut sorted_versions_keys: Vec<String> = versions_map.keys().cloned().collect();
    sorted_versions_keys.sort();
    for key in sorted_versions_keys {
        row.push(versions_map.get(&key).unwrap_or(&"".to_string()).to_string());
    }

    // Add references to the row
    row.push(references.iter().map(|r| r.as_str().unwrap_or("")).collect::<Vec<&str>>().join(","));

    // Write the row to the CSV file
    writer.write_record(&row)?;

    Ok(())
}
