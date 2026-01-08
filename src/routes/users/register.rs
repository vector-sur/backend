use super::login::AppState;
use crate::handlers::stats::increment_user_stats;
use crate::middleware::auth::hash_password;
use crate::models::user::{RegisterRequest, RegisterResponse};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

// Register endpoint
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = OK, description = "Registration successful", body = RegisterResponse),
        (status = CONFLICT, description = "Username already exists"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    // Hash password
    let password_hash =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert user into database
    sqlx::query!(
        "INSERT INTO users (username, name, lastname, phone, email, password_hash) VALUES (?, ?, ?, ?, ?, ?)",
        payload.username,
        payload.name,
        payload.lastname,
        payload.phone,
        payload.email,
        password_hash
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::CONFLICT)?; // Username already exists

    // Update stats: increment total_accounts and active_accounts
    increment_user_stats(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch the user by username to get the user_id safely
    let user = sqlx::query!("SELECT id FROM users WHERE username = ?", payload.username)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = user.id;

    Ok(Json(RegisterResponse {
        user_id,
        username: payload.username,
        message: "User registered successfully".to_string(),
    }))
}
