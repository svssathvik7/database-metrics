use std::env;

use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

use crate::models::rune_pool_model::RunePool;

pub struct MongoDB {
    pub rune_pool_collection: Collection<RunePool>,
}

impl MongoDB {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("DB_URI").expect("Failed to get DB_URI");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("mongodb-db-metrics");
        let rune_pool_collection = db.collection("rune_pool_collection");
        MongoDB {
            rune_pool_collection,
        }
    }

    pub async fn read_rune_pool(&self) -> Vec<RunePool> {
        let filter = doc! {};
        let cursor = self
            .rune_pool_collection
            .find(filter)
            .await
            .expect("Failed to fetch rune pool records");
        cursor
            .try_collect()
            .await
            .expect("Failed to collect rune pool records")
    }
}
