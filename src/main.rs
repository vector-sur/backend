mod config;
mod middleware;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use middleware::auth::{Claims, create_token, hash_password, verify_password};

#[derive(Clone)]
struct AppState {
    db: MySqlPool,
}

// Request/Response types
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    name: String,
    lastname: String,
    phone: i64,
    email: Option<String>,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
    user_id: i32,
    username: String,
}

#[derive(Serialize)]
struct ProtectedResponse {
    message: String,
    user_id: String,
    username: String,
}

// Login endpoint
async fn login(
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

// Register endpoint
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Hash password
    let password_hash = hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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

// Protected endpoint example - requires valid JWT
async fn protected(claims: Claims) -> Json<ProtectedResponse> {
    Json(ProtectedResponse {
        message: "You are authenticated!".to_string(),
        user_id: claims.sub,
        username: claims.username,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::load_env();
    let pool = config::database::create_pool()
        .await
        .expect("Failed to connect to the database");
    let state = AppState { db: pool };
    
    let app = Router::new()
        // Public routes
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        // Protected routes (require JWT)
        .route("/protected", get(protected))
        .with_state(state);
    
    let addr = config::get_server_addr();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
