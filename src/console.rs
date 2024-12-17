use crate::{ApiClient, BaseClient, ClientConfig, Result};
use serde::{Deserialize, Serialize};

pub struct ConsoleClient {
    base: BaseClient,
}

impl ConsoleClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            base: BaseClient::new(config)?,
        })
    }

    // Example method for getting sites
    pub async fn get_sites(&self, params: GetSitesParams) -> Result<GetSitesResponse> {
        let url = self.base.build_url("/web/api/v2.1/sites")?;
        let response = self.base
            .request(
                reqwest::Client::new()
                    .get(&url)
                    .query(&params)
            )
            .await?;

        Ok(response.json().await?)
    }

    // Example method for getting users
    pub async fn get_users(&self, params: GetUsersParams) -> Result<GetUsersResponse> {
        let url = self.base.build_url("/web/api/v2.1/users")?;
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
pub struct GetSitesParams {
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetSitesResponse {
    pub data: Vec<Site>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    // Add more fields as needed
}

#[derive(Debug, Serialize)]
pub struct GetUsersParams {
    pub limit: Option<u32>,
    pub skip: Option<u32>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetUsersResponse {
    pub data: Vec<User>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub total_items: u32,
    pub limit: u32,
    pub skip: u32,
}
