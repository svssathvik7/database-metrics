use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum OperationType {
    Insert,
    Read,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct DBMetrics {
    pub execution_time: f64,
    pub total_records: u64,
    pub operation: OperationType,
}

#[derive(Serialize, Clone)]
pub struct DBPerformance {
    pub db_name: String,
    pub performance: Vec<DBMetrics>,
}
