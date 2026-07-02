use axum::{routing::post, Router};
use std::sync::Arc;
use git_webhook::*;



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();

    let (tx, rx) = tokio::sync::mpsc::channel::<()>(1);

    let secret = std::env::var("GITHUB_SECRET")
        .expect("GITHUB_SECRET not set");

    git_webhook::write_deploy_script().unwrap_or_else(|e| {
        tracing::error!("Failed to write deploy script: {}", e);
    });
    
    let state = AppState {
        sender: tx,
        github_secret: Arc::new(secret),
        last_delivery: Arc::new(tokio::sync::Mutex::new(None)),
    };

    tokio::spawn(async move {
        run_worker(rx).await;
    });

    let app = Router::new()
        .route("/webhook", post(github_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("Server listening on {:#}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
