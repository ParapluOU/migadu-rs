use serde::Deserialize;

use crate::client::MigaduClient;
use crate::error::Result;
use crate::types::{CreateRewrite, Rewrite, UpdateRewrite};

#[derive(Deserialize)]
struct RewritesResponse {
    rewrites: Vec<Rewrite>,
}

impl MigaduClient {
    /// Lists all rewrite rules for a domain.
    pub async fn list_rewrites(&self, domain: &str) -> Result<Vec<Rewrite>> {
        let path = format!("/domains/{}/rewrites", domain);
        let response: RewritesResponse = self.get(&path).await?;
        Ok(response.rewrites)
    }

    /// Gets a specific rewrite rule by its name.
    pub async fn get_rewrite(&self, domain: &str, name: &str) -> Result<Rewrite> {
        let path = format!("/domains/{}/rewrites/{}", domain, name);
        self.get(&path).await
    }

    /// Creates a new rewrite rule.
    pub async fn create_rewrite(&self, domain: &str, rewrite: &CreateRewrite) -> Result<Rewrite> {
        let path = format!("/domains/{}/rewrites", domain);
        self.post(&path, rewrite).await
    }

    /// Updates an existing rewrite rule.
    pub async fn update_rewrite(
        &self,
        domain: &str,
        name: &str,
        update: &UpdateRewrite,
    ) -> Result<Rewrite> {
        let path = format!("/domains/{}/rewrites/{}", domain, name);
        self.put(&path, update).await
    }

    /// Deletes a rewrite rule.
    pub async fn delete_rewrite(&self, domain: &str, name: &str) -> Result<Rewrite> {
        let path = format!("/domains/{}/rewrites/{}", domain, name);
        self.delete(&path).await
    }
}
