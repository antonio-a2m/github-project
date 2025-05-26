use crate::infrastructure::github::models::{GithubIssue, GithubUser};
use bson::DateTime;
use mongodb::bson::oid::ObjectId; // Added import for ObjectId
use serde::{Deserialize, Serialize};
use utils::string_to_bson_datetime::string_to_bson_datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueModel {
    #[serde(rename = "_id")]
    id: ObjectId,
    snapshot_id: ObjectId,
    // github_issue_id: String, // Potentially remove if DomainIssue is the source of truth
    // url: String, // Potentially remove
    title: String,
    body: Option<String>, // Added from DomainIssue
    number: i32, // Added from DomainIssue
    project_id: Option<ObjectId>, // Added from DomainIssue
    state: String, // Maps to status in DomainIssue
    state_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    estimate: Option<f64>,
    hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iteration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assigned: Option<UserModel>,
    created_at: DateTime, // This is bson::DateTime
    updated_at: DateTime, // This is bson::DateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    closed_at: Option<DateTime>, // No equivalent in DomainIssue shown

    // Fields like github_issue_id, url, state_reason, label, estimate, hours, iteration,
    // start_date, end_date, assigned might become Option or be removed if not in DomainIssue
    // For now, I'll keep them as per existing IssueModel and the From<DomainIssue> will not populate them.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserModel {
    id: String,
    login: String,
    url: String,
}

impl From<GithubUser> for UserModel {
    fn from(user: GithubUser) -> Self {
        UserModel {
            id: user.id,
            login: user.login,
            url: user.url,
        }
    }
}
// ...existing code...
impl IssueModel {
    // This method is for converting GithubIssue -> IssueModel
    // It will likely be removed or adapted if all issues come via DomainIssue
    pub fn from_issue(issue: GithubIssue, snapshot_id: ObjectId) -> Self {
        // Get the first label if available
        let label = issue.labels.as_ref().and_then(|labels| {
            if !labels.nodes.is_empty() {
                Some(labels.nodes[0].name.clone())
            } else {
                None
            }
        });

        // Get the first assignee if available
        let assigned = issue.assignees.as_ref().and_then(|assignees| {
            if !assignees.nodes.is_empty() {
                Some(assignees.nodes[0].clone().into())
            } else {
                None
            }
        });

        // Default values for optional fields
        let mut estimate: Option<f64> = None;
        let mut iteration: Option<String> = None;
        let mut start_date: Option<String> = None;
        let mut end_date: Option<String> = None;
        let mut hours: Option<f64> = None;

        let nodes = issue
            .project_items
            .as_ref()
            .map_or(vec![], |project_items| project_items.nodes.clone());
        // Extract custom field values from project_items
        for node in nodes {
            for field_value in node.field_values.nodes {
                if let Some(field) = field_value.field {
                    if field.name.is_empty() {
                        continue;
                    }
                    match field.name.as_str() {
                        "Estimate" => estimate = field_value.number,
                        "Iteration" => iteration = field_value.text,
                        "Start Date" => start_date = field_value.text,
                        "End Date" => end_date = field_value.text,
                        "Hours" => hours = field_value.number,
                        "Horas" => hours = field_value.number,
                        _ => {}
                    }
                }
            }
        }

        IssueModel {
            id: ObjectId::new(), // Generate a new ObjectId
            snapshot_id,         // Use the provided snapshot_id
            github_issue_id: issue.id.unwrap_or_default(),
            url: issue.url.unwrap_or_default(),
            title: issue.title.unwrap_or_default(),
            state: issue.state.unwrap_or_default(),
            state_reason: issue.state_reason,
            label,
            estimate,
            iteration,
            start_date,
            end_date,
            hours,
            assigned,
            created_at: string_to_bson_datetime(issue.created_at),
            updated_at: string_to_bson_datetime(issue.updated_at),
            closed_at: string_to_bson_datetime(issue.closed_at),
            // Initialize new fields not present in GithubIssue; they'll be set by From<DomainIssue>
            body: None, // Default for now
            number: issue.number.unwrap_or_default(), // GithubIssue now has number
            project_id: None, // Default for now, DomainIssue will provide
        }
    }
}

// Implementation of From<DomainIssue> for IssueModel
use crate::domain::entities::issue::Issue as DomainIssue;
use mongodb::bson::DateTime as BsonDateTime; // Ensure this is the correct DateTime

impl From<DomainIssue> for IssueModel {
    fn from(domain_issue: DomainIssue) -> Self {
        // Fields from existing IssueModel that are not in DomainIssue will be defaulted or None.
        // This might mean some data is lost if IssueModel was richer and is now sourced from DomainIssue.
        Self {
            _id: domain_issue.id.unwrap_or_else(ObjectId::new),
            title: domain_issue.title,
            body: domain_issue.body,
            state: domain_issue.status.unwrap_or_default(), // Corrected: maps to IssueModel.state
            number: domain_issue.number,
            project_id: domain_issue.project_id,
            snapshot_id: domain_issue.snapshot_id.expect("Snapshot ID must be present in DomainIssue when saving"),
            created_at: BsonDateTime::from_chrono(domain_issue.created_at),
            updated_at: BsonDateTime::from_chrono(domain_issue.updated_at),

            // Fields not in DomainIssue, set to default or None:
            github_issue_id: String::new(), // Or Option<String> and None
            url: String::new(), // Or Option<String> and None
            state_reason: None,
            label: None,
            estimate: None,
            hours: None,
            iteration: None,
            start_date: None,
            end_date: None,
            assigned: None,
            closed_at: None,
            // The 'state' field of IssueModel is mapped from domain_issue.status.
            // All other fields not in DomainIssue are defaulted as per previous logic.
        }
    }
}

// Implementation of From<IssueModel> for DomainIssue
impl From<IssueModel> for DomainIssue {
    fn from(model: IssueModel) -> Self {
        Self {
            id: Some(model._id),
            title: model.title,
            body: model.body, // Assumes body was added to IssueModel
            status: Some(model.state), // Maps from IssueModel.state
            number: model.number,   // Assumes number was added to IssueModel
            project_id: model.project_id, // Assumes project_id was added
            snapshot_id: Some(model.snapshot_id),
            created_at: model.created_at.to_chrono(),
            updated_at: model.updated_at.to_chrono(),
        }
    }
}
