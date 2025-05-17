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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    HttpServer::new(move || App::new())
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
