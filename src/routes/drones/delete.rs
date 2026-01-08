use crate::middleware::auth::Claims;
use crate::models::drone::DeleteDroneResponse;
use crate::routes::users::login::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Delete (deactivate) drone endpoint
///
/// Only the owner of the drone can delete it.
/// The drone is soft-deleted by setting active = FALSE.
/// The ownership is verified by checking if the authenticated user (from JWT)
/// is the owner of the drone.
#[utoipa::path(
    delete,
    path = "/drones/{id}",
    tag = "Drones",
    params(
        ("id" = i32, Path, description = "Drone database id to delete")
    ),
    responses(
        (status = OK, description = "Drone deactivated successfully", body = DeleteDroneResponse),
        (status = FORBIDDEN, description = "User is not the owner of this drone"),
        (status = NOT_FOUND, description = "Drone not found or already inactive"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_drone(
    claims: Claims,
    State(state): State<AppState>,
    Path(drone_id): Path<i32>,
) -> Result<Json<DeleteDroneResponse>, StatusCode> {
    // Get the requesting user's ID from the JWT claims
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify the drone exists and get its user_id and active status
    let drone = sqlx::query!(
        "SELECT id, user_id, active FROM drones WHERE id = ?",
        drone_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if drone is already inactive
    if drone.active == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check if the requesting user is the owner of the drone
    if drone.user_id != requesting_user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Deactivate the drone by setting active = FALSE
    let result = sqlx::query!("UPDATE drones SET active = FALSE WHERE id = ?", drone_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(DeleteDroneResponse {
        message: format!("Drone {} has been deactivated", drone_id),
        drone_id,
    }))
}
