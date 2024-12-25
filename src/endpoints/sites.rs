use crate::{ApiClient, BaseClient, Result};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Sites {
    base: BaseClient,
}

impl Sites {
    pub fn new(base: BaseClient) -> Self {
        Self { base }
    }

    pub async fn get_sites(&self) -> Result<Vec<Value>> {
        let mut params = GetSitesParams{
            cursor: "".to_string(),
            limit: 100.to_string(),
        };
        let mut data = Vec::new();
        let url = self.base.build_url("/web/api/v2.1/sites")?;
        loop {
            let response = self.base
                .request(
                    self.base.client
                        .get(&url)
                        .header("Content-Type", "application/json")
                        .query(&[("cursor", &params.cursor), ("limit", &params.limit)])
                )
                .await?;
            let response_text = response.text().await?;
            let response = serde_json::from_str::<GetSitesResponse>(&response_text).unwrap();
            data.extend(response.data.sites);
            println!("INFO: GOT {} SITES", data.len());
            if response.pagination.next_cursor.is_none() {
                break;
            } else if Some(params.cursor.clone()) == response.pagination.next_cursor {
                println!("DEBUG: Error with pagination");
                break;
            }
            params.cursor = response.pagination.next_cursor.unwrap_or_default();
        }

        Ok(data)
    }
}

#[derive(Debug, Serialize)]
struct GetSitesParams {
    pub cursor: String,
    pub limit: String,
}

#[derive(Debug, Deserialize)]
struct GetSitesResponse {
    pub data: SitesData,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SitesData {
    pub sites: Vec<Value>,
}