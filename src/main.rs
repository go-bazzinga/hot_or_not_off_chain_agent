use std::env;

use axum::{routing, Router};
use metrics::receive_metrics;
use sqlx::postgres::PgPoolOptions;

mod metrics;

#[tokio::main]
async fn main() {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/receive-metrics", routing::post(receive_metrics))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
