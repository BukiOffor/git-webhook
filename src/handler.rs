use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;

use crate::*;

pub async fn github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {

    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !verify_signature(
        &state.github_secret,
        &body,
        signature,
    ) {
        return (
            axum::http::StatusCode::UNAUTHORIZED,
            "Invalid signature",
        );
    }

    let event = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    tracing::info!("Event: {}", event);

    tracing::info!(
        "{}",
        String::from_utf8_lossy(&body)
    );

    let delivery = headers
        .get("X-GitHub-Delivery")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    let mut locked = state.last_delivery.lock().await;

    if let Some(last) = &*locked {
        if last == delivery {
            return (axum::http::StatusCode::OK, "Already processed");
        }
    }

    *locked = Some(delivery.to_string());
    drop(locked);

    if event == "push" {
        state.sender.send(()).await.unwrap_or_else(|e| {
            tracing::error!("Failed to send signal: {}", e);
        });
    }
    (axum::http::StatusCode::OK, "OK")
}