use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use actix_web::{App, Error, HttpResponse, HttpServer, web};
use futures::{Stream, StreamExt};
use rmcp::transport::sse_server::SseServer;
use rmcp::transport::stdio;
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation, ServerInfo},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, mpsc};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

// Configuration for our MCP server connections
#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpServerConfig {
    name: String,
    url: String,
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    did: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    servers: Vec<McpServerConfig>,
    listen_addr: String, // eg 127.0.0.1:8000
}

// Store the active MCP server connections
struct McpServerConnection {
    config: McpServerConfig,
    server_info: Option<ServerInfo>,
    available_tools: Option<Vec<String>>, // List of available tool names
    status: ConnectionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ConnectionStatus {
    Connected,
    Disconnected,
    Error(String),
    Initializing,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    HttpServer::new(move || App::new())
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
