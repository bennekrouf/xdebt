
use std::error::Error;
use tracing_subscriber;
use std::env;

mod generate_analysis;
mod utils;
mod process_pom;

use crate::generate_analysis::generate_analysis;
use crate::utils::fetch_repositories::fetch_repositories;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::utils::append_json_to_csv::append_json_to_csv;
use crate::utils::get_projects::get_projects;
use utils::create_client_with_auth::create_client_with_auth;
// use crate::csv_to_excel::csv_to_excel;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let (client, auth_header) = create_client_with_auth()?;

    // Fetch the list of projects using the new function
    let projects = get_projects(&client, &auth_header)?;

    // Get the base URL template for repos from .env
    let repos_url_template = env::var("REPOS_URL")
        .map_err(|e| format!("Missing REPOS_URL environment variable: {}", e))?;


    let projects = vec!["PTEP"];

    // Iterate over the list of projects
    for project in projects {
        // let project_name = project["key"].as_str()
        //     .ok_or("Failed to get project name")?;
        let project_name = project;

        // Fetch repositories using the new function
        let all_repos = fetch_repositories(&client, &auth_header, &repos_url_template, project_name)?;

        let all_repos = vec!["cadero"];

        // Iterate over all the repos and process each one
        for repo in &all_repos {
            // let repo_name = repo["name"].as_str()
            //     .ok_or("Missing repo name")?;
            let repo_name = repo;
            if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
                continue;
            }

            // Call the function to generate the JSON result for each repo
            match generate_analysis(&client, &auth_header, project_name, repo_name) {
                Ok(json_result) => {
                    println!("Project: {}, Repo: {}", project_name, repo_name);
                    println!("{}", serde_json::to_string_pretty(&json_result)?);

                    // Append the JSON result to the file
                    if let Err(e) = append_json_to_file(project_name, &json_result) {
                        eprintln!("Failed to append JSON to file for project '{}', repo '{}': {}", project_name, repo_name, e);
                    }

                    // Also append the JSON result to the CSV file
                    if let Err(e) = append_json_to_csv(project_name, &json_result) {
                        eprintln!("Failed to append JSON to CSV for project '{}', repo '{}': {}", project_name, repo_name, e);
                    }

                    // Convert the CSV to Excel with the same name
                    // let target_folder = env::var("TARGET_FOLDER").unwrap_or_else(|_| "tmp".to_string());
                    // let csv_file_path = format!("{}/{}.csv", &target_folder, project_name);
                    // let excel_file_path = format!("{}/{}.xlsx", &target_folder, project_name);
                    // if let Err(e) = csv_to_excel(&csv_file_path, &excel_file_path) {
                    //     eprintln!("Failed to convert CSV to Excel for project '{}': {}", project_name, e);
                    // }
                }
                Err(e) => {
                    eprintln!("Failed to generate POM analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
                }
            }
        }
    }

    Ok(())
}
