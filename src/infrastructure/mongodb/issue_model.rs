use crate::infrastructure::github::models::{GithubIssue, GithubUser};
use bson::DateTime;
use mongodb::bson::oid::ObjectId; // Added import for ObjectId
use serde::{Deserialize, Serialize};
use crate::utils::string_to_bson_datetime::string_to_bson_datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueModel {
    #[serde(rename = "_id")]
    id: ObjectId,
    snapshot_id: ObjectId,
    github_issue_id: String,
    url: String,
    title: String,
    state: String,
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
    created_at: DateTime,
    updated_at: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed_at: Option<DateTime>,
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
            created_at: string_to_bson_datetime(&issue.created_at.unwrap_or_default()).unwrap(),
            updated_at: string_to_bson_datetime(&issue.updated_at.unwrap_or_default()).unwrap(),
            closed_at: issue.closed_at.map(|date| string_to_bson_datetime(&date).unwrap()),
        }
    }
}
