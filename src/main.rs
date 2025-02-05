use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    serve, Json, Router,
};
use models::{db_services::DBServices, MetricsQuery};
pub mod databases;
pub mod models;

async fn home() -> impl IntoResponse {
    "Welcome to db metrics assignment"
}

async fn get_metrics(
    State(db_services): State<DBServices>,
    Query(params): Query<MetricsQuery>,
) -> impl IntoResponse {
    Json(db_services.get_db_metric(&params.db).await)
}

#[tokio::main]
async fn main() {
    let db_services = DBServices::init().await;
    let app = Router::new()
        .route("/", get(home))
        .route("/metrics", get(get_metrics))
        .with_state(db_services);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("Server running on port 3000");
    serve(listener, app).await.unwrap();
}
