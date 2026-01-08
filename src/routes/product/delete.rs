use crate::middleware::auth::Claims;
use crate::models::product::DeleteProductResponse;
use crate::routes::users::login::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Delete (deactivate) product endpoint
///
/// Only the owner of the business that owns the product can delete it.
/// The product is soft-deleted by setting active = FALSE.
/// The ownership is verified by checking if the authenticated user (from JWT)
/// is the owner of the business associated with the product.
#[utoipa::path(
    delete,
    path = "/product/{id}",
    tag = "Products",
    params(
        ("id" = i32, Path, description = "Product database id to delete")
    ),
    responses(
        (status = OK, description = "Product deactivated successfully", body = DeleteProductResponse),
        (status = FORBIDDEN, description = "User is not the owner of the business that owns this product"),
        (status = NOT_FOUND, description = "Product not found or already inactive"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_product(
    claims: Claims,
    State(state): State<AppState>,
    Path(product_id): Path<i32>,
) -> Result<Json<DeleteProductResponse>, StatusCode> {
    // Get the requesting user's ID from the JWT claims
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify the product exists and get its business_id and active status
    let product = sqlx::query!(
        "SELECT id, business_id, active FROM products WHERE id = ?",
        product_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if product is already inactive
    if product.active == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Get the business owner_id
    let business = sqlx::query!(
        "SELECT owner_id FROM businesses WHERE id = ?",
        product.business_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the requesting user is the owner of the business
    if business.owner_id != requesting_user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Deactivate the product by setting active = FALSE
    let result = sqlx::query!(
        "UPDATE products SET active = FALSE WHERE id = ?",
        product_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(DeleteProductResponse {
        message: format!("Product {} has been deactivated", product_id),
        product_id,
    }))
}
