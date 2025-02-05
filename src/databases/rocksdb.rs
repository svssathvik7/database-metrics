use crate::models::rune_pool_model::RunePool;
use rocksdb::{Options, DB};
use std::{path::Path, sync::Arc};

#[derive(Clone)]
pub struct RocksDB {
    pub db: Arc<DB>,
}

impl RocksDB {
    pub async fn init() -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);

        let path = Path::new("./data/rocksdb");
        let db = Arc::new(DB::open(&options, path).expect("Failed to open RocksDB"));

        Self { db }
    }

    pub async fn read_rune_pool(&self) -> Vec<RunePool> {
        let mut rune_pool_records = Vec::new();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);

        for item in iter.take(400) {
            if let Ok((_, value)) = item {
                if let Ok(record) = serde_json::from_slice::<RunePool>(&value) {
                    rune_pool_records.push(record);
                }
            }
        }

        rune_pool_records
    }

    pub async fn write_rune_pool(&self, record: RunePool) {
        let key = format!("{}:{}", record.start_time, record.end_time);
        let value = serde_json::to_vec(&record).expect("Failed to serialize RunePool record");

        self.db
            .put(key, value)
            .expect("Failed to write rune pool record to RocksDB");
    }
}
