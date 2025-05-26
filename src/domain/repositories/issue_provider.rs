use async_trait::async_trait;
use crate::domain::entities::issue::Issue; // Or DTOs if direct mapping is not feasible
use anyhow::Result;

// May need a struct to represent pagination info if the provider supports it
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[async_trait]
pub trait IssueProvider {
    async fn fetch_issues(
        &self,
        project_identifier: &str, // e.g., project number or name
        limit: Option<i32>,
        after_cursor: Option<String>,
    ) -> Result<(Vec<Issue>, PageInfo)>; // Returning domain Issues directly might be an option, or DTOs that the use case then maps
}
