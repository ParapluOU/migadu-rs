use serde::Deserialize;

use crate::client::MigaduClient;
use crate::error::Result;
use crate::types::{CreateMailbox, Mailbox, UpdateMailbox};

#[derive(Deserialize)]
struct MailboxesResponse {
    mailboxes: Vec<Mailbox>,
}

impl MigaduClient {
    /// Lists all mailboxes for a domain.
    pub async fn list_mailboxes(&self, domain: &str) -> Result<Vec<Mailbox>> {
        let path = format!("/domains/{}/mailboxes", domain);
        let response: MailboxesResponse = self.get(&path).await?;
        Ok(response.mailboxes)
    }

    /// Gets a specific mailbox by its local part.
    pub async fn get_mailbox(&self, domain: &str, local_part: &str) -> Result<Mailbox> {
        let path = format!("/domains/{}/mailboxes/{}", domain, local_part);
        self.get(&path).await
    }

    /// Creates a new mailbox.
    pub async fn create_mailbox(&self, domain: &str, mailbox: &CreateMailbox) -> Result<Mailbox> {
        let path = format!("/domains/{}/mailboxes", domain);
        self.post(&path, mailbox).await
    }

    /// Updates an existing mailbox.
    pub async fn update_mailbox(
        &self,
        domain: &str,
        local_part: &str,
        update: &UpdateMailbox,
    ) -> Result<Mailbox> {
        let path = format!("/domains/{}/mailboxes/{}", domain, local_part);
        self.put(&path, update).await
    }

    /// Deletes a mailbox.
    pub async fn delete_mailbox(&self, domain: &str, local_part: &str) -> Result<Mailbox> {
        let path = format!("/domains/{}/mailboxes/{}", domain, local_part);
        self.delete(&path).await
    }
}
