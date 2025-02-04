use std::time::Instant;

use crate::databases::mongodb::MongoDB;

use super::{db_metrics::DBMetrics, rune_pool_model::RunePoolResponse};

fn generate_rune_api_url(interval: &str, count: &str) -> String {
    format!(
        "https://midgard.ninerealms.com/v2/history/runepool?interval={}&count={}",
        interval, count
    )
}

#[derive(Clone)]
pub struct DBServices {
    mongodb: MongoDB,
}

impl DBServices {
    pub async fn init() -> Self {
        Self {
            mongodb: MongoDB::init().await,
        }
    }

    pub async fn mongo_fetch_rune(&self) -> DBMetrics {
        let start = Instant::now();
        let records = self.mongodb.read_rune_pool().await;
        let duration = start.elapsed();
        let metrics = DBMetrics {
            execution_time: duration.as_secs_f64(),
            total_records: records.len() as u64,
            operation: super::db_metrics::OperationType::Read,
        };
        metrics
    }

    pub async fn mongo_write_rune(&self) -> DBMetrics {
        let url = generate_rune_api_url("hour", "400");
        let response: RunePoolResponse = reqwest::get(url)
            .await
            .unwrap()
            .json::<RunePoolResponse>()
            .await
            .unwrap();
        let total_records = response.intervals.len() as u64;
        let start = Instant::now();
        for interval in response.intervals {
            self.mongodb.write_rune_pool(interval).await;
        }
        let duration = start.elapsed();
        let metrics = DBMetrics {
            execution_time: duration.as_secs_f64(),
            operation: super::db_metrics::OperationType::Insert,
            total_records,
        };
        metrics
    }
}
