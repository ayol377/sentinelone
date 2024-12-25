use crate::{ApiClient, BaseClient, Result};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Activities {
    base: BaseClient,
}

impl Activities {
    pub fn new(base: BaseClient) -> Self {
        Self { base }
    }   

    pub async fn get_activities(&self, lookback: u32, types: Vec<String>) -> Result<Vec<Value>> {
        let mut types_string = String::new();
        // if !types.is_empty() {
        //     for atype in types {
        //         types_string.push_str(&format!(",{}", atype));
        //     }
        // }

        types_string = types.join(",");
        println!("types_string={}", types_string);

        let mut params = GetActivityParams{
            cursor: "".to_string(),
        };
        let mut data = Vec::new();
        let url = self.base.build_url("/web/api/v2.1/activities")?;
        let now = Utc::now();
        let lookback_time = now - Duration::seconds(lookback as i64);
        let lookback_str = lookback_time.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        loop {
            let response = self.base
                .request(
                    self.base.client
                        .get(&url)
                        .header("Content-Type", "application/json")
                        .query(&[("cursor", &params.cursor), ("createdAt__gte", &lookback_str), ("activityTypes", &types_string)])
                )
                .await?;
            let response_text = response.text().await?;
            let response = serde_json::from_str::<GetActivityResponse>(&response_text).unwrap();
            data.extend(response.data);
            println!("INFO: GOT {} ACTIVITIES", data.len());
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
struct GetActivityParams {
    pub cursor: String,
}

#[derive(Debug, Deserialize)]
struct GetActivityResponse {
    pub data: Vec<Value>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    pub next_cursor: Option<String>,
}