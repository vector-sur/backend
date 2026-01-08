use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RegisterResponse {
    pub user_id: i32,
    pub username: String,
    pub message: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub phone: i64,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub phone: Option<i64>,
    pub email: Option<String>,
    pub password: Option<String>,
}
