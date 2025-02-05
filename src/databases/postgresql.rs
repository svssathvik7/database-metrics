use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

use crate::models::rune_pool_model::RunePool;

pub struct PostgreSQL {
    pub pool: Pool<Postgres>,
}

impl PostgreSQL {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("POSTGRES_URI").expect("Failed to get POSTGRES_URI");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
            .map_err(|e| e)
            .unwrap();

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS rune_pool_history (
                start_time TEXT NOT NULL,
                end_time TEXT NOT NULL,
                count TEXT NOT NULL,
                units TEXT NOT NULL
            )"#,
        )
        .execute(&pool)
        .await
        .unwrap();
        Self { pool }
    }

    pub async fn read_rune_pool(&self) -> Vec<RunePool> {
        let rows =
            sqlx::query(r#"SELECT count, end_time, start_time, units FROM rune_pool LIMIT 400"#)
                .fetch_all(&self.pool)
                .await
                .expect("Failed to read rune pool from postgresql");
        let rune_pool_records: Vec<RunePool> = rows
            .into_iter()
            .map(|record| RunePool {
                count: record.get("count"),
                end_time: record.get("end_time"),
                start_time: record.get("start_time"),
                units: record.get("units"),
            })
            .collect();
        rune_pool_records
    }
}
