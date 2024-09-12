
use std::fs::OpenOptions;
use std::io::Write;
use serde_json::Value;
use std::error::Error;
use serde_json::json;
use std::env;

pub fn append_json_to_file(project_name: &str, json_data: &Value) -> Result<(), Box<dyn Error>> {
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string());  // Default to "tmp" if not set

    // Create or open the file for the project
    let file_path = format!("{}/{}.json", &target_folder, project_name);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| format!("Error opening file {}: {}", file_path, e))?;

    // Prepare the JSON data to append
    let json_entry = json!(json_data);

    // Write the JSON data to the file
    writeln!(file, "{}", serde_json::to_string_pretty(&json_entry)?)?;

    Ok(())
}
