use mongodb::{options::ClientOptions, Client as MongoClient};
use std::sync::Arc;
use anyhow::Result; // For error handling in main

// Utilities
mod utils; // Keep this if utils::env is still used directly in main
use utils::env::{load_env_config, EnvConfig};

// Application Layer
mod application; // Declare application module
use crate::application::use_cases::sync_project_issues::SyncProjectIssues;

// Domain Layer (Traits for Arc<dyn...>)
mod domain; // Declare domain module
use crate::domain::repositories::issue_provider::IssueProvider;
use crate::domain::repositories::issue_repository::IssueRepository as DomainIssueRepoTrait;
use crate::domain::repositories::project_repository::ProjectRepository as DomainProjectRepoTrait;
use crate::domain::repositories::snapshot_repository::SnapshotRepository as DomainSnapshotRepoTrait;

// Infrastructure Layer (Concrete Implementations)
mod infrastructure; // Declare infrastructure module
use crate::infrastructure::github::github_repository::GitHubRepository;
use crate::infrastructure::mongodb::{
    issue_repository::IssueRepository as MongoIssueRepo, // Alias to avoid confusion
    project_repository::ProjectRepository as MongoProjectRepo,
    snapshot_repository::SnapshotRepository as MongoSnapshotRepo,
};

#[tokio::main]
async fn main() -> Result<()> { // Changed to anyhow::Result
    // 1. Load environment configuration
    let env: EnvConfig = load_env_config()?;
    println!("Environment configuration loaded.");

    // 2. Connect to MongoDB
    let mongo_client_options = ClientOptions::parse(&env.mongodb_uri).await?;
    let mongo_client = MongoClient::with_options(mongo_client_options)?;
    let database = mongo_client.database(env.mongodb_database.as_str());
    println!("MongoDB connection established to database: {}", env.mongodb_database);

    // 3. Initialize concrete repositories and providers
    // GitHub as IssueProvider
    let github_provider = Arc::new(GitHubRepository::new(
        env.github_token.clone(), // Assuming GitHubRepository::new takes these
        env.github_api_url.clone(),
        env.github_owner.clone(),
    ));
    println!("GitHubProvider initialized for owner: {}.", env.github_owner);

    // MongoDB Repositories
    let project_repo_concrete = MongoProjectRepo::new(&database);
    let project_repo = Arc::new(project_repo_concrete);
    println!("MongoProjectRepo initialized.");

    let issue_repo_concrete = MongoIssueRepo::new(&database);
    let issue_repo = Arc::new(issue_repo_concrete);
    println!("MongoIssueRepo initialized.");

    let snapshot_repo_concrete = MongoSnapshotRepo::new(&database);
    let snapshot_repo = Arc::new(snapshot_repo_concrete);
    println!("MongoSnapshotRepo initialized.");

    // 4. Initialize the Use Case
    // Ensure types match for Arc<dyn Trait>
    let sync_use_case = SyncProjectIssues::new(
        project_repo as Arc<dyn DomainProjectRepoTrait + Send + Sync>,
        issue_repo as Arc<dyn DomainIssueRepoTrait + Send + Sync>,
        snapshot_repo as Arc<dyn DomainSnapshotRepoTrait + Send + Sync>,
        github_provider as Arc<dyn IssueProvider + Send + Sync>,
    );
    println!("SyncProjectIssues use case initialized.");

    // 5. Execute the Use Case
    println!(
        "Executing SyncProjectIssues use case for project #{} (Name: {})...", 
        env.github_project_number, env.github_project_name
    );
    sync_use_case
        .execute(env.github_project_number, &env.github_project_name)
        .await?;
    println!("Use case execution completed successfully for project #{}.", env.github_project_number);

    Ok(())
}
