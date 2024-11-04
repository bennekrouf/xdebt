use serde_json::Value;
use std::error::Error;

use crate::models::AppConfig;
use crate::utils::run_json_get_query::run_json_get_query;

pub fn fetch_repositories(
    config: &AppConfig,
    project_name: &str,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let url_config = &*config.url_config; // Dereference the Box

    // Use UrlConfig to get the URL for repositories
    let repos_url = url_config.repos_url(project_name, "");

    let mut start = 0;
    let limit = 50; // Adjust limit as needed
    let mut more_pages = true;
    let mut all_repos = Vec::new();

    while more_pages {
        let paginated_repos_url = format!("{}?start={}&limit={}", repos_url, start, limit);

        let repos_json = run_json_get_query(config, &paginated_repos_url)?;
        tracing::info!("repos_json : {}", repos_json);

        // Handle platform-specific response structures
        let repos = match config.platform.as_str() {
            "bitbucket" => {
                // Bitbucket uses a "values" key for paginated repositories
                repos_json["values"]
                    .as_array()
                    .ok_or("Failed to parse Bitbucket repos list")?
                    .to_vec()
            }
            "github" => {
                // GitHub returns the repositories directly as an array in the root
                repos_json
                    .as_array()
                    .ok_or("Failed to parse GitHub repos list")?
                    .to_vec()
            }
            _ => return Err("Unsupported platform".into()),
        };
        let length = repos.len();
        all_repos.extend(repos);

        // Check if there are more pages (Bitbucket-specific logic, GitHub has no pagination here)
        if config.platform == "bitbucket" {
            if length < limit {
                more_pages = false;
            } else {
                start += limit;
            }
        } else {
            more_pages = false;
        }
    }

    Ok(all_repos)
}
