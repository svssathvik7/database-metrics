pub enum OperationType {
    Insert,
    Read,
}
pub struct DBMetrics {
    pub db_name: String,
    pub execution_time: f64,
    pub total_records: u64,
    pub operation: OperationType,
}
