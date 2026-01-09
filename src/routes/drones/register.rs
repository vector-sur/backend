use axum::{Json, extract::State, http::StatusCode};

use crate::{
    middleware::auth::Claims,
    models::drone::{RegisterDroneRequest, RegisterDroneResponse},
    routes::users::login::AppState,
};

/// Register a new drone
///
/// Registers a new drone in the system. Only active users with a valid JWT token can register drones.
/// The drone will be associated with the authenticated user (extracted from JWT token).
#[utoipa::path(
    post,
    path = "/drones/register",
    tag = "Drones",
    request_body = RegisterDroneRequest,
    responses(
        (status = OK, description = "Drone registered successfully", body = RegisterDroneResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = FORBIDDEN, description = "User is not active"),
        (status = BAD_REQUEST, description = "Invalid request data or drone number already exists"),
        (status = NOT_FOUND, description = "User not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn register_drone(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<RegisterDroneRequest>,
) -> Result<Json<RegisterDroneResponse>, StatusCode> {
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

    // Check if drone number already exists
    let existing_drone = sqlx::query!("SELECT id FROM drones WHERE number = ?", payload.number)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_drone.is_some() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Insert the new drone
    let result = sqlx::query!(
        "INSERT INTO drones (name, number, user_id) VALUES (?, ?, ?)",
        payload.name,
        payload.number,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let drone_id = result.last_insert_id() as i32;

    Ok(Json(RegisterDroneResponse {
        drone_id,
        name: payload.name,
        number: payload.number,
        message: "Drone registered successfully".to_string(),
    }))
}
