use std::env;

use axum::{routing, Router};
use metrics::receive_metrics;
use sqlx::postgres::PgPoolOptions;

mod metrics;

#[tokio::main]
async fn main() {
    let database_url =
        env::var("DATABASE_URL").expect("🛑 DATABASE_URL environment variable is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("🛑 Can't connect to database");

    let app = Router::new()
        .route("/receive-metrics", routing::post(receive_metrics))
        .with_state(pool);

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
