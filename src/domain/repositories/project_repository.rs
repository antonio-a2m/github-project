use async_trait::async_trait;
use crate::domain::entities::project::Project;
use anyhow::Result; // Or a custom domain error type

#[async_trait]
pub trait ProjectRepository {
    async fn find_by_number(&self, number: i32) -> Result<Option<Project>>;
    async fn find_or_create(&self, project_number: i32, name: &str /* other necessary fields */) -> Result<Project>;
    async fn create(&self, project: &Project) -> Result<Project>;
    // Add other necessary methods
}
