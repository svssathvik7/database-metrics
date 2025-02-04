use std::env;

use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};

pub struct PostgreSQL {
    pub pool: Pool<Postgres>,
}

impl PostgreSQL {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("POSTGRES_URI").expect("Failed to get POSTGRES_URI");
        let pool = PgPool::connect(&uri)
            .await
            .expect("Failed to connect to PostgreSQL");
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS rune_pool (
            count TEXT, end_time TEXT, start_time TEXT, units TEXT)",
        )
        .execute(&pool)
        .await
        .expect("Failed to create table in postgresql");

        Self { pool }
    }
}
