use infrastructure::mongodb::issue_repository;
use mongodb::{options::ClientOptions, Client as MongoClient};

mod infrastructure;
mod utils;

use crate::infrastructure::{
    github::github_repository::GitHubRepository,
    mongodb::{project_repository::ProjectRepository, snapshot_repository::SnapshotRepository},
};
use utils::env::{load_env_config, EnvConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <project_number>", args[0]);
        std::process::exit(1);
    }
    let project_number = args[1]
        .parse()
        .expect("Project number must be a positive integer");

    //lectura de variables de entorno
    let env: EnvConfig = load_env_config()?;

    // conectar base de datos
    let mongo_client_options = ClientOptions::parse(&env.mongodb_uri).await?;
    let mongo_client = MongoClient::with_options(mongo_client_options)?;
    let database = mongo_client.database(env.mongodb_database.as_str());

    // crear githu repository
    let github_repository =
        GitHubRepository::new(env.github_token, env.github_api_url, env.github_owner);

    // leer el proyecto desde base de datos
    let project_repository = ProjectRepository::new(&database, github_repository.clone());
    let project_info = project_repository.find_or_create(project_number).await?;
    let project_id = project_info.id.clone();
    println!("project_id: {:?}", project_id);

    // crear snapshot
    let snapshot_repository = SnapshotRepository::new(&database);
    let snapshot = snapshot_repository
        .create(project_number, project_id)
        .await?;

    // leer issues
    let issue_repository =
        issue_repository::IssueRepository::new(&database, github_repository, snapshot.id.clone());

    issue_repository
        .insert_from_github_project(project_number)
        .await?;

    Ok(())
}
