
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::error::Error;
use tracing::{error,info};

pub fn fetch_repositories(
    client: &Client,
    auth_header: &str,
    repos_url_template: &str,
    project_name: &str,
) -> Result<Vec<Value>, Box<dyn Error>> {
    info!("repos_url_template : {}", &repos_url_template);
    let repos_url = repos_url_template.replace("{project_name}", project_name);

    let mut start = 0;
    let limit = 50;  // Adjust limit as needed
    let mut more_pages = true;
    let mut all_repos = Vec::new();

    while more_pages {
        let paginated_repos_url = format!(
            "{}?start={}&limit={}",
            repos_url, start, limit
        );

        let response = client
            .get(&paginated_repos_url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .map_err(|e| format!("Error fetching repos URL {}: {}", paginated_repos_url, e))?;

        if response.status().is_success() {
            let repos_body = response.text()
                .map_err(|e| format!("Error reading repos response body: {}", e))?;
            let repos_json: Value = serde_json::from_str(&repos_body)
                .map_err(|e| format!("Error parsing repos JSON: {}", e))?;
            let repos = repos_json["values"]
                .as_array()
                .ok_or("Failed to parse repos list")?;

            all_repos.extend(repos.to_vec());

            // Check if there are more pages
            if repos.len() < limit {
                more_pages = false;
            } else {
                start += limit;
            }
        } else {
            error!("Failed to fetch repos {}, status: {}", paginated_repos_url, response.status());
            more_pages = false;
        }
    }

    Ok(all_repos)
}

