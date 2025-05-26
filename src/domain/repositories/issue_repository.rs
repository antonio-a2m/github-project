use async_trait::async_trait;
use crate::domain::entities::issue::Issue;
use anyhow::Result;
use mongodb::bson::oid::ObjectId; // Placeholder

#[async_trait]
pub trait IssueRepository {
    async fn save_issues(&self, issues: Vec<Issue>) -> Result<Vec<String>>; // Returns list of saved issue IDs
    async fn get_issues_by_snapshot_id(&self, snapshot_id: ObjectId) -> Result<Vec<Issue>>;
    // Add other necessary methods
}
