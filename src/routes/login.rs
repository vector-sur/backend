use crate::middleware::auth::{create_token, verify_password};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

// Request/Response types
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

// Login endpoint
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Query user from database
    let user = sqlx::query!(
        "SELECT id, username, password_hash FROM users WHERE username = ?",
        payload.username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let password_hash = user.password_hash.ok_or(StatusCode::UNAUTHORIZED)?;
    verify_password(&payload.password, &password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .then_some(())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Create JWT token
    let token = create_token(user.id, user.username.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
    }))
}
