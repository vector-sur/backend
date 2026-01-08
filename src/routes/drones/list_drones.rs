use crate::middleware::auth::Claims;
use crate::models::drone::Drone;
use crate::routes::users::login::AppState;
use axum::{Json, extract::State, http::StatusCode};

/// List user's drones
///
/// Returns all drones registered by the authenticated user.
/// Only users with a valid JWT token can access this endpoint.
/// The user_id is extracted from the JWT token.
#[utoipa::path(
    get,
    path = "/drones/list",
    tag = "Drones",
    responses(
        (status = OK, description = "Drones retrieved successfully", body = Vec<Drone>),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn list_drones(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Drone>>, StatusCode> {
    // Extract user_id from JWT claims for security
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Query all drones for this user
    let drones = sqlx::query_as!(
        Drone,
        r#"
        SELECT 
            id,
            name,
            number,
            user_id
        FROM drones 
        WHERE user_id = ?
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(drones))
}
