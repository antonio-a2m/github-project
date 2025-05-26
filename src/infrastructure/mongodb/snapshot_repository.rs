use async_trait::async_trait;
use mongodb::{bson::{doc, oid::ObjectId, DateTime as BsonDateTime}, Collection, Database};
use anyhow::{Result, Context}; // Removed unused 'anyhow' import, kept Context
use chrono::Utc;

use crate::domain::entities::snapshot::Snapshot as DomainSnapshot;
use crate::domain::repositories::snapshot_repository::SnapshotRepository as DomainSnapshotRepository;
use super::snapshot_model::SnapshotModel;

pub struct SnapshotRepository {
    collection: Collection<SnapshotModel>,
}

impl SnapshotRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<SnapshotModel>("snapshots");
        Self { collection }
    }
}

#[async_trait]
impl DomainSnapshotRepository for SnapshotRepository {
    async fn create(&self, project_id: ObjectId, project_number: i32) -> Result<DomainSnapshot> {
        let now_utc = Utc::now();
        // SnapshotModel::new was updated in the previous step to take (project_id, project_number)
        // and set _id internally and created_at.
        // Let's use that constructor for clarity if it matches, or construct directly.
        // The SnapshotModel::new(project_id, project_number) from previous step:
        // Self { _id: ObjectId::new(), project_id, project_number, created_at: BsonDateTime::from_chrono(Utc::now()) }
        // This is perfect.
        let new_snapshot_model = SnapshotModel::new(project_id, project_number);
        
        // The previous SnapshotModel::new() in snapshot_model.rs was:
        // pub fn new(project_id: ObjectId, project_number: i32) -> Self {
        //     Self {
        //         _id: ObjectId::new(),
        //         project_id,
        //         project_number,
        //         created_at: BsonDateTime::from_chrono(Utc::now()),
        //     }
        // }
        // This aligns with the fields needed.

        self.collection
            .insert_one(&new_snapshot_model, None)
            .await
            .context("Failed to insert new snapshot into MongoDB")?;

        // The From<SnapshotModel> for DomainSnapshot is already in snapshot_model.rs
        Ok(DomainSnapshot::from(new_snapshot_model))
    }

    async fn get_by_id(&self, id: ObjectId) -> Result<Option<DomainSnapshot>> {
        let model = self
            .collection
            .find_one(doc! { "_id": id }, None)
            .await
            .context(format!("Failed to find snapshot by id: {}", id))?;
        
        // The From<SnapshotModel> for DomainSnapshot is already in snapshot_model.rs
        Ok(model.map(DomainSnapshot::from))
    }
}
