use crate::middleware::auth::Claims;
use crate::models::business::{UpdateBusinessRequest, UpdateBusinessResponse};
use crate::routes::users::login::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Update business endpoint
///
/// Only the owner of the business can update it.
/// The owner_id is verified against the JWT token.
#[utoipa::path(
    put,
    path = "/business/{id}",
    tag = "Business",
    params(
        ("id" = i32, Path, description = "Business database id to update")
    ),
    request_body = UpdateBusinessRequest,
    responses(
        (status = OK, description = "Business updated successfully", body = UpdateBusinessResponse),
        (status = FORBIDDEN, description = "User is not the owner of this business"),
        (status = NOT_FOUND, description = "Business not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_business(
    claims: Claims,
    State(state): State<AppState>,
    Path(business_id): Path<i32>,
    Json(payload): Json<UpdateBusinessRequest>,
) -> Result<Json<UpdateBusinessResponse>, StatusCode> {
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

    // Build the update query dynamically based on provided fields
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(name) = &payload.name {
        updates.push("name = ?");
        values.push(name.clone());
    }

    if let Some(description) = &payload.description {
        updates.push("description = ?");
        values.push(description.clone());
    }

    // If no fields to update, return early
    if updates.is_empty() {
        return Ok(Json(UpdateBusinessResponse {
            message: format!("No fields to update for business {}", business_id),
            business_id,
        }));
    }

    // Execute the update query
    let query_str = format!("UPDATE businesses SET {} WHERE id = ?", updates.join(", "));

    let mut query = sqlx::query(&query_str);
    for value in &values {
        query = query.bind(value);
    }
    query = query.bind(business_id);

    query
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpdateBusinessResponse {
        message: format!("Business {} has been updated successfully", business_id),
        business_id,
    }))
}
