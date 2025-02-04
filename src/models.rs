use serde::Deserialize;

pub mod db_metrics;
pub mod db_services;
pub mod rune_pool_model;

#[derive(Deserialize)]
pub struct MetricsQuery {
    pub db: String,
}
