use std::sync::Arc;
// Using ApplicationError for custom error handling
use crate::application::error::ApplicationError;

// Import traits from the domain layer
use crate::domain::repositories::project_repository::ProjectRepository;
use crate::domain::repositories::issue_repository::IssueRepository;
use crate::domain::repositories::snapshot_repository::SnapshotRepository;
use crate::domain::repositories::issue_provider::{IssueProvider, PageInfo};
use crate::domain::entities::{project::Project, issue::Issue, snapshot::Snapshot}; // Assuming these entities are in domain::entities

pub struct SyncProjectIssues {
    project_repository: Arc<dyn ProjectRepository + Send + Sync>, // Added Send + Sync for async trait methods in Arc<dyn>
    issue_repository: Arc<dyn IssueRepository + Send + Sync>,
    snapshot_repository: Arc<dyn SnapshotRepository + Send + Sync>,
    issue_provider: Arc<dyn IssueProvider + Send + Sync>,
}

impl SyncProjectIssues {
    pub fn new(
        project_repository: Arc<dyn ProjectRepository + Send + Sync>,
        issue_repository: Arc<dyn IssueRepository + Send + Sync>,
        snapshot_repository: Arc<dyn SnapshotRepository + Send + Sync>,
        issue_provider: Arc<dyn IssueProvider + Send + Sync>,
    ) -> Self {
        Self {
            project_repository,
            issue_repository,
            snapshot_repository,
            issue_provider,
        }
    }

    pub async fn execute(&self, project_number: i32, project_name: &str) -> Result<(), ApplicationError> {
        // 1. Find or create the project using the project_repository.
        println!("Attempting to find or create project #{}", project_number);
        let project = self.project_repository
            .find_or_create(project_number, project_name)
            .await
            .map_err(ApplicationError::RepositoryError)?;
        let project_id = project.id.ok_or_else(|| ApplicationError::ProcessingError("Project ID not found after creation/retrieval".to_string()))?;
        println!("Project ID: {:?}", project_id);

        // 2. Create a new snapshot for this synchronization event.
        println!("Creating snapshot for project #{}", project_number);
        let snapshot = self.snapshot_repository
            .create(project_id, project_number)
            .await
            .map_err(ApplicationError::RepositoryError)?;
        let snapshot_id = snapshot.id.ok_or_else(|| ApplicationError::ProcessingError("Snapshot ID not found after creation".to_string()))?;
        println!("Snapshot ID: {:?}", snapshot_id);

        // 3. Fetch issues from the issue_provider (e.g., GitHub).
        //    This might involve pagination.
        let mut all_issues: Vec<Issue> = Vec::new();
        let mut after_cursor: Option<String> = None;
        let limit = Some(100); // Or from config

        println!("Fetching issues for project #{} (identifier: {})", project_number, project.number); // Assuming project.number is the identifier for the provider

        loop {
            let (mut issues_page, page_info) = self.issue_provider
                .fetch_issues(&project.number.to_string(), limit, after_cursor.clone())
                .await
                .map_err(|e| ApplicationError::ProviderError { source_error: e })?;

            if issues_page.is_empty() {
                println!("No more issues found for project #{}", project_number);
                break;
            }

            println!("Fetched {} issues in this page.", issues_page.len());

            // Assign snapshot_id to each issue
            for issue in issues_page.iter_mut() {
                issue.snapshot_id = Some(snapshot_id);
                // issue.project_id = Some(project_id); // Assign project_id if not already set by provider
            }

            all_issues.extend(issues_page);

            if !page_info.has_next_page {
                break;
            }
            after_cursor = page_info.end_cursor;
            println!("Proceeding to next page of issues...");
            // Optional: Add a small delay if required by the provider's rate limits
            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // 4. Save the fetched issues to the database via the issue_repository.
        if !all_issues.is_empty() {
            println!("Saving {} issues to the database...", all_issues.len());
            self.issue_repository
                .save_issues(all_issues)
                .await
                .map_err(ApplicationError::RepositoryError)?;
            println!("Successfully saved issues for project #{}", project_number);
        } else {
            println!("No issues to save for project #{}", project_number);
        }

        Ok(())
    }
}
