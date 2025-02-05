use crate::models::rune_pool_model::RunePool;
use dotenv::dotenv;
use std::env;
use surrealdb::engine::any::{self, Any};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct SurrealDB {
    db: Surreal<Any>,
}

impl SurrealDB {
    pub async fn init() -> Self {
        dotenv().ok();
        let surrealdb_url =
            env::var("SURREAL_DATABASE_URL").expect("Failed to get SURREAL_DATABASE_URL");

        let db = any::connect(surrealdb_url)
            .await
            .expect("Failed to connect to SurrealDB");

        // Authenticate first
        db.signin(Root {
            username: "sathvik",
            password: "sathviksathvik",
        })
        .await
        .map_err(|e| {
            eprintln!("SurrealDB Authentication Error: {:?}", e);
            e
        })
        .expect("Failed to authenticate with SurrealDB");

        // Select namespace and database after authentication
        db.use_ns("db-metrics")
            .use_db("db-metrics")
            .await
            .expect("Failed to select namespace and database");

        Self { db }
    }

    pub async fn read_rune_pool(&self) -> Vec<RunePool> {
        self.db.select("rune_pool").await.unwrap_or_default()
    }

    pub async fn write_rune_pool(&self, record: RunePool) {
        let _: Vec<RunePool> = self.db.insert("rune_pool").content(record).await.unwrap();
    }
}
