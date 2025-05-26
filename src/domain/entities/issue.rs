use mongodb::bson::oid::ObjectId; // Temporary
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: Option<ObjectId>, // Or String
    pub title: String,
    pub body: Option<String>,
    pub status: Option<String>, // Consider an enum later
    pub number: i32,
    pub project_id: Option<ObjectId>, // Or String, to link to Project
    pub snapshot_id: Option<ObjectId>, // Or String, to link to Snapshot
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Add other relevant fields
}
