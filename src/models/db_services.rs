use std::time::Instant;

use crate::databases::mongodb::MongoDB;

use super::db_metrics::DBMetrics;

pub struct DBServices {
    mongodb: MongoDB,
}

impl DBServices {
    pub async fn init() -> Self {
        Self {
            mongodb: MongoDB::init().await,
        }
    }

    pub async fn mongo_fetch_rune(&self) {
        let start = Instant::now();
        let records = self.mongodb.read_rune_pool().await;
        let duration = start.elapsed();
        let metrics = DBMetrics {
            db_name: "MongoDB".to_string(),
            execution_time: duration.as_secs_f64(),
            total_records: records.len() as u64,
            operation: super::db_metrics::OperationType::Read,
        };
        println!("{:?}", metrics);
    }
}
