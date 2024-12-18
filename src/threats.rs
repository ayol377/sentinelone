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
            cursor: None,
        };
        let mut data = Vec::new();
        let url = self.base.build_url("/web/api/v2.1/threats")?;

        loop {
            #[cfg(debug_assertions)]
            {
                println!("DEBUG: Fetching threats");
            }
            let response = self.base
                .request(
                    self.base.client
                        .get(&url)
                        .query(&params)
                )
                .await?;

            let response = response.json::<GetThreatsResponse>().await?;

            data.extend(response.data);

            if response.pagination.next_cursor.is_empty() {
                break;
            }
            #[cfg(debug_assertions)]
            {
                println!("DEBUG: Paginating: {:?}", Some(&response.pagination.next_cursor));
            }
            params.cursor = Some(response.pagination.next_cursor);
        }

        Ok(data)
    }
}

#[derive(Debug, Serialize)]
pub struct GetThreatsParams {
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Threat {
    #[serde(flatten)]
    pub json: serde_json::Map<String, serde_json::Value>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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