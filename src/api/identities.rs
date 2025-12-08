use serde::Deserialize;

use crate::client::MigaduClient;
use crate::error::Result;
use crate::types::{CreateIdentity, Identity, UpdateIdentity};

#[derive(Deserialize)]
struct IdentitiesResponse {
    identities: Vec<Identity>,
}

impl MigaduClient {
    /// Lists all identities for a mailbox.
    pub async fn list_identities(
        &self,
        domain: &str,
        mailbox_local_part: &str,
    ) -> Result<Vec<Identity>> {
        let path = format!(
            "/domains/{}/mailboxes/{}/identities",
            domain, mailbox_local_part
        );
        let response: IdentitiesResponse = self.get(&path).await?;
        Ok(response.identities)
    }

    /// Gets a specific identity.
    pub async fn get_identity(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        identity_local_part: &str,
    ) -> Result<Identity> {
        let path = format!(
            "/domains/{}/mailboxes/{}/identities/{}",
            domain, mailbox_local_part, identity_local_part
        );
        self.get(&path).await
    }

    /// Creates a new identity for a mailbox.
    pub async fn create_identity(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        identity: &CreateIdentity,
    ) -> Result<Identity> {
        let path = format!(
            "/domains/{}/mailboxes/{}/identities",
            domain, mailbox_local_part
        );
        self.post(&path, identity).await
    }

    /// Updates an existing identity.
    pub async fn update_identity(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        identity_local_part: &str,
        update: &UpdateIdentity,
    ) -> Result<Identity> {
        let path = format!(
            "/domains/{}/mailboxes/{}/identities/{}",
            domain, mailbox_local_part, identity_local_part
        );
        self.put(&path, update).await
    }

    /// Deletes an identity.
    pub async fn delete_identity(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        identity_local_part: &str,
    ) -> Result<Identity> {
        let path = format!(
            "/domains/{}/mailboxes/{}/identities/{}",
            domain, mailbox_local_part, identity_local_part
        );
        self.delete(&path).await
    }
}
