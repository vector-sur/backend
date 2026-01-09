use axum::{Json, extract::State, http::StatusCode};

use crate::{
    middleware::auth::Claims,
    models::business::{RegisterBusinessRequest, RegisterBusinessResponse},
    routes::users::login::AppState,
};

/// Register a new business
///
/// Registers a new business in the system. Only active users with a valid JWT token can register businesses.
/// The business will be associated with the authenticated user (extracted from JWT token) as the owner.
/// New businesses start with verified = false by default.
#[utoipa::path(
    post,
    path = "/business/register",
    tag = "Business",
    request_body = RegisterBusinessRequest,
    responses(
        (status = OK, description = "Business registered successfully", body = RegisterBusinessResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = FORBIDDEN, description = "User is not active"),
        (status = BAD_REQUEST, description = "Invalid request data"),
        (status = NOT_FOUND, description = "User not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn register_business(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<RegisterBusinessRequest>,
) -> Result<Json<RegisterBusinessResponse>, StatusCode> {
    // Extract user_id from JWT claims for security
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify that the user exists and is active
    let user = sqlx::query!("SELECT id, active FROM users WHERE id = ?", user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check if user is active (MySQL BOOLEAN is stored as i8)
    if user.active == 0 {
        return Err(StatusCode::FORBIDDEN);
    }

    // Insert the new business (verified defaults to FALSE in the database)
    let result = sqlx::query!(
        "INSERT INTO businesses (name, description, owner_id) VALUES (?, ?, ?)",
        payload.name,
        payload.description,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let business_id = result.last_insert_id() as i32;

    Ok(Json(RegisterBusinessResponse {
        business_id,
        name: payload.name,
        description: payload.description,
        owner_id: user_id,
        verified: false,
        active: true,
        message: "Business registered successfully".to_string(),
    }))
}
