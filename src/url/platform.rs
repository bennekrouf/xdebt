
use crate::url::bitbucket::BitbucketConfig;
use crate::url::github::GithubConfig;
use crate::url::models::UrlConfig;

enum Platform {
    Bitbucket(BitbucketConfig),
    Github(GithubConfig),
}

impl Platform {
    pub fn get_urls(&self) -> &dyn UrlConfig {
        match self {
            Platform::Bitbucket(config) => config,
            Platform::Github(config) => config,
        }
    }
}
