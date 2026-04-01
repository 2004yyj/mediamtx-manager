pub mod config;
pub mod paths;

use reqwest::Client;

/// MediaMTX REST API v3 클라이언트
#[derive(Debug, Clone)]
pub struct MediaMtxClient {
    client: Client,
    base_url: String,
}

impl MediaMtxClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
