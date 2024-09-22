use serde_json::Value;
use std::error::Error;

use crate::models::AppConfig;

pub fn run_get_query(
    config: &AppConfig,
    paginated_repos_url: &str,
) -> Result<Value, Box<dyn Error>> {
    let client = &config.client;
    let (auth_name, auth_value) = config.auth_header.clone(); // Extract auth header
    let (user_agent_name, user_agent_value) = config.auth_user_agent.clone();

    let response = client
        .get(paginated_repos_url)
        .header(auth_name, auth_value) // Use the extracted auth header
        .header(user_agent_name, user_agent_value)
        .send()
        .map_err(|e| format!("Error fetching repos URL {}: {}", paginated_repos_url, e))?;

    if response.status().is_success() {
        let repos_body = response.text()
            .map_err(|e| format!("Error reading repos response body: {}", e))?;
        let repos_json: Value = serde_json::from_str(&repos_body)
            .map_err(|e| format!("Error parsing repos JSON: {}", e))?;
        Ok(repos_json)
    } else {
        Err(format!("Failed to fetch repos, status: {}", response.status()).into())
    }
}
