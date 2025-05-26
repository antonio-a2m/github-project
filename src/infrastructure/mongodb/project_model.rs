use crate::{domain::entities::project::Project as DomainProject, infrastructure::github::models::GithubProjectV2};
use bson::DateTime as BsonDateTime; // Renamed to avoid conflict if chrono::DateTime is used directly
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModel {
    #[serde(rename = "_id")]
    pub _id: ObjectId, // Changed from id to _id to match convention and avoid conflict
    pub github_id: Option<String>, // Made Option, not in DomainProject
    pub owner_id: Option<String>,  // Made Option, not in DomainProject
    pub title: String, // Maps to DomainProject.name
    pub number: i32,
    pub url: Option<String>,       // Made Option, not in DomainProject
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}

// This From is from the old structure, might be deprecated or adapted if GitHub direct import is still needed
impl From<GithubProjectV2> for ProjectModel {
    fn from(project: GithubProjectV2) -> Self {
        let now = BsonDateTime::from_chrono(Utc::now());
        Self {
            _id: ObjectId::new(),
            github_id: Some(project.id),
            owner_id: Some("somossoftrek".to_string()), // Example default
            title: project.title,
            number: project.number,
            url: Some(project.url),
            created_at: now,
            updated_at: now,
        }
    }
}

// From DomainProject to ProjectModel (for saving to DB)
impl From<DomainProject> for ProjectModel {
    fn from(domain_project: DomainProject) -> Self {
        let now_bson = BsonDateTime::from_chrono(Utc::now());
        Self {
            _id: domain_project.id.unwrap_or_else(ObjectId::new),
            title: domain_project.name, // Map name to title
            number: domain_project.number,
            // Fields not in DomainProject are defaulted or None
            github_id: None,
            owner_id: None,
            url: None,
            created_at: now_bson, // Set timestamps during creation
            updated_at: now_bson,
        }
    }
}

// From ProjectModel to DomainProject (for reading from DB)
impl From<ProjectModel> for DomainProject {
    fn from(model: ProjectModel) -> Self {
        Self {
            id: Some(model._id),
            name: model.title, // Map title back to name
            number: model.number,
        }
    }
}
