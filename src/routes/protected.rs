use crate::middleware::auth::Claims;
use axum::extract::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProtectedResponse {
    pub message: String,
    pub user_id: String,
    pub username: String,
}

// Protected endpoint example - requires valid JWT
pub async fn protected(claims: Claims) -> Json<ProtectedResponse> {
    Json(ProtectedResponse {
        message: "You are authenticated!".to_string(),
        user_id: claims.sub,
        username: claims.username,
    })
}
