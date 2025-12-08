use serde::Deserialize;

use crate::client::MigaduClient;
use crate::error::Result;
use crate::types::{Alias, CreateAlias, UpdateAlias};

#[derive(Deserialize)]
struct AliasesResponse {
    address_aliases: Vec<Alias>,
}

impl MigaduClient {
    /// Lists all aliases for a domain.
    pub async fn list_aliases(&self, domain: &str) -> Result<Vec<Alias>> {
        let path = format!("/domains/{}/aliases", domain);
        let response: AliasesResponse = self.get(&path).await?;
        Ok(response.address_aliases)
    }

    /// Gets a specific alias by its local part.
    pub async fn get_alias(&self, domain: &str, local_part: &str) -> Result<Alias> {
        let path = format!("/domains/{}/aliases/{}", domain, local_part);
        self.get(&path).await
    }

    /// Creates a new alias.
    pub async fn create_alias(&self, domain: &str, alias: &CreateAlias) -> Result<Alias> {
        let path = format!("/domains/{}/aliases", domain);
        self.post(&path, alias).await
    }

    /// Updates an existing alias.
    pub async fn update_alias(
        &self,
        domain: &str,
        local_part: &str,
        update: &UpdateAlias,
    ) -> Result<Alias> {
        let path = format!("/domains/{}/aliases/{}", domain, local_part);
        self.put(&path, update).await
    }

    /// Deletes an alias.
    pub async fn delete_alias(&self, domain: &str, local_part: &str) -> Result<Alias> {
        let path = format!("/domains/{}/aliases/{}", domain, local_part);
        self.delete(&path).await
    }
}
