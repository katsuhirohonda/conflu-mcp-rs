use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetPageParams {
    /// The page ID to retrieve
    pub page_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetPagesBySpaceParams {
    /// The space ID to retrieve pages from
    pub space_id: String,
    /// Maximum number of pages to return (default: 25, max: 250)
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePageParams {
    /// The space ID where the page will be created
    pub space_id: String,
    /// The title of the new page
    pub title: String,
    /// The body content in Confluence storage format (HTML-like format)
    pub body: String,
    /// Optional parent page ID
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdatePageParams {
    /// The page ID to update
    pub page_id: String,
    /// The new title for the page
    pub title: String,
    /// The new body content in Confluence storage format
    pub body: String,
    /// The current version number of the page (required for optimistic locking)
    pub version_number: u32,
    /// Optional version message describing the changes
    pub version_message: Option<String>,
}
