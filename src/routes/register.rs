use crate::middleware::auth::{create_token, hash_password};
use crate::routes::login::{AppState, AuthResponse};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub phone: i64,
    pub email: String,
    pub password: String,
}

// Register endpoint
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Hash password
    let password_hash =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert user into database
    let result = sqlx::query!(
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

    let user_id = result.last_insert_id() as i32;

    // Create JWT token
    let token = create_token(user_id, payload.username.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user_id,
        username: payload.username,
    }))
}
