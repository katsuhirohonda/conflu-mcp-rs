mod confluence;
mod server;
mod tools;

use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};

use confluence::ConfluenceClient;
use server::ConfluenceServer;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let base_url = std::env::var("CONFLUENCE_BASE_URL")
        .expect("CONFLUENCE_BASE_URL environment variable is required");
    let email =
        std::env::var("CONFLUENCE_EMAIL").expect("CONFLUENCE_EMAIL environment variable is required");
    let api_token = std::env::var("CONFLUENCE_API_TOKEN")
        .expect("CONFLUENCE_API_TOKEN environment variable is required");

    let confluence = ConfluenceClient::new(&base_url, &email, &api_token);
    let server = ConfluenceServer::new(confluence);

    tracing::info!("Starting Confluence MCP server...");

    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
