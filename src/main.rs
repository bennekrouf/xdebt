
use std::error::Error;
use std::env;
use dialog::Input;
use tracing_subscriber;

mod analyze_one_repo;
mod utils;
mod plugins;

use crate::analyze_one_repo::analyze_one_repo;
use crate::utils::fetch_repositories::fetch_repositories;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::utils::append_json_to_csv::append_json_to_csv;
use crate::utils::get_projects::get_projects;
use utils::create_client_with_auth::create_client_with_auth;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let (client, auth_header) = create_client_with_auth()?;
    let repos_url_template = env::var("REPOS_URL")
        .map_err(|e| format!("Missing REPOS_URL environment variable: {}", e))?;

    loop {
        let choice = display_menu()?;

        match choice.trim() {
            "1" => analyze_all_repositories(&client, &auth_header, &repos_url_template)?,
            "2" => analyze_project_repositories(&client, &auth_header, &repos_url_template)?,
            "3" => analyze_specific_repository(&client, &auth_header, &repos_url_template)?,
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}

fn display_menu() -> Result<String, Box<dyn Error>> {
    let choice: String = Input::new()
        .with_prompt("Select an option:\n1. Run analysis on all projects and all repositories\n2. Enter a project and analyze all repositories\n3. Select a repository to analyze across all projects\n4. Exit")
        .interact_text()?;
    Ok(choice)
}

fn analyze_all_repositories(
    client: &reqwest::blocking::Client,
    auth_header: &str,
    repos_url_template: &str
) -> Result<(), Box<dyn Error>> {
    let projects = get_projects(client, auth_header)?;
    for project in projects {
        let project_name = project["key"].as_str().ok_or("Failed to get project name")?;
        let all_repos = fetch_repositories(client, auth_header, repos_url_template, project_name)?;
        for repo in all_repos {
            let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;
            run_analysis(client, auth_header, project_name, repo_name)?;
        }
    }
    Ok(())
}

fn analyze_project_repositories(
    client: &reqwest::blocking::Client,
    auth_header: &str,
    repos_url_template: &str
) -> Result<(), Box<dyn Error>> {
    let project_name: String = Input::new()
        .with_prompt("Enter the project name (e.g., PTEP):")
        .interact_text()?;

    let all_repos = fetch_repositories(client, auth_header, repos_url_template, &project_name)?;
    for repo in all_repos {
        let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;
        run_analysis(client, auth_header, &project_name, repo_name)?;
    }
    Ok(())
}

fn analyze_specific_repository(
    client: &reqwest::blocking::Client,
    auth_header: &str,
    repos_url_template: &str
) -> Result<(), Box<dyn Error>> {
    let repo_name: String = Input::new()
        .with_prompt("Enter the repository name (e.g., xcad):")
        .interact_text()?;

    let projects = get_projects(client, auth_header)?;
    for project in projects {
        let project_name = project["key"].as_str().ok_or("Failed to get project name")?;
        let all_repos = fetch_repositories(client, auth_header, repos_url_template, project_name)?;
        for repo in all_repos {
            let repo_actual_name = repo["name"].as_str().ok_or("Missing repo name")?;
            if repo_actual_name == repo_name {
                run_analysis(client, auth_header, project_name, &repo_name)?;
            }
        }
    }
    Ok(())
}

fn run_analysis(
    client: &reqwest::blocking::Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn Error>> {
    if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
        return Ok(());
    }

    match analyze_one_repo(client, auth_header, project_name, repo_name) {
        Ok(json_result) => {
            println!("Project: {}, Repo: {}", project_name, repo_name);
            println!("{}", serde_json::to_string_pretty(&json_result)?);

            if let Err(e) = append_json_to_file(project_name, &json_result) {
                eprintln!("Failed to append JSON to file for project '{}', repo '{}': {}", project_name, repo_name, e);
            }

            if let Err(e) = append_json_to_csv(project_name, &json_result) {
                eprintln!("Failed to append JSON to CSV for project '{}', repo '{}': {}", project_name, repo_name, e);
            }
        }
        Err(e) => {
            eprintln!("Failed to generate POM analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
        }
    }

    Ok(())
}

