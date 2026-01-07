use crate::middleware::auth::{create_token, hash_password};
use crate::models::user::RegisterRequest;
use crate::routes::login::{AppState, AuthResponse};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

// Register endpoint
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
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

    // Fetch the user by username to get the user_id safely
    let user = sqlx::query!("SELECT id FROM users WHERE username = ?", payload.username)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = user.id;

    // Create JWT token
    let token = create_token(user_id, payload.username.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user_id,
        username: payload.username,
    }))
}
