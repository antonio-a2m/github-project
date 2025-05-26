use async_trait::async_trait;
use crate::domain::entities::snapshot::Snapshot;
use anyhow::Result;
use mongodb::bson::oid::ObjectId; // Placeholder

#[async_trait]
pub trait SnapshotRepository {
    async fn create(&self, project_id: ObjectId, project_number: i32) -> Result<Snapshot>;
    async fn get_by_id(&self, id: ObjectId) -> Result<Option<Snapshot>>;
    // Add other necessary methods
}
