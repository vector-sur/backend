use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    middleware::auth::Claims,
    models::product::{UpdateProductRequest, UpdateProductResponse},
    routes::users::login::AppState,
};

/// Update product endpoint
///
/// Only the owner of the business that owns the product can update it.
/// The ownership is verified by checking if the authenticated user (from JWT)
/// is the owner of the business associated with the product.
#[utoipa::path(
    put,
    path = "/product/{id}",
    tag = "Products",
    params(
        ("id" = i32, Path, description = "Product database id to update")
    ),
    request_body = UpdateProductRequest,
    responses(
        (status = OK, description = "Product updated successfully", body = UpdateProductResponse),
        (status = FORBIDDEN, description = "User is not the owner of the business that owns this product"),
        (status = NOT_FOUND, description = "Product not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_product(
    claims: Claims,
    State(state): State<AppState>,
    Path(product_id): Path<i32>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<UpdateProductResponse>, StatusCode> {
    // Get the requesting user's ID from the JWT claims
    let requesting_user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify the product exists and get its business_id
    let product = sqlx::query!(
        "SELECT id, business_id FROM products WHERE id = ?",
        product_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

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

    if let Some(price) = payload.price {
        updates.push("price = ?");
        values.push(price.to_string());
    }

    // If no fields to update, return early
    if updates.is_empty() {
        return Ok(Json(UpdateProductResponse {
            message: format!("No fields to update for product {}", product_id),
            product_id,
        }));
    }

    // Execute the update query
    let query_str = format!("UPDATE products SET {} WHERE id = ?", updates.join(", "));

    let mut query = sqlx::query(&query_str);
    for value in &values {
        query = query.bind(value);
    }
    query = query.bind(product_id);

    query
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpdateProductResponse {
        message: format!("Product {} has been updated successfully", product_id),
        product_id,
    }))
}
