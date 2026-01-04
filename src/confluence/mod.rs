mod models;

pub use models::*;

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;

#[derive(Clone)]
pub struct ConfluenceClient {
    client: Client,
    base_url: String,
    auth_header: String,
}

impl ConfluenceClient {
    pub fn new(base_url: &str, email: &str, api_token: &str) -> Self {
        let credentials = format!("{}:{}", email, api_token);
        let auth_header = format!("Basic {}", STANDARD.encode(credentials));

        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_header,
        }
    }

    pub async fn get_page(&self, page_id: &str) -> Result<Page> {
        let url = format!(
            "{}/wiki/api/v2/pages/{}?body-format=storage",
            self.base_url, page_id
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", &self.auth_header)
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Confluence API error ({}): {}", status, error_text);
        }

        let text = response.text().await?;
        eprintln!("DEBUG get_page response: {}", text);
        let page = serde_json::from_str::<Page>(&text)?;
        Ok(page)
    }



    pub async fn create_page(&self, request: CreatePageRequest) -> Result<Page> {
        let url = format!("{}/wiki/api/v2/pages", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Confluence API error ({}): {}", status, error_text);
        }

        let page = response.json::<Page>().await?;
        Ok(page)
    }

    pub async fn update_page(&self, page_id: &str, request: UpdatePageRequest) -> Result<Page> {
        let url = format!("{}/wiki/api/v2/pages/{}", self.base_url, page_id);

        let response = self
            .client
            .put(&url)
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Confluence API error ({}): {}", status, error_text);
        }

        let page = response.json::<Page>().await?;
        Ok(page)
    }
}
