use crate::middleware::auth::Claims;
use crate::models::business::DeleteBusinessResponse;
use crate::routes::users::login::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Delete (deactivate) business endpoint
///
/// Only the owner of the business can delete it.
/// The business is soft-deleted by setting active = FALSE.
/// The owner_id is verified against the JWT token.
#[utoipa::path(
    delete,
    path = "/business/{id}",
    tag = "Business",
    params(
        ("id" = i32, Path, description = "Business database id to delete")
    ),
    responses(
        (status = OK, description = "Business deactivated successfully", body = DeleteBusinessResponse),
        (status = FORBIDDEN, description = "User is not the owner of this business"),
        (status = NOT_FOUND, description = "Business not found or already inactive"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_business(
    claims: Claims,
    State(state): State<AppState>,
    Path(business_id): Path<i32>,
) -> Result<Json<DeleteBusinessResponse>, StatusCode> {
    // Get the requesting user's ID from the JWT claims
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify the business exists and get its owner_id
    let business = sqlx::query!(
        "SELECT id, owner_id, active FROM businesses WHERE id = ?",
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

    // Check if business is already inactive
    if business.active == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Deactivate the business by setting active = FALSE
    let result = sqlx::query!(
        "UPDATE businesses SET active = FALSE WHERE id = ?",
        business_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(DeleteBusinessResponse {
        message: format!("Business {} has been deactivated", business_id),
        business_id,
    }))
}
