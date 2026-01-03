use std::sync::Arc;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::tool::Parameters,
    model::*,
    tool, tool_handler, tool_router,
    ErrorData as McpError,
};

use crate::confluence::{ConfluenceClient, CreatePageRequest, UpdatePageRequest};
use crate::tools::{
    format_page, format_page_created, format_page_list, format_page_updated, CreatePageParams,
    GetPageParams, GetPagesBySpaceParams, UpdatePageParams,
};

#[derive(Clone)]
pub struct ConfluenceServer {
    confluence: Arc<ConfluenceClient>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ConfluenceServer {
    pub fn new(confluence: ConfluenceClient) -> Self {
        Self {
            confluence: Arc::new(confluence),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get a Confluence page by its ID. Returns the page title, content, version, and metadata.")]
    async fn get_page(
        &self,
        Parameters(params): Parameters<GetPageParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.confluence.get_page(&params.page_id).await {
            Ok(page) => {
                let output = format_page(&page);
                Ok(CallToolResult::success(vec![Content::text(output)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to get page: {}",
                e
            ))])),
        }
    }

    #[tool(description = "List pages in a Confluence space. Returns a list of pages with their titles, IDs, and basic metadata.")]
    async fn get_pages_by_space(
        &self,
        Parameters(params): Parameters<GetPagesBySpaceParams>,
    ) -> Result<CallToolResult, McpError> {
        let limit = params.limit.unwrap_or(25).min(250);

        match self
            .confluence
            .get_pages_by_space(&params.space_id, limit)
            .await
        {
            Ok(response) => {
                let output = format_page_list(&response, &params.space_id);
                Ok(CallToolResult::success(vec![Content::text(output)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to get pages: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Create a new Confluence page. Requires space ID, title, and content in Confluence storage format (HTML-like). Optionally specify a parent page ID.")]
    async fn create_page(
        &self,
        Parameters(params): Parameters<CreatePageParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut request = CreatePageRequest::new(params.space_id, params.title, params.body);

        if let Some(parent_id) = params.parent_id {
            request = request.parent_id(parent_id);
        }

        match self.confluence.create_page(request).await {
            Ok(page) => {
                let output = format_page_created(&page);
                Ok(CallToolResult::success(vec![Content::text(output)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to create page: {}",
                e
            ))])),
        }
    }

    #[tool(description = "Update an existing Confluence page. Requires page ID, new title, new content, and current version number. Optionally include a version message describing the changes.")]
    async fn update_page(
        &self,
        Parameters(params): Parameters<UpdatePageParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut request = UpdatePageRequest::new(
            params.page_id.clone(),
            params.title,
            params.body,
            params.version_number,
        );

        if let Some(message) = params.version_message {
            request = request.version_message(message);
        }

        match self.confluence.update_page(&params.page_id, request).await {
            Ok(page) => {
                let output = format_page_updated(&page);
                Ok(CallToolResult::success(vec![Content::text(output)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to update page: {}",
                e
            ))])),
        }
    }
}

#[tool_handler]
impl rmcp::ServerHandler for ConfluenceServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Confluence MCP Server - Read, create, and update Confluence pages".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
