use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct EnvConfig {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub github_api_url: String,
    pub github_token: String,
    pub github_owner: String,
}

pub fn load_env_config() -> Result<EnvConfig, String> {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");

    let mongodb_uri = env::var("MONGODB_URI")
        .map_err(|_| "Error loading MONGODB_URI env variable".to_string())?;
    let mongodb_database = env::var("MONGODB_DATABASE")
        .map_err(|_| "Error loading MONGODB_DATABASE env variable".to_string())?;
    let github_api_url = env::var("GITHUB_API_URL")
        .map_err(|_| "Error loading GITHUB_API_URL env variable".to_string())?;
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| "Error loading GITHUB_TOKEN env variable".to_string())?;
    let github_owner = env::var("GITHUB_OWNER")
        .map_err(|_| "Error loading GITHUB_OWNER env variable".to_string())?;

    Ok(EnvConfig {
        mongodb_uri,
        mongodb_database,
        github_api_url,
        github_token,
        github_owner,
    })
}
