use async_trait::async_trait;
use reqwest::{Client as ReqwestClient, RequestBuilder};
use std::time::Duration;
use thiserror::Error;

pub mod xdr;
pub mod console;
pub mod error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid API configuration: {0}")]
    ConfigError(String),
    #[error("API response error: {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub api_token: String,
    pub base_url: String,
    pub timeout: Duration,
}

impl ClientConfig {
    pub fn new(api_token: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_token: api_token.into(),
            base_url: base_url.into(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait]
pub trait ApiClient {
    async fn request(&self, builder: RequestBuilder) -> Result<reqwest::Response>;
}

pub struct BaseClient {
    client: ReqwestClient,
    config: ClientConfig,
}

impl BaseClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(config.timeout)
            .build()
            .map_err(Error::RequestError)?;

        Ok(Self { client, config })
    }

    fn build_url(&self, path: &str) -> Result<String> {
        let base = self.config.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        Ok(format!("{}/{}", base, path))
    }
}

#[async_trait]
impl ApiClient for BaseClient {
    async fn request(&self, builder: RequestBuilder) -> Result<reqwest::Response> {
        let response = builder
            .header("Authorization", format!("ApiToken {}", self.config.api_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Error::ApiError(error_text));
        }

        Ok(response)
    }
}
