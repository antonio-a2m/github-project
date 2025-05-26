use super::models::{GithubIssue, GithubPageInfo, GithubProjectV2, GithubResponse};
use reqwest::Client;
use serde::Serialize;
use std::error::Error; // Keep for the internal get_issues, but trait will use anyhow

use super::graphql::{query_issues::QUERY_ISSUES, query_project::QUERY_PROJECT};

// Imports for IssueProvider implementation
use crate::domain::entities::issue::Issue as DomainIssue;
use crate::domain::repositories::issue_provider::{IssueProvider, PageInfo as DomainPageInfo};
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, TimeZone}; // Added TimeZone for parsing

/// GitHub client for interacting with GitHub's GraphQL API
#[derive(Debug, Clone)]
pub struct GitHubRepository {
    client: Client,
    token: String,
    api_url: String,
    organization: String,
}

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

impl GitHubRepository {
    /// Create a new GitHub client
    pub fn new(token: String, api_url: String, organization: String) -> Self {
        let client = Client::new();
        let api_url = api_url;

        Self {
            client,
            token,
            api_url,
            organization,
        }
    }

    /// Get project information
    pub async fn get_project(
        &self,
        project_number: i32,
    ) -> Result<GithubProjectV2, Box<dyn Error>> {
        let variables = serde_json::json!({
            "organization": self.organization,
            "projectNumber": project_number
        });

        let response = self.execute_query(QUERY_PROJECT, variables).await?;
        let project_v2 = response.data.organization.project_v2;

        Ok(project_v2)
    }

    /// Get issues for a project with pagination
    // This is the original method, now effectively internal.
    // It returns Result with Box<dyn Error> which is fine for internal logic.
    // The trait implementation will wrap this and convert errors to anyhow::Error.
    async fn get_issues_from_github(
        &self,
        project_number: i32,
        limit: Option<i32>,
        after: Option<String>,
    ) -> Result<(Vec<GithubIssue>, GithubPageInfo), Box<dyn Error>> {
        // println!("project_number: {}", project_number); // Original println
        let variables = serde_json::json!({
            "organization": self.organization,
            "projectNumber": project_number,
            "limit": limit.unwrap_or(100),
            "after": after
        });

        let response = match self.execute_query(QUERY_ISSUES, variables).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error graphql github: {:?}", e);
                return Err(e);
            }
        };

        let issues = response.get_issues();
        let page_info = response.get_page_info();
        // println!("Número de issues obtenidos {}", issues.len()); // Original println

        Ok((issues, page_info))
    }

    /// Execute a GraphQL query against the GitHub API
    async fn execute_query(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<GithubResponse, Box<dyn Error>> {
        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(&self.api_url)
            .bearer_auth(&self.token)
            .header("User-Agent", "github-issues-migrator")
            .header("Accept", "application/json; charset=utf-8")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()).into());
        }

        let response_body: serde_json::Value = response.json().await?;

        if let Some(errors) = response_body.get("errors") {
            return Err(format!("GraphQL error: {}", errors).into());
        }

        let repsonse: GithubResponse = serde_json::from_value(response_body)?;

        Ok(repsonse)
    }
}

#[async_trait]
impl IssueProvider for GitHubRepository {
    async fn fetch_issues(
        &self,
        project_number_str: &str,
        limit: Option<i32>,
        after_cursor: Option<String>,
    ) -> Result<(Vec<DomainIssue>, DomainPageInfo)> {
        // 1. Parse project_number_str to i32
        let project_number = project_number_str
            .parse::<i32>()
            .map_err(|e| anyhow!("Invalid project number string: {}. Error: {}", project_number_str, e))?;

        // 2. Call the existing internal logic to fetch issues from GitHub
        let (github_issues, github_page_info) = self
            .get_issues_from_github(project_number, limit, after_cursor) // Calling the original method, renamed for clarity
            .await
            .map_err(|e| anyhow!("Failed to fetch issues from GitHub for project {}: {}", project_number, e))?;

        // 3. Transform GitHub issues (super::models::GithubIssue) to domain issues (DomainIssue)
        let domain_issues: Vec<DomainIssue> = github_issues
            .into_iter()
            .map(|gh_issue| {
                // Transform GithubIssue to DomainIssue
                // The fields `number` and `body` are now available in `gh_issue` due to previous subtask.
                DomainIssue {
                    id: None, // Will be set by the database layer
                    title: gh_issue.title.unwrap_or_default(),
                    body: gh_issue.body.clone(), // Updated mapping for body
                    status: gh_issue.state, // `state` in GithubIssue seems to map to `status`
                    number: gh_issue.number.unwrap_or(0), // Updated mapping for number
                    project_id: None, // This will be set by the use case or when saving
                    snapshot_id: None, // This will be set by the use case
                    created_at: gh_issue.created_at.as_ref().map_or_else(Utc::now, |dt_str| {
                        DateTime::parse_from_rfc3339(dt_str)
                            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc))
                    }),
                    updated_at: gh_issue.updated_at.as_ref().map_or_else(Utc::now, |dt_str| {
                        DateTime::parse_from_rfc3339(dt_str)
                            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc))
                    }),
                }
            })
            .collect();

        // 4. Transform GitHub page info (super::models::GithubPageInfo) to domain page info (DomainPageInfo)
        let domain_page_info = DomainPageInfo {
            has_next_page: github_page_info.has_next_page,
            end_cursor: github_page_info.end_cursor,
        };

        Ok((domain_issues, domain_page_info))
    }
}
