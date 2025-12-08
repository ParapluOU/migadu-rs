# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build all crates
cargo build

# Check for errors (faster than build)
cargo check

# Run all tests (requires env vars, see Testing section)
cargo test

# Run tests for a specific crate
cargo test -p migadu-client

# Run a single test
cargo test -p migadu-client test_list_mailboxes

# Build the CLI binary
cargo build -p migadu-cli --release

# Build the MCP server
cargo build -p migadu-mcp --release
```

## Testing

Integration tests run against the real Migadu API and require credentials:

```bash
export MIGADU_EMAIL="your-email@example.com"
export MIGADU_API_KEY="your-api-key"
export MIGADU_DOMAIN="your-domain.com"
cargo test -p migadu-client
```

Tests are organized by resource type in modules: `mailboxes`, `aliases`, `rewrites`, `identities`.

## Architecture

This is a Cargo workspace with three crates:

### migadu-client (library)
Core API client library for the Migadu email hosting API. Structure:
- `client.rs` - `MigaduClient` struct with HTTP methods (get/post/put/delete) using reqwest
- `api/` - API endpoint implementations as trait extensions on MigaduClient (mailboxes, aliases, rewrites, identities, forwardings)
- `types/` - Request/response types (Create*, Update*, and response structs for each resource)
- `error.rs` - Error types using thiserror

API methods follow the pattern: `list_*`, `get_*`, `create_*`, `update_*`, `delete_*`

### migadu-cli (binary: `migadu`)
CLI using clap with subcommands: `mailboxes`, `aliases`, `rewrites`, `identities`. Each has actions: list, get, create, update, delete.

Credentials via `--email`/`--api-key`/`--domain` flags or `MIGADU_*` env vars.

### migadu-mcp (binary: `migadu-mcp`)
MCP (Model Context Protocol) server using rust-mcp-sdk. Exposes Migadu operations as MCP tools: `configure`, `list_mailboxes`, `get_mailbox`, `list_aliases`, `list_rewrites`.

Credentials via env vars or the `configure` tool at runtime.
