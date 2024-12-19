use crate::{ApiClient, BaseClient, Result};
use serde::{Deserialize, Serialize};

pub struct Threats {
    base: BaseClient,
}

impl Threats {
    pub fn new(base: BaseClient) -> Self {
        Self { base }
    }

    pub async fn get_threats(&self) -> Result<Vec<Threat>> {
        let mut params = GetThreatsParams{
            cursor: "".to_string(),
        };
        let mut data = Vec::new();
        let url = self.base.build_url("/web/api/v2.1/threats")?;
        loop {
            let response = self.base
                .request(
                    self.base.client
                        .get(&url)
                        .header("Content-Type", "application/json")
                        .query(&params)
                )
                .await?;
            let response = serde_json::from_str::<GetThreatsResponse>(&response.text().await?).unwrap();
            data.extend(response.data);
            if response.pagination.next_cursor == "".to_string() {
                break;
            } else if params.cursor == response.pagination.next_cursor {
                println!("DEBUG: Error with pagination");
            }
            params.cursor = response.pagination.next_cursor;
        }

        Ok(data)
    }
}

#[derive(Debug, Serialize)]
pub struct GetThreatsParams {
    pub cursor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Threat {
    #[serde(flatten)]
    pub json: serde_json::Map<String, serde_json::Value>
}

#[derive(Debug, Deserialize)]
pub struct GetThreatsResponse {
    pub data: Vec<Threat>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub total_items: u32,
    pub next_cursor: String,
}