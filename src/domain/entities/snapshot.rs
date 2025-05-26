use mongodb::bson::oid::ObjectId; // Temporary
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: Option<ObjectId>, // Or String
    pub project_id: Option<ObjectId>, // Or String
    pub created_at: DateTime<Utc>,
    // Add other relevant fields
}
