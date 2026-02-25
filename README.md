# migadu-rs

Rust client library and tools for the [Migadu](https://www.migadu.com/) email hosting API.

> ### Looking for IT services?
> <img src="https://fromulo.com/codesociety.png" align="left" width="80" alt="CodeSociety">
>
> **[CodeSociety](https://codesocietyhub.com/)** is our consulting & contracting arm — specializing in
> **IT architecture**, **XML authoring systems**, **FontoXML integration**, and **TerminusDB consulting**.
> We build structured content platforms and data solutions that power digital publishing.
>
> **[Let's talk! &#8594;](https://codesocietyhub.com/contact.html)**

## Crates

| Crate | Description |
|-------|-------------|
| `migadu-client` | Core API client library |
| `migadu-cli` | Command-line interface |
| `migadu-mcp` | MCP server for LLM tool integration |

## Installation

```bash
# CLI
cargo install --path crates/migadu-cli

# MCP server
cargo install --path crates/migadu-mcp
```

## Library Usage

```rust
use migadu_client::{MigaduClient, CreateMailbox};

#[tokio::main]
async fn main() -> migadu_client::Result<()> {
    let client = MigaduClient::new("user@example.com", "api-key");

    // List all mailboxes
    let mailboxes = client.list_mailboxes("example.com").await?;
    for mailbox in mailboxes {
        println!("{}: {}", mailbox.address, mailbox.name);
    }

    // Create a new mailbox
    let new_mailbox = CreateMailbox::new("demo", "Demo User", "secure-password");
    let mailbox = client.create_mailbox("example.com", &new_mailbox).await?;
    println!("Created: {}", mailbox.address);

    Ok(())
}
```

## CLI Usage

```bash
# Set credentials
export MIGADU_EMAIL="your-email@example.com"
export MIGADU_API_KEY="your-api-key"
export MIGADU_DOMAIN="your-domain.com"

# Or use flags
migadu --email user@example.com --api-key xxx --domain example.com mailboxes list

# Mailboxes
migadu mailboxes list
migadu mailboxes get admin
migadu mailboxes create demo "Demo User" "password123"
migadu mailboxes update demo --name "New Name"
migadu mailboxes delete demo

# Aliases
migadu aliases list
migadu aliases create support "admin@example.com"
migadu aliases update support --destinations "team@example.com"
migadu aliases delete support

# Rewrites
migadu rewrites list
migadu rewrites create catchall "support-*" "admin@example.com"
migadu rewrites delete catchall

# Identities (per mailbox)
migadu identities list admin
migadu identities create admin sales "Sales Team"
migadu identities delete admin sales
```

## MCP Server

The MCP server exposes Migadu operations as tools for LLM integrations.

```bash
# Set credentials via environment
export MIGADU_EMAIL="your-email@example.com"
export MIGADU_API_KEY="your-api-key"
export MIGADU_DOMAIN="your-domain.com"

# Run the server
migadu-mcp
```

Available tools:
- `configure` - Set credentials at runtime
- `list_mailboxes` - List all mailboxes
- `get_mailbox` - Get mailbox details
- `list_aliases` - List all aliases
- `list_rewrites` - List all rewrite rules

## API Coverage

| Resource | List | Get | Create | Update | Delete |
|----------|------|-----|--------|--------|--------|
| Mailboxes | ✓ | ✓ | ✓ | ✓ | ✓ |
| Aliases | ✓ | ✓ | ✓ | ✓ | ✓ |
| Rewrites | ✓ | ✓ | ✓ | ✓ | ✓ |
| Identities | ✓ | ✓ | ✓ | ✓ | ✓ |
| Forwardings | ✓ | ✓ | ✓ | ✓ | ✓ |

## Getting API Credentials

1. Log in to your Migadu admin panel
2. Go to Account Settings → API Keys
3. Create a new API key

## License

MIT
