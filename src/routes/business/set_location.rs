use crate::middleware::auth::Claims;
use crate::models::location::{SetLocationRequest, SetLocationResponse};
use crate::routes::users::login::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Set location for a business
///
/// Creates a new location and assigns it to the specified business.
/// Only the owner of the business can set its location.
/// The ownership is verified by checking if the authenticated user (from JWT)
/// is the owner of the business.
#[utoipa::path(
    post,
    path = "/business/{id}/location",
    tag = "Business",
    params(
        ("id" = i32, Path, description = "Business database id to set location for")
    ),
    request_body = SetLocationRequest,
    responses(
        (status = OK, description = "Location set successfully", body = SetLocationResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = FORBIDDEN, description = "User is not the owner of this business"),
        (status = NOT_FOUND, description = "Business not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn set_business_location(
    claims: Claims,
    State(state): State<AppState>,
    Path(business_id): Path<i32>,
    Json(payload): Json<SetLocationRequest>,
) -> Result<Json<SetLocationResponse>, StatusCode> {
    // Get the requesting user's ID from the JWT claims
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify the business exists and get its owner_id
    let business = sqlx::query!(
        "SELECT id, owner_id FROM businesses WHERE id = ?",
        business_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the requesting user is the owner of the business
    if business.owner_id != requesting_user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Insert the new location
    let location_result = sqlx::query!(
        "INSERT INTO locations (name, latitude, longitude) VALUES (?, ?, ?)",
        payload.name,
        payload.latitude,
        payload.longitude
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let location_id = location_result.last_insert_id() as i32;

    // Update the business with the new location_id
    sqlx::query!(
        "UPDATE businesses SET location_id = ? WHERE id = ?",
        location_id,
        business_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SetLocationResponse {
        location_id,
        business_id,
        name: payload.name,
        latitude: payload.latitude.to_string(),
        longitude: payload.longitude.to_string(),
        message: format!("Location set successfully for business {}", business_id),
    }))
}
