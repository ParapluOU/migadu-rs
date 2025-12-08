//! Integration tests for the Migadu API client.
//!
//! These tests run against the real Migadu API and require valid credentials.
//! Set the following environment variables:
//! - MIGADU_EMAIL: Your Migadu account email
//! - MIGADU_API_KEY: Your Migadu API key
//! - MIGADU_DOMAIN: The domain to test with

use migadu_client::{
    CreateAlias, CreateIdentity, CreateMailbox, CreateRewrite, MigaduClient, UpdateAlias,
    UpdateIdentity, UpdateMailbox, UpdateRewrite,
};

fn get_client() -> MigaduClient {
    let email = std::env::var("MIGADU_EMAIL").expect("MIGADU_EMAIL environment variable required");
    let api_key =
        std::env::var("MIGADU_API_KEY").expect("MIGADU_API_KEY environment variable required");
    MigaduClient::new(email, api_key)
}

fn get_domain() -> String {
    std::env::var("MIGADU_DOMAIN").expect("MIGADU_DOMAIN environment variable required")
}

// ============================================================================
// Mailbox Tests
// ============================================================================

mod mailboxes {
    use super::*;

    #[tokio::test]
    async fn test_list_mailboxes() {
        let client = get_client();
        let domain = get_domain();

        let result = client.list_mailboxes(&domain).await;
        assert!(result.is_ok(), "Failed to list mailboxes: {:?}", result.err());

        let mailboxes = result.unwrap();
        println!("Found {} mailboxes", mailboxes.len());
        for mb in &mailboxes {
            println!("  - {} ({})", mb.address, mb.name);
        }
    }

    #[tokio::test]
    async fn test_mailbox_crud() {
        let client = get_client();
        let domain = get_domain();
        let local_part = "test-mailbox-crud";

        // Clean up any existing test mailbox
        let _ = client.delete_mailbox(&domain, local_part).await;

        // CREATE
        let create = CreateMailbox::new(local_part, "Test CRUD Mailbox", "SecurePass123!");
        let created = client.create_mailbox(&domain, &create).await;
        assert!(created.is_ok(), "Failed to create mailbox: {:?}", created.err());
        let created = created.unwrap();
        assert_eq!(created.local_part, local_part);
        assert_eq!(created.name, "Test CRUD Mailbox");
        println!("Created mailbox: {}", created.address);

        // READ
        let fetched = client.get_mailbox(&domain, local_part).await;
        assert!(fetched.is_ok(), "Failed to get mailbox: {:?}", fetched.err());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.address, created.address);
        println!("Fetched mailbox: {}", fetched.address);

        // UPDATE
        let update = UpdateMailbox {
            name: Some("Updated CRUD Mailbox".into()),
            ..Default::default()
        };
        let updated = client.update_mailbox(&domain, local_part, &update).await;
        assert!(updated.is_ok(), "Failed to update mailbox: {:?}", updated.err());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "Updated CRUD Mailbox");
        println!("Updated mailbox name to: {}", updated.name);

        // DELETE
        let deleted = client.delete_mailbox(&domain, local_part).await;
        assert!(deleted.is_ok(), "Failed to delete mailbox: {:?}", deleted.err());
        println!("Deleted mailbox: {}", deleted.unwrap().address);

        // Verify deletion
        let verify = client.get_mailbox(&domain, local_part).await;
        assert!(verify.is_err(), "Mailbox should not exist after deletion");
    }
}

// ============================================================================
// Identity Tests
// ============================================================================

mod identities {
    use super::*;

    #[tokio::test]
    async fn test_identity_crud() {
        let client = get_client();
        let domain = get_domain();
        let mailbox_local = "test-identity-mb";
        let identity_local = "test-identity";

        // Setup: Create a mailbox for testing identities
        let _ = client.delete_mailbox(&domain, mailbox_local).await;
        let mb = CreateMailbox::new(mailbox_local, "Identity Test MB", "SecurePass123!");
        let mb_result = client.create_mailbox(&domain, &mb).await;
        assert!(mb_result.is_ok(), "Failed to create mailbox: {:?}", mb_result.err());
        println!("Created mailbox for identity test: {}", mailbox_local);

        // LIST (should be empty initially, or just the mailbox itself)
        let list = client.list_identities(&domain, mailbox_local).await;
        assert!(list.is_ok(), "Failed to list identities: {:?}", list.err());
        println!("Initial identities: {}", list.unwrap().len());

        // CREATE
        let create = CreateIdentity::new(identity_local, "Test Identity");
        let created = client.create_identity(&domain, mailbox_local, &create).await;
        assert!(created.is_ok(), "Failed to create identity: {:?}", created.err());
        let created = created.unwrap();
        assert_eq!(created.local_part, identity_local);
        println!("Created identity: {}", created.address);

        // READ
        let fetched = client.get_identity(&domain, mailbox_local, identity_local).await;
        assert!(fetched.is_ok(), "Failed to get identity: {:?}", fetched.err());
        println!("Fetched identity: {}", fetched.unwrap().address);

        // UPDATE
        let update = UpdateIdentity {
            name: Some("Updated Identity".into()),
            ..Default::default()
        };
        let updated = client
            .update_identity(&domain, mailbox_local, identity_local, &update)
            .await;
        assert!(updated.is_ok(), "Failed to update identity: {:?}", updated.err());
        assert_eq!(updated.unwrap().name, "Updated Identity");
        println!("Updated identity name");

        // DELETE
        let deleted = client
            .delete_identity(&domain, mailbox_local, identity_local)
            .await;
        assert!(deleted.is_ok(), "Failed to delete identity: {:?}", deleted.err());
        println!("Deleted identity");

        // Cleanup: Delete the mailbox
        let _ = client.delete_mailbox(&domain, mailbox_local).await;
    }
}

// ============================================================================
// Alias Tests
// ============================================================================

mod aliases {
    use super::*;

    #[tokio::test]
    async fn test_list_aliases() {
        let client = get_client();
        let domain = get_domain();

        let result = client.list_aliases(&domain).await;
        assert!(result.is_ok(), "Failed to list aliases: {:?}", result.err());

        let aliases = result.unwrap();
        println!("Found {} aliases", aliases.len());
        for alias in &aliases {
            println!("  - {} -> {:?}", alias.address, alias.destinations);
        }
    }

    #[tokio::test]
    async fn test_alias_crud() {
        let client = get_client();
        let domain = get_domain();
        let local_part = "test-alias-crud";

        // Clean up any existing test alias
        let _ = client.delete_alias(&domain, local_part).await;

        // CREATE
        let create = CreateAlias::new(local_part, format!("admin@{}", domain));
        let created = client.create_alias(&domain, &create).await;
        assert!(created.is_ok(), "Failed to create alias: {:?}", created.err());
        let created = created.unwrap();
        assert_eq!(created.local_part, local_part);
        println!("Created alias: {} -> {:?}", created.address, created.destinations);

        // READ
        let fetched = client.get_alias(&domain, local_part).await;
        assert!(fetched.is_ok(), "Failed to get alias: {:?}", fetched.err());
        println!("Fetched alias: {}", fetched.unwrap().address);

        // UPDATE
        let update = UpdateAlias {
            destinations: Some(format!("postmaster@{}", domain)),
            ..Default::default()
        };
        let updated = client.update_alias(&domain, local_part, &update).await;
        assert!(updated.is_ok(), "Failed to update alias: {:?}", updated.err());
        println!("Updated alias destinations: {:?}", updated.unwrap().destinations);

        // DELETE
        let deleted = client.delete_alias(&domain, local_part).await;
        assert!(deleted.is_ok(), "Failed to delete alias: {:?}", deleted.err());
        println!("Deleted alias: {}", deleted.unwrap().address);
    }
}

// ============================================================================
// Rewrite Tests
// ============================================================================

mod rewrites {
    use super::*;

    #[tokio::test]
    async fn test_list_rewrites() {
        let client = get_client();
        let domain = get_domain();

        let result = client.list_rewrites(&domain).await;
        assert!(result.is_ok(), "Failed to list rewrites: {:?}", result.err());

        let rewrites = result.unwrap();
        println!("Found {} rewrites", rewrites.len());
        for rw in &rewrites {
            println!("  - {} ({}) -> {:?}", rw.name, rw.local_part_rule, rw.destinations);
        }
    }

    #[tokio::test]
    async fn test_rewrite_crud() {
        let client = get_client();
        let domain = get_domain();
        let name = "test-rewrite-crud";

        // Clean up any existing test rewrite
        let _ = client.delete_rewrite(&domain, name).await;

        // CREATE
        let create = CreateRewrite::new(name, "support-*", format!("admin@{}", domain));
        let created = client.create_rewrite(&domain, &create).await;
        assert!(created.is_ok(), "Failed to create rewrite: {:?}", created.err());
        let created = created.unwrap();
        assert_eq!(created.name, name);
        println!(
            "Created rewrite: {} ({}) -> {:?}",
            created.name, created.local_part_rule, created.destinations
        );

        // READ
        let fetched = client.get_rewrite(&domain, name).await;
        assert!(fetched.is_ok(), "Failed to get rewrite: {:?}", fetched.err());
        println!("Fetched rewrite: {}", fetched.unwrap().name);

        // UPDATE
        let update = UpdateRewrite {
            local_part_rule: Some("helpdesk-*".into()),
            ..Default::default()
        };
        let updated = client.update_rewrite(&domain, name, &update).await;
        assert!(updated.is_ok(), "Failed to update rewrite: {:?}", updated.err());
        println!("Updated rewrite rule to: {}", updated.unwrap().local_part_rule);

        // DELETE
        let deleted = client.delete_rewrite(&domain, name).await;
        assert!(deleted.is_ok(), "Failed to delete rewrite: {:?}", deleted.err());
        println!("Deleted rewrite: {}", deleted.unwrap().name);
    }
}
