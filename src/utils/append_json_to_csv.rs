
use std::env;
use std::error::Error;
use std::path::Path;
use serde_json::Value;
use csv::WriterBuilder;
use serde_json::Map;
use std::fs::OpenOptions;
// use std::io::Write;

pub fn append_json_to_csv(project_name: &str, json_data: &Value) -> Result<(), Box<dyn Error>> {
    let output_folder = env::var("OUTPUT_FOLDER").unwrap_or_else(|_| "tmp".to_string()); // Default to "tmp" if not set

    // Create or open the CSV file for the project
    let file_path = format!("{}/{}.csv", &output_folder, project_name);
    let path = Path::new(&file_path);

    // Create a persistent Map to use as the default value for versions
    let default_versions = Map::new();

    // Determine headers from JSON object keys
    let versions = json_data.get("versions").and_then(Value::as_object).unwrap_or(&default_versions);
    let mut headers: Vec<String> = versions.keys().cloned().collect();
    headers.sort(); // Optional: sort headers alphabetically

    // Check if file exists and open it in append mode
    let file_exists = path.exists();
    let file = OpenOptions::new().create(true).append(true).open(&file_path)?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    // Write headers only if the file did not previously exist
    if !file_exists {
        writer.write_record(
            ["repository".to_string()]
                .into_iter()
                .chain(headers.clone())
                .chain(vec!["references".to_string()]) // References is the last column
        )?;
    }

    // Flatten the JSON data
    let repository = json_data.get("repository").and_then(Value::as_str).unwrap_or("");

    // Create a persistent Vec to use as the default value for references
    let default_references: Vec<Value> = Vec::new();
    let references = json_data.get("references").and_then(Value::as_array).unwrap_or(&default_references);

    // Prepare the CSV row
    let mut row = vec![repository.to_string()];

    // Add versions to the row in the order of headers
    for header in &headers {
        row.push(versions.get(header).and_then(Value::as_str).unwrap_or("").to_string());
    }

    // Add references to the row
    row.push(references.iter().map(|r| r.as_str().unwrap_or("")).collect::<Vec<&str>>().join(","));

    // Write the row to the CSV file
    writer.write_record(&row)?;

    Ok(())
}

