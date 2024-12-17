use crate::{ApiClient, BaseClient, ClientConfig, Result};
use serde::{Deserialize, Serialize};

pub struct XdrClient {
    base: BaseClient,
}

impl XdrClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            base: BaseClient::new(config)?,
        })
    }

    // Example method for getting threats
    pub async fn get_threats(&self, params: GetThreatsParams) -> Result<GetThreatsResponse> {
        let url = self.base.build_url("/web/api/v2.1/threats")?;
        let response = self.base
            .request(
                reqwest::Client::new()
                    .get(&url)
                    .query(&params)
            )
            .await?;

        Ok(response.json().await?)
    }

    // Example method for getting agents
    pub async fn get_agents(&self, params: GetAgentsParams) -> Result<GetAgentsResponse> {
        let url = self.base.build_url("/web/api/v2.1/agents")?;
        let response = self.base
            .request(
                reqwest::Client::new()
                    .get(&url)
                    .query(&params)
            )
            .await?;

        Ok(response.json().await?)
    }
}

#[derive(Debug, Serialize)]
pub struct GetThreatsParams {
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub resolved: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GetThreatsResponse {
    pub data: Vec<Threat>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Threat {
    pub id: String,
    pub threat_name: String,
    pub status: String,
    pub severity: String,
    // Add more fields as needed
}

#[derive(Debug, Serialize)]
pub struct GetAgentsParams {
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GetAgentsResponse {
    pub data: Vec<Agent>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub id: String,
    pub computer_name: String,
    pub is_active: bool,
    pub last_active_date: String,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub total_items: u32,
    pub limit: u32,
    pub skip: u32,
}
