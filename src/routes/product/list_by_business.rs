use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{middleware::auth::Claims, models::product::Product, routes::users::login::AppState};

/// List products by business
///
/// Returns all active products for a specific business.
/// Only the owner of the business can access this endpoint.
/// The user_id is extracted from the JWT token and verified against the business owner_id.
#[utoipa::path(
    get,
    path = "/product/business/{business_id}",
    tag = "Products",
    params(
        ("business_id" = i32, Path, description = "Business ID to list products from")
    ),
    responses(
        (status = OK, description = "Products retrieved successfully", body = Vec<Product>),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = FORBIDDEN, description = "User is not the owner of this business"),
        (status = NOT_FOUND, description = "Business not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn list_products_by_business(
    claims: Claims,
    State(state): State<AppState>,
    Path(business_id): Path<i32>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    // Extract user_id from JWT claims for security
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify that the business exists and get its owner_id
    let business = sqlx::query!(
        "SELECT id, owner_id FROM businesses WHERE id = ?",
        business_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the requesting user is the owner of the business
    if business.owner_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Query all active products for this business
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT 
            id,
            name,
            description,
            CAST(price AS CHAR) as price,
            business_id,
            active
        FROM products 
        WHERE business_id = ? AND active = TRUE
        ORDER BY created_at DESC
        "#,
        business_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}
