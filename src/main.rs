use axum::{response::IntoResponse, routing::get, serve, Router};
pub mod databases;
pub mod models;
async fn home() -> impl IntoResponse {
    "Welcome to db metrics assignment"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    serve(listener, app).await.unwrap();
}
