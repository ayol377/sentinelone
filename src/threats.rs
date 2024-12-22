use crate::{ApiClient, BaseClient, Result};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use serde_json::Value;

pub struct Threats {
    base: BaseClient,
}

impl Threats {
    pub fn new(base: BaseClient) -> Self {
        Self { base }
    }

    pub async fn get_threats(&self, lookback: u32) -> Result<Vec<Value>> {
        let mut params = GetThreatsParams{
            cursor: "".to_string(),
        };
        let mut data = Vec::new();
        let url = self.base.build_url("/web/api/v2.1/threats")?;
        let now = Utc::now();
        let lookback_time = now - Duration::seconds(lookback as i64);
        let lookback_str = lookback_time.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        loop {
            let response = self.base
                .request(
                    self.base.client
                        .get(&url)
                        .header("Content-Type", "application/json")
                        .query(&[("cursor", &params.cursor), ("createdAt__gte", &lookback_str)])
                )
                .await?;
            let response_text = response.text().await?;
            // println!("INFO: RESPONSE TEXT: {}", response_text);
            let response = serde_json::from_str::<GetThreatsResponse>(&response_text).unwrap();
            data.extend(response.data);
            if response.pagination.next_cursor.is_none() {
                break;
            } else if Some(params.cursor.clone()) == response.pagination.next_cursor {
                println!("DEBUG: Error with pagination");
                break;
            }
            println!("INFO: GOT {} THREATS", data.len());
            params.cursor = response.pagination.next_cursor.unwrap_or_default();
        }

        Ok(data)
    }
}

#[derive(Debug, Serialize)]
pub struct GetThreatsParams {
    pub cursor: String,
}

#[derive(Debug, Deserialize)]
pub struct GetThreatsResponse {
    pub data: Vec<Value>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub total_items: u32,
    pub next_cursor: Option<String>,
}