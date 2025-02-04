use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum OperationType {
    Insert,
    Read,
}

#[derive(Debug, Serialize)]
pub struct DBMetrics {
    pub execution_time: f64,
    pub total_records: u64,
    pub operation: OperationType,
}

#[derive(Serialize)]
pub struct DBMetricResponse {
    pub db_name: String,
    pub performance: Vec<DBMetrics>,
}
