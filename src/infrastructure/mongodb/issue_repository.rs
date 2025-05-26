use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime}, // BsonDateTime not strictly needed here if From traits handle it
    options::InsertManyOptions,
    Collection, Database, // InsertManyResult is part of the output of insert_many directly
};
use anyhow::{Result, Context}; // anyhow is not used in the final snippet, but Context is.
use futures::TryStreamExt; // For cursor.try_next()

use crate::domain::entities::issue::Issue as DomainIssue;
use crate::domain::repositories::issue_repository::IssueRepository as DomainIssueRepository;
use super::issue_model::IssueModel;

pub struct IssueRepository {
    collection: Collection<IssueModel>,
}

impl IssueRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<IssueModel>("issues");
        Self { collection }
    }
}

#[async_trait]
impl DomainIssueRepository for IssueRepository {
    async fn save_issues(&self, domain_issues: Vec<DomainIssue>) -> Result<Vec<String>> {
        if domain_issues.is_empty() {
            return Ok(Vec::new());
        }

        // The From<DomainIssue> for IssueModel is already implemented in issue_model.rs
        let issue_models: Vec<IssueModel> = domain_issues
            .into_iter()
            .map(IssueModel::from) 
            .collect();

        let options = InsertManyOptions::builder().ordered(false).build();
        let result = self
            .collection
            .insert_many(issue_models)
            .with_options(options)
            .await
            .context("Failed to insert issues into MongoDB")?;

        let ids = result
            .inserted_ids
            .values()
            .map(|id| id.as_object_id().expect("Expected ObjectId from MongoDB insertion").to_hex())
            .collect();
        Ok(ids)
    }

    async fn get_issues_by_snapshot_id(&self, snapshot_id: ObjectId) -> Result<Vec<DomainIssue>> {
        let mut cursor = self
            .collection
            .find(doc! { "snapshot_id": snapshot_id }, None)
            .await
            .context(format!("Failed to find issues by snapshot_id: {}", snapshot_id))?;

        let mut issues = Vec::new();
        // The From<IssueModel> for DomainIssue is already implemented in issue_model.rs
        while let Some(model) = cursor
            .try_next()
            .await
            .context("Error iterating over issue models from MongoDB cursor")? 
        {
            issues.push(DomainIssue::from(model)); 
        }
        Ok(issues)
    }
}
