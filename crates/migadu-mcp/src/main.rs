//! MCP server for Migadu email hosting API.

use async_trait::async_trait;
use migadu_client::MigaduClient;
use rust_mcp_sdk::mcp_server::{server_runtime, ServerHandler};
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ContentBlock, Implementation, InitializeResult,
    ListToolsRequest, ListToolsResult, RpcError, ServerCapabilities, ServerCapabilitiesTools,
    TextContent, Tool, ToolInputSchema,
};
use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::{McpServer, StdioTransport, TransportOptions};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

struct MigaduMcpServer {
    client: RwLock<Option<MigaduClient>>,
    domain: RwLock<Option<String>>,
}

impl MigaduMcpServer {
    fn new() -> Self {
        // Try to initialize from env vars
        let client = match (env::var("MIGADU_EMAIL"), env::var("MIGADU_API_KEY")) {
            (Ok(email), Ok(api_key)) => Some(MigaduClient::new(email, api_key)),
            _ => None,
        };
        let domain = env::var("MIGADU_DOMAIN").ok();

        Self {
            client: RwLock::new(client),
            domain: RwLock::new(domain),
        }
    }

    async fn get_client(&self) -> Result<MigaduClient, String> {
        self.client
            .read()
            .await
            .clone()
            .ok_or_else(|| "Not configured. Set MIGADU_EMAIL and MIGADU_API_KEY env vars, or call 'configure' tool.".to_string())
    }

    async fn get_domain(&self) -> Result<String, String> {
        self.domain
            .read()
            .await
            .clone()
            .ok_or_else(|| "Domain not configured. Set MIGADU_DOMAIN env var, or call 'configure' tool.".to_string())
    }

    fn make_tool(name: &str, description: &str, properties: HashMap<String, serde_json::Map<String, serde_json::Value>>, required: Vec<String>) -> Tool {
        Tool {
            name: name.to_string(),
            description: Some(description.to_string()),
            input_schema: ToolInputSchema::new(required, Some(properties)),
            annotations: None,
            meta: None,
            output_schema: None,
            title: None,
        }
    }

    fn tools() -> Vec<Tool> {
        vec![
            Self::make_tool(
                "configure",
                "Configure the Migadu API client credentials",
                {
                    let mut props = HashMap::new();
                    props.insert("email".to_string(), {
                        let mut m = serde_json::Map::new();
                        m.insert("type".to_string(), serde_json::json!("string"));
                        m.insert("description".to_string(), serde_json::json!("Migadu account email"));
                        m
                    });
                    props.insert("api_key".to_string(), {
                        let mut m = serde_json::Map::new();
                        m.insert("type".to_string(), serde_json::json!("string"));
                        m.insert("description".to_string(), serde_json::json!("Migadu API key"));
                        m
                    });
                    props.insert("domain".to_string(), {
                        let mut m = serde_json::Map::new();
                        m.insert("type".to_string(), serde_json::json!("string"));
                        m.insert("description".to_string(), serde_json::json!("Domain to operate on"));
                        m
                    });
                    props
                },
                vec!["email".to_string(), "api_key".to_string(), "domain".to_string()],
            ),
            Self::make_tool(
                "list_mailboxes",
                "List all mailboxes for the configured domain",
                HashMap::new(),
                vec![],
            ),
            Self::make_tool(
                "get_mailbox",
                "Get details of a specific mailbox",
                {
                    let mut props = HashMap::new();
                    props.insert("local_part".to_string(), {
                        let mut m = serde_json::Map::new();
                        m.insert("type".to_string(), serde_json::json!("string"));
                        m.insert("description".to_string(), serde_json::json!("Local part of the email (before @)"));
                        m
                    });
                    props
                },
                vec!["local_part".to_string()],
            ),
            Self::make_tool(
                "list_aliases",
                "List all email aliases for the configured domain",
                HashMap::new(),
                vec![],
            ),
            Self::make_tool(
                "list_rewrites",
                "List all rewrite rules for the configured domain",
                HashMap::new(),
                vec![],
            ),
        ]
    }

    fn text_result(text: String, is_error: bool) -> CallToolResult {
        CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent::new(text, None, None))],
            is_error: Some(is_error),
            meta: None,
            structured_content: None,
        }
    }

    fn error_result(msg: String) -> CallToolResult {
        Self::text_result(msg, true)
    }

    fn success_result(text: String) -> CallToolResult {
        Self::text_result(text, false)
    }
}

#[async_trait]
impl ServerHandler for MigaduMcpServer {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: Arc<dyn McpServer>,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: Self::tools(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: Arc<dyn McpServer>,
    ) -> Result<CallToolResult, CallToolError> {
        let args = request.params.arguments.unwrap_or_default();
        let tool_name = &request.params.name;

        match tool_name.as_str() {
            "configure" => {
                let email = args.get("email").and_then(|v| v.as_str()).unwrap_or("");
                let api_key = args.get("api_key").and_then(|v| v.as_str()).unwrap_or("");
                let domain = args.get("domain").and_then(|v| v.as_str()).unwrap_or("");

                if email.is_empty() || api_key.is_empty() || domain.is_empty() {
                    return Ok(Self::error_result("Missing required parameters: email, api_key, domain".to_string()));
                }

                *self.client.write().await = Some(MigaduClient::new(email, api_key));
                *self.domain.write().await = Some(domain.to_string());

                Ok(Self::success_result(format!("Configured Migadu client for domain: {}", domain)))
            }

            "list_mailboxes" => {
                let client = match self.get_client().await {
                    Ok(c) => c,
                    Err(e) => return Ok(Self::error_result(e)),
                };
                let domain = match self.get_domain().await {
                    Ok(d) => d,
                    Err(e) => return Ok(Self::error_result(e)),
                };

                match client.list_mailboxes(&domain).await {
                    Ok(mailboxes) => {
                        let result: Vec<_> = mailboxes
                            .iter()
                            .map(|m| serde_json::json!({"address": m.address, "name": m.name}))
                            .collect();
                        Ok(Self::success_result(serde_json::to_string_pretty(&result).unwrap()))
                    }
                    Err(e) => Ok(Self::error_result(e.to_string())),
                }
            }

            "get_mailbox" => {
                let local_part = args.get("local_part").and_then(|v| v.as_str()).unwrap_or("");
                if local_part.is_empty() {
                    return Ok(Self::error_result("Missing required parameter: local_part".to_string()));
                }

                let client = match self.get_client().await {
                    Ok(c) => c,
                    Err(e) => return Ok(Self::error_result(e)),
                };
                let domain = match self.get_domain().await {
                    Ok(d) => d,
                    Err(e) => return Ok(Self::error_result(e)),
                };

                match client.get_mailbox(&domain, local_part).await {
                    Ok(mailbox) => Ok(Self::success_result(serde_json::to_string_pretty(&mailbox).unwrap())),
                    Err(e) => Ok(Self::error_result(e.to_string())),
                }
            }

            "list_aliases" => {
                let client = match self.get_client().await {
                    Ok(c) => c,
                    Err(e) => return Ok(Self::error_result(e)),
                };
                let domain = match self.get_domain().await {
                    Ok(d) => d,
                    Err(e) => return Ok(Self::error_result(e)),
                };

                match client.list_aliases(&domain).await {
                    Ok(aliases) => {
                        let result: Vec<_> = aliases
                            .iter()
                            .map(|a| serde_json::json!({"address": a.address, "destinations": a.destinations}))
                            .collect();
                        Ok(Self::success_result(serde_json::to_string_pretty(&result).unwrap()))
                    }
                    Err(e) => Ok(Self::error_result(e.to_string())),
                }
            }

            "list_rewrites" => {
                let client = match self.get_client().await {
                    Ok(c) => c,
                    Err(e) => return Ok(Self::error_result(e)),
                };
                let domain = match self.get_domain().await {
                    Ok(d) => d,
                    Err(e) => return Ok(Self::error_result(e)),
                };

                match client.list_rewrites(&domain).await {
                    Ok(rewrites) => {
                        let result: Vec<_> = rewrites
                            .iter()
                            .map(|r| serde_json::json!({
                                "name": r.name,
                                "local_part_rule": r.local_part_rule,
                                "destinations": r.destinations
                            }))
                            .collect();
                        Ok(Self::success_result(serde_json::to_string_pretty(&result).unwrap()))
                    }
                    Err(e) => Ok(Self::error_result(e.to_string())),
                }
            }

            _ => Ok(CallToolError::unknown_tool(format!("Unknown tool: {}", tool_name)).into()),
        }
    }
}

fn server_info() -> InitializeResult {
    InitializeResult {
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools {
                list_changed: Some(false),
            }),
            ..Default::default()
        },
        instructions: Some("Migadu email hosting API server. Configure with MIGADU_EMAIL, MIGADU_API_KEY, and MIGADU_DOMAIN environment variables, or use the 'configure' tool.".into()),
        meta: None,
        protocol_version: rust_mcp_sdk::schema::LATEST_PROTOCOL_VERSION.to_string(),
        server_info: Implementation {
            name: "migadu-mcp".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            title: Some("Migadu MCP Server".into()),
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = MigaduMcpServer::new();
    let transport = StdioTransport::new(TransportOptions::default())?;

    let server = server_runtime::create_server(server_info(), transport, handler);
    server.start().await?;

    Ok(())
}
