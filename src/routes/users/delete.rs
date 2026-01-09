use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::{
    handlers::stats::deactivate_user_stats, middleware::auth::Claims,
    routes::users::login::AppState,
};

#[derive(Serialize, utoipa::ToSchema)]
pub struct DeleteUserResponse {
    pub message: String,
    pub user_id: i32,
}

/// Delete (deactivate) user endpoint
///
/// - Admin only
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, Path, description = "User database id to delete")
    ),
    responses(
        (status = OK, description = "User deactivated successfully", body = DeleteUserResponse),
        (status = FORBIDDEN, description = "User is not an admin"),
        (status = NOT_FOUND, description = "User not found or already inactive"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_user(
    claims: Claims,
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<DeleteUserResponse>, StatusCode> {
    // Check if the requesting user is an admin
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let is_admin = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM admins WHERE user_id = ?) as is_admin",
        requesting_user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if is_admin is 1 (true)
    if is_admin.is_admin == 0 {
        return Err(StatusCode::FORBIDDEN);
    }

    // Deactivate the user by setting active = FALSE
    let result = sqlx::query!("UPDATE users SET active = FALSE WHERE id = ?", user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Update stats: decrement active_accounts and increment inactive_accounts
    deactivate_user_stats(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DeleteUserResponse {
        message: format!("User {} has been deactivated", user_id),
        user_id,
    }))
}
