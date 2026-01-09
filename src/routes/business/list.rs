use axum::{Json, extract::State, http::StatusCode};

use crate::{middleware::auth::Claims, models::business::Business, routes::users::login::AppState};

/// List user's active businesses
///
/// Returns all active businesses owned by the authenticated user.
/// Only users with a valid JWT token can access this endpoint.
/// The user_id is extracted from the JWT token.
/// Inactive businesses (active = FALSE) are not included in the response.
#[utoipa::path(
    get,
    path = "/business/list",
    tag = "Business",
    responses(
        (status = OK, description = "Businesses retrieved successfully", body = Vec<Business>),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn list_businesses(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Business>>, StatusCode> {
    // Extract user_id from JWT claims for security
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Query all active businesses for this user
    let businesses = sqlx::query_as!(
        Business,
        r#"
        SELECT 
            id,
            name,
            description,
            owner_id,
            verified,
            active
        FROM businesses 
        WHERE owner_id = ? AND active = TRUE
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(businesses))
}
