use crate::models::rune_pool_model::RunePool;
use rusty_leveldb::{AsyncDB, Options};
use serde_json;
use std::sync::Arc;

#[derive(Clone)]
pub struct LevelDB {
    db: Arc<AsyncDB>,
}

impl LevelDB {
    pub async fn init() -> Self {
        let opt = Options::default();
        let db = AsyncDB::new("./data/leveldb", opt)
            .map_err(|e| format!("Failed to initialize LevelDB: {:?}", e))
            .unwrap();

        LevelDB { db: Arc::new(db) }
    }

    pub async fn read_rune_pool(&self) -> Vec<RunePool> {
        let mut rune_pool_records = Vec::new();

        let keys_index: Vec<Vec<u8>> = match self.db.get(b"_keys_index".to_vec()).await {
            Ok(Some(index)) => serde_json::from_slice(&index).unwrap_or_default(),
            _ => return rune_pool_records,
        };

        let mut count = 0;
        for key in keys_index {
            if count >= 400 {
                break;
            }
            if let Ok(Some(value)) = self.db.get(key).await {
                if let Ok(record) = serde_json::from_slice::<RunePool>(&value) {
                    rune_pool_records.push(record);
                    count += 1;
                }
            }
        }

        rune_pool_records
    }

    pub async fn write_rune_pool(&self, record: RunePool) {
        let key = format!("{}:{}", record.start_time, record.end_time);
        let value = serde_json::to_vec(&record)
            .map_err(|e| format!("Failed to serialize RunePool record: {:?}", e))
            .unwrap();

        // Update the data
        self.db
            .put(key.as_bytes().to_vec(), value.clone())
            .await
            .map_err(|e| format!("Failed to write rune pool record: {:?}", e))
            .unwrap();

        // Update keys index
        let mut keys_index: Vec<Vec<u8>> = match self.db.get(b"_keys_index".to_vec()).await {
            Ok(Some(index)) => serde_json::from_slice(&index).unwrap_or_default(),
            _ => Vec::new(),
        };

        keys_index.push(key.as_bytes().to_vec());
        let index_value = serde_json::to_vec(&keys_index)
            .map_err(|e| format!("Failed to serialize keys index: {:?}", e))
            .unwrap();

        self.db
            .put(b"_keys_index".to_vec(), index_value)
            .await
            .map_err(|e| format!("Failed to update keys index: {:?}", e))
            .unwrap();
    }
}
