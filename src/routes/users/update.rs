use crate::middleware::auth::{Claims, hash_password};
use crate::models::user::UpdateUserRequest;
use crate::routes::users::login::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize, utoipa::ToSchema)]
pub struct UpdateUserResponse {
    pub message: String,
    pub user_id: i32,
}

// Update user endpoint - User can update their own data or admin can update any user
// PUT /users/:id
#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User database id to update")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = OK, description = "User updated successfully", body = UpdateUserResponse),
        (status = FORBIDDEN, description = "User is not authorized to update this user"),
        (status = NOT_FOUND, description = "User not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_user(
    claims: Claims,
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UpdateUserResponse>, StatusCode> {
    // Get the requesting user's ID from the JWT claims
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if the user is updating their own data
    let is_own_user = requesting_user_id == user_id;

    // Check if the requesting user is an admin
    let is_admin = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM admins WHERE user_id = ?) as is_admin",
        requesting_user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // User must be either updating their own data or be an admin
    if !is_own_user && is_admin.is_admin == 0 {
        return Err(StatusCode::FORBIDDEN);
    }

    // Verify the user exists
    let user_exists = sqlx::query!("SELECT id FROM users WHERE id = ?", user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if user_exists.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Build the update query dynamically based on provided fields
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(name) = &payload.name {
        updates.push("name = ?");
        values.push(name.clone());
    }

    if let Some(lastname) = &payload.lastname {
        updates.push("lastname = ?");
        values.push(lastname.clone());
    }

    if let Some(phone) = payload.phone {
        updates.push("phone = ?");
        values.push(phone.to_string());
    }

    if let Some(email) = &payload.email {
        updates.push("email = ?");
        values.push(email.clone());
    }

    if let Some(password) = &payload.password {
        let password_hash =
            hash_password(password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        updates.push("password_hash = ?");
        values.push(password_hash);
    }

    // If no fields to update, return early
    if updates.is_empty() {
        return Ok(Json(UpdateUserResponse {
            message: format!("No fields to update for user {}", user_id),
            user_id,
        }));
    }

    // Execute the update query
    let query_str = format!("UPDATE users SET {} WHERE id = ?", updates.join(", "));

    let mut query = sqlx::query(&query_str);
    for value in &values {
        query = query.bind(value);
    }
    query = query.bind(user_id);

    query
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpdateUserResponse {
        message: format!("User {} has been updated successfully", user_id),
        user_id,
    }))
}
