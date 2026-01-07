use crate::middleware::auth::{create_token, verify_password};
use crate::models::user::{AuthResponse, LoginRequest};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

// Login endpoint
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = OK, description = "Login successful", body = AuthResponse),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
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
