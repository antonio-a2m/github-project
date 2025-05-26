use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubIssue {
    pub id: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>, // Added body
    pub number: Option<i32>, // Added number (GitHub issue numbers are i32)
    pub state: Option<String>,
    pub state_reason: Option<String>,
    pub labels: Option<GithubLabels>,
    pub assignees: Option<GithubAssignees>,
    pub project_items: Option<GithubProjectItem>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubProjectItem {
    pub nodes: Vec<GithubFieldValues>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubFieldValues {
    pub field_values: GithubFieldValuesNode,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubFieldValuesNode {
    pub nodes: Vec<GithubCustomField>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubCustomField {
    pub field: Option<GithubCustomFieldInfo>,
    pub text: Option<String>,
    pub number: Option<f64>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubCustomFieldInfo {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubLabels {
    pub nodes: Vec<GithubLabel>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubLabel {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubAssignees {
    pub nodes: Vec<GithubUser>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct GithubUser {
    pub id: String,
    pub login: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubPageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubContent {
    pub content: Option<GithubIssue>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubProjectItems {
    pub page_info: GithubPageInfo,
    pub nodes: Vec<GithubContent>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubProjectV2 {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub number: i32,
    pub created_at: String,
    pub updated_at: String,
    pub items: Option<GithubProjectItems>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubUserProjects {
    pub project_v2: GithubProjectV2,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubData {
    pub organization: GithubUserProjects,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GithubResponse {
    pub data: GithubData,
}

impl GithubResponse {
    pub fn get_issues(&self) -> Vec<GithubIssue> {
        let mut issues = Vec::new();
        if let Some(items) = &self.data.organization.project_v2.items {
            for item in &items.nodes {
                if let Some(issue) = &item.content {
                    if let Some(issue_id) = &issue.id {
                        if issue_id.is_empty() {
                            continue;
                        }
                        issues.push(issue.clone());
                    }
                }
            }
        }
        issues
    }

    pub fn get_page_info(&self) -> GithubPageInfo {
        let new = GithubPageInfo {
            end_cursor: None,
            has_next_page: false,
        };
        if let Some(items) = &self.data.organization.project_v2.items {
            return items.page_info.clone();
        }
        new
    }
}
