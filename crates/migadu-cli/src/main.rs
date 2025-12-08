use clap::{Parser, Subcommand};
use migadu_client::{
    CreateAlias, CreateIdentity, CreateMailbox, CreateRewrite, MigaduClient, UpdateAlias,
    UpdateIdentity, UpdateMailbox, UpdateRewrite,
};

#[derive(Parser)]
#[command(name = "migadu")]
#[command(about = "CLI for Migadu email hosting API", long_about = None)]
struct Cli {
    /// Migadu account email
    #[arg(long, env = "MIGADU_EMAIL")]
    email: String,

    /// Migadu API key
    #[arg(long, env = "MIGADU_API_KEY")]
    api_key: String,

    /// Domain to operate on
    #[arg(long, env = "MIGADU_DOMAIN")]
    domain: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage mailboxes
    Mailboxes {
        #[command(subcommand)]
        action: MailboxAction,
    },
    /// Manage aliases
    Aliases {
        #[command(subcommand)]
        action: AliasAction,
    },
    /// Manage rewrites
    Rewrites {
        #[command(subcommand)]
        action: RewriteAction,
    },
    /// Manage identities
    Identities {
        #[command(subcommand)]
        action: IdentityAction,
    },
}

#[derive(Subcommand)]
enum MailboxAction {
    /// List all mailboxes
    List,
    /// Get a specific mailbox
    Get {
        /// Local part of the mailbox (e.g., "admin" for admin@domain.com)
        local_part: String,
    },
    /// Create a new mailbox
    Create {
        /// Local part
        local_part: String,
        /// Display name
        name: String,
        /// Password
        password: String,
    },
    /// Update a mailbox
    Update {
        /// Local part
        local_part: String,
        /// New display name
        #[arg(long)]
        name: Option<String>,
        /// New password
        #[arg(long)]
        password: Option<String>,
    },
    /// Delete a mailbox
    Delete {
        /// Local part
        local_part: String,
    },
}

#[derive(Subcommand)]
enum AliasAction {
    /// List all aliases
    List,
    /// Get a specific alias
    Get {
        /// Local part of the alias
        local_part: String,
    },
    /// Create a new alias
    Create {
        /// Local part
        local_part: String,
        /// Comma-separated list of destination addresses
        destinations: String,
    },
    /// Update an alias
    Update {
        /// Local part
        local_part: String,
        /// New comma-separated list of destination addresses
        #[arg(long)]
        destinations: Option<String>,
    },
    /// Delete an alias
    Delete {
        /// Local part
        local_part: String,
    },
}

#[derive(Subcommand)]
enum RewriteAction {
    /// List all rewrites
    List,
    /// Get a specific rewrite
    Get {
        /// Name of the rewrite rule
        name: String,
    },
    /// Create a new rewrite
    Create {
        /// Name (slug identifier)
        name: String,
        /// Pattern to match (e.g., "support-*")
        local_part_rule: String,
        /// Comma-separated list of destination addresses
        destinations: String,
    },
    /// Update a rewrite
    Update {
        /// Name
        name: String,
        /// New pattern
        #[arg(long)]
        local_part_rule: Option<String>,
        /// New destinations
        #[arg(long)]
        destinations: Option<String>,
    },
    /// Delete a rewrite
    Delete {
        /// Name
        name: String,
    },
}

#[derive(Subcommand)]
enum IdentityAction {
    /// List all identities for a mailbox
    List {
        /// Mailbox local part
        mailbox: String,
    },
    /// Get a specific identity
    Get {
        /// Mailbox local part
        mailbox: String,
        /// Identity local part
        identity: String,
    },
    /// Create a new identity
    Create {
        /// Mailbox local part
        mailbox: String,
        /// Identity local part
        local_part: String,
        /// Display name
        name: String,
    },
    /// Update an identity
    Update {
        /// Mailbox local part
        mailbox: String,
        /// Identity local part
        identity: String,
        /// New display name
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete an identity
    Delete {
        /// Mailbox local part
        mailbox: String,
        /// Identity local part
        identity: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = MigaduClient::new(&cli.email, &cli.api_key);
    let domain = &cli.domain;

    match cli.command {
        Commands::Mailboxes { action } => match action {
            MailboxAction::List => {
                let mailboxes = client.list_mailboxes(domain).await?;
                for mb in mailboxes {
                    println!("{}\t{}", mb.address, mb.name);
                }
            }
            MailboxAction::Get { local_part } => {
                let mb = client.get_mailbox(domain, &local_part).await?;
                println!("{}", serde_json::to_string_pretty(&mb)?);
            }
            MailboxAction::Create {
                local_part,
                name,
                password,
            } => {
                let create = CreateMailbox::new(&local_part, &name, &password);
                let mb = client.create_mailbox(domain, &create).await?;
                println!("Created: {}", mb.address);
            }
            MailboxAction::Update {
                local_part,
                name,
                password,
            } => {
                let update = UpdateMailbox {
                    name,
                    password,
                    ..Default::default()
                };
                let mb = client.update_mailbox(domain, &local_part, &update).await?;
                println!("Updated: {}", mb.address);
            }
            MailboxAction::Delete { local_part } => {
                let mb = client.delete_mailbox(domain, &local_part).await?;
                println!("Deleted: {}", mb.address);
            }
        },

        Commands::Aliases { action } => match action {
            AliasAction::List => {
                let aliases = client.list_aliases(domain).await?;
                for alias in aliases {
                    println!("{}\t{:?}", alias.address, alias.destinations);
                }
            }
            AliasAction::Get { local_part } => {
                let alias = client.get_alias(domain, &local_part).await?;
                println!("{}", serde_json::to_string_pretty(&alias)?);
            }
            AliasAction::Create {
                local_part,
                destinations,
            } => {
                let create = CreateAlias::new(&local_part, &destinations);
                let alias = client.create_alias(domain, &create).await?;
                println!("Created: {}", alias.address);
            }
            AliasAction::Update {
                local_part,
                destinations,
            } => {
                let update = UpdateAlias {
                    destinations,
                    ..Default::default()
                };
                let alias = client.update_alias(domain, &local_part, &update).await?;
                println!("Updated: {}", alias.address);
            }
            AliasAction::Delete { local_part } => {
                let alias = client.delete_alias(domain, &local_part).await?;
                println!("Deleted: {}", alias.address);
            }
        },

        Commands::Rewrites { action } => match action {
            RewriteAction::List => {
                let rewrites = client.list_rewrites(domain).await?;
                for rw in rewrites {
                    println!("{}\t{}\t{:?}", rw.name, rw.local_part_rule, rw.destinations);
                }
            }
            RewriteAction::Get { name } => {
                let rw = client.get_rewrite(domain, &name).await?;
                println!("{}", serde_json::to_string_pretty(&rw)?);
            }
            RewriteAction::Create {
                name,
                local_part_rule,
                destinations,
            } => {
                let create = CreateRewrite::new(&name, &local_part_rule, &destinations);
                let rw = client.create_rewrite(domain, &create).await?;
                println!("Created: {}", rw.name);
            }
            RewriteAction::Update {
                name,
                local_part_rule,
                destinations,
            } => {
                let update = UpdateRewrite {
                    local_part_rule,
                    destinations,
                    ..Default::default()
                };
                let rw = client.update_rewrite(domain, &name, &update).await?;
                println!("Updated: {}", rw.name);
            }
            RewriteAction::Delete { name } => {
                let rw = client.delete_rewrite(domain, &name).await?;
                println!("Deleted: {}", rw.name);
            }
        },

        Commands::Identities { action } => match action {
            IdentityAction::List { mailbox } => {
                let identities = client.list_identities(domain, &mailbox).await?;
                for id in identities {
                    println!("{}\t{}", id.address, id.name);
                }
            }
            IdentityAction::Get { mailbox, identity } => {
                let id = client.get_identity(domain, &mailbox, &identity).await?;
                println!("{}", serde_json::to_string_pretty(&id)?);
            }
            IdentityAction::Create {
                mailbox,
                local_part,
                name,
            } => {
                let create = CreateIdentity::new(&local_part, &name);
                let id = client.create_identity(domain, &mailbox, &create).await?;
                println!("Created: {}", id.address);
            }
            IdentityAction::Update {
                mailbox,
                identity,
                name,
            } => {
                let update = UpdateIdentity {
                    name,
                    ..Default::default()
                };
                let id = client
                    .update_identity(domain, &mailbox, &identity, &update)
                    .await?;
                println!("Updated: {}", id.address);
            }
            IdentityAction::Delete { mailbox, identity } => {
                let id = client.delete_identity(domain, &mailbox, &identity).await?;
                println!("Deleted: {}", id.address);
            }
        },
    }

    Ok(())
}
