use axum::{routing, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/receive-metrics", routing::post(receive_metrics));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn receive_metrics() {
    // TODO: deduplicate metrics
    // TODO: verify metric coming from a fleet canister
}
