use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    serve, Json, Router,
};
use models::{
    db_metrics::DBMetricResponse,
    db_services::{self, DBServices},
    MetricsQuery,
};
use serde_json::json;
pub mod databases;
pub mod models;

async fn home() -> impl IntoResponse {
    "Welcome to db metrics assignment"
}

async fn get_metrics(
    State(db_services): State<DBServices>,
    Query(params): Query<MetricsQuery>,
) -> impl IntoResponse {
    match params.db.clone().as_str() {
        "mongodb" => {
            let read_metrics = db_services.mongo_fetch_rune().await;
            let write_metrics = db_services.mongo_write_rune().await;
            let metrics = DBMetricResponse {
                db_name: params.db.to_string(),
                performance: vec![read_metrics, write_metrics],
            };
            Json(metrics)
        }
        _ => Json(DBMetricResponse {
            db_name: "Invalid".to_string(),
            performance: vec![],
        }),
    }
}

#[tokio::main]
async fn main() {
    let db_services = DBServices::init().await;
    let app = Router::new()
        .route("/", get(home))
        .route("/metrics", get(get_metrics))
        .with_state(db_services);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on port 3000");
    serve(listener, app).await.unwrap();
}
