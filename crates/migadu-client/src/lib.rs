//! Rust client for the Migadu email hosting API.
//!
//! # Example
//!
//! ```no_run
//! use migadu_client::{MigaduClient, CreateMailbox};
//!
//! #[tokio::main]
//! async fn main() -> migadu_client::Result<()> {
//!     let client = MigaduClient::new("user@example.com", "api-key");
//!
//!     // List all mailboxes
//!     let mailboxes = client.list_mailboxes("example.com").await?;
//!     for mailbox in mailboxes {
//!         println!("{}: {}", mailbox.address, mailbox.name);
//!     }
//!
//!     // Create a new mailbox
//!     let new_mailbox = CreateMailbox::new("demo", "Demo User", "secure-password");
//!     let mailbox = client.create_mailbox("example.com", &new_mailbox).await?;
//!     println!("Created: {}", mailbox.address);
//!
//!     Ok(())
//! }
//! ```

mod api;
mod client;
mod error;
mod types;

pub use client::MigaduClient;
pub use error::{Error, Result};
pub use types::*;
