use serde::Deserialize;

use crate::client::MigaduClient;
use crate::error::Result;
use crate::types::{CreateForwarding, Forwarding, UpdateForwarding};

#[derive(Deserialize)]
struct ForwardingsResponse {
    forwardings: Vec<Forwarding>,
}

impl MigaduClient {
    /// Lists all forwarding addresses for a mailbox.
    pub async fn list_forwardings(
        &self,
        domain: &str,
        mailbox_local_part: &str,
    ) -> Result<Vec<Forwarding>> {
        let path = format!(
            "/domains/{}/mailboxes/{}/forwardings",
            domain, mailbox_local_part
        );
        let response: ForwardingsResponse = self.get(&path).await?;
        Ok(response.forwardings)
    }

    /// Gets a specific forwarding address.
    pub async fn get_forwarding(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        address: &str,
    ) -> Result<Forwarding> {
        let path = format!(
            "/domains/{}/mailboxes/{}/forwardings/{}",
            domain, mailbox_local_part, address
        );
        self.get(&path).await
    }

    /// Creates a new forwarding address for a mailbox.
    pub async fn create_forwarding(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        forwarding: &CreateForwarding,
    ) -> Result<Forwarding> {
        let path = format!(
            "/domains/{}/mailboxes/{}/forwardings",
            domain, mailbox_local_part
        );
        self.post(&path, forwarding).await
    }

    /// Updates an existing forwarding address.
    pub async fn update_forwarding(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        address: &str,
        update: &UpdateForwarding,
    ) -> Result<Forwarding> {
        let path = format!(
            "/domains/{}/mailboxes/{}/forwardings/{}",
            domain, mailbox_local_part, address
        );
        self.put(&path, update).await
    }

    /// Deletes a forwarding address.
    pub async fn delete_forwarding(
        &self,
        domain: &str,
        mailbox_local_part: &str,
        address: &str,
    ) -> Result<Forwarding> {
        let path = format!(
            "/domains/{}/mailboxes/{}/forwardings/{}",
            domain, mailbox_local_part, address
        );
        self.delete(&path).await
    }
}
