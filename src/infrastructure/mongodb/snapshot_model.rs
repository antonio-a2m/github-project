use crate::domain::entities::snapshot::Snapshot as DomainSnapshot;
use bson::DateTime as BsonDateTime; // Explicitly use BsonDateTime
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::Utc; // For Utc::now()

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotModel {
    #[serde(rename = "_id")]
    pub _id: ObjectId, // Renamed from id
    pub project_id: ObjectId,
    pub project_number: i32, // This field is in SnapshotModel but not DomainSnapshot
    pub created_at: BsonDateTime, // Changed from Option<DateTime>
    // updated_at and closed_at removed for simplicity as they are not in DomainSnapshot
    // and updated_at was just being set to now() in the old new() method.
}

impl SnapshotModel {
    // This constructor might still be useful for direct instantiation if needed,
    // but the repository's create method will construct it directly.
    pub fn new(project_id: ObjectId, project_number: i32) -> Self {
        Self {
            _id: ObjectId::new(),
            project_id,
            project_number,
            created_at: BsonDateTime::from_chrono(Utc::now()),
        }
    }
}

// From SnapshotModel to DomainSnapshot (for reading from DB)
impl From<SnapshotModel> for DomainSnapshot {
    fn from(model: SnapshotModel) -> Self {
        Self {
            id: Some(model._id),
            project_id: Some(model.project_id), // DomainSnapshot.project_id is Option<ObjectId>
            created_at: model.created_at.to_chrono(), // Convert BsonDateTime to chrono::DateTime<Utc>
        }
    }
}

// From DomainSnapshot to SnapshotModel (potentially for a generic save, but create method is specific)
// This direction is problematic due to project_number.
impl From<DomainSnapshot> for SnapshotModel {
    fn from(domain_snapshot: DomainSnapshot) -> Self {
        Self {
            _id: domain_snapshot.id.unwrap_or_else(ObjectId::new),
            project_id: domain_snapshot.project_id.expect("DomainSnapshot must have a project_id for this conversion"),
            // project_number is missing from DomainSnapshot. This is a known issue.
            // If this conversion is strictly needed, DomainSnapshot must be augmented or
            // project_number must be supplied differently.
            // For the current task, the repo's create method builds SnapshotModel directly.
            project_number: 0, // Placeholder, as DomainSnapshot doesn't carry this.
            created_at: BsonDateTime::from_chrono(domain_snapshot.created_at),
        }
    }
}
