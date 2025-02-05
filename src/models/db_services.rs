use std::{future, time::Instant};

use crate::databases::{mongodb::MongoDB, postgresql::PostgreSQL};

use super::{
    db_metrics::{DBMetrics, DBPerformance},
    rune_pool_model::{RunePool, RunePoolResponse},
};

const DB_NAMES: [&str; 2] = ["mongodb", "postgresql"];
fn generate_rune_api_url(interval: &str, count: &str) -> String {
    format!(
        "https://midgard.ninerealms.com/v2/history/runepool?interval={}&count={}",
        interval, count
    )
}

#[derive(Clone)]
pub struct DBServices {
    mongodb: MongoDB,
    postgresql: PostgreSQL,
}

impl DBServices {
    pub async fn init() -> Self {
        Self {
            mongodb: MongoDB::init().await,
            postgresql: PostgreSQL::init().await,
        }
    }

    pub async fn read_metric<F, Fut>(read_fn: F) -> DBMetrics
    where
        F: FnOnce() -> Fut,
        Fut: future::Future<Output = Vec<RunePool>>,
    {
        let start = Instant::now();
        let records = read_fn().await;
        let duration = start.elapsed();
        let metrics = DBMetrics {
            execution_time: duration.as_secs_f64(),
            total_records: records.len() as u64,
            operation: super::db_metrics::OperationType::Read,
        };
        metrics
    }

    pub async fn get_db_metric(&self, db_name: &str) -> Vec<DBPerformance> {
        match db_name {
            "all" => {
                let mut performances: Vec<DBPerformance> = Vec::new();
                for db_name in DB_NAMES {
                    let fut = Box::pin(self.get_db_metric(db_name));
                    let performance = fut.await;
                    performances.push(performance[0].clone());
                }
                return performances;
            }
            "mongodb" => {
                let read_metrics = Self::read_metric(|| self.mongodb.read_rune_pool()).await;
                let write_metrics = Self::write_metrics(|interval: RunePool| async move {
                    self.mongodb.write_rune_pool(interval).await
                })
                .await;

                let performance = DBPerformance {
                    db_name: db_name.to_string(),
                    performance: vec![read_metrics, write_metrics],
                };
                vec![performance]
            }
            "postgresql" => {
                let read_metrics = Self::read_metric(|| self.postgresql.read_rune_pool()).await;
                let write_metrics = Self::write_metrics(|interval: RunePool| async move {
                    self.postgresql.write_rune_pool(interval).await
                })
                .await;
                let performance = DBPerformance {
                    db_name: db_name.to_string(),
                    performance: vec![read_metrics, write_metrics],
                };
                vec![performance]
            }
            _ => vec![DBPerformance {
                db_name: db_name.to_string(),
                performance: vec![],
            }],
        }
    }

    pub async fn write_metrics<F, Fut>(write_fn: F) -> DBMetrics
    where
        F: Fn(RunePool) -> Fut,
        Fut: future::Future<Output = ()>,
    {
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
            write_fn(interval).await;
        }
        let duration = start.elapsed();
        DBMetrics {
            execution_time: duration.as_secs_f64(),
            operation: super::db_metrics::OperationType::Insert,
            total_records,
        }
    }
}
