
use serde_json::Value;

use crate::models::AppConfig;
use crate::utils::run_json_get_query::run_json_get_query;
use crate::types::MyError;

pub fn fetch_repositories(
    config: &AppConfig,
    project_name: &str,
) -> Result<Vec<Value>, MyError> {
    let url_config = &*config.url_config; // Dereference the Box

    // Use UrlConfig to get the URL for repositories
    let repos_url = url_config.repos_url(project_name, "");

    let mut start = 0;
    let limit = 50;  // Adjust limit as needed
    let mut more_pages = true;
    let mut all_repos = Vec::new();

    while more_pages {
        let paginated_repos_url = format!(
            "{}?start={}&limit={}",
            repos_url, start, limit
        );

        let repos_json = run_json_get_query(config, &paginated_repos_url)?;

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
    }

    Ok(all_repos)
}

