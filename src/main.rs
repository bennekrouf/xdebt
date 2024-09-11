
use std::error::Error;
use dotenv::dotenv;

mod download_file;
mod run_maven_effective_pom;
mod analyze_pom_content;
mod generate_pom_analysis_json;

use crate::generate_pom_analysis_json::generate_pom_analysis_json;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

    // Define the repository and project names
    let repo_name = "gpecs";
    let project_name = "SES";

    // Define the target folder
    let target_folder = "tmp";

    // Define the list of keywords to search for
    let reference_keywords = ["jencks", "nexus", "xfile", "php", "richfaces"];

    // Call the function to generate the JSON result
    let json_result = generate_pom_analysis_json(project_name, repo_name, target_folder, &reference_keywords)?;

    // Print the JSON result
    println!("{}", serde_json::to_string_pretty(&json_result)?);

    Ok(())
}
