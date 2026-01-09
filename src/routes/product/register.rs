use axum::{Json, extract::State, http::StatusCode};

use crate::{
    middleware::auth::Claims,
    models::product::{RegisterProductRequest, RegisterProductResponse},
    routes::users::login::AppState,
};

/// Register a new product
///
/// Registers a new product in a business. Only the owner of the business can register products.
/// The business_id is provided in the request, and the system verifies that the authenticated user
/// (from JWT) is the owner of that business before allowing the product registration.
#[utoipa::path(
    post,
    path = "/product/register",
    tag = "Products",
    request_body = RegisterProductRequest,
    responses(
        (status = OK, description = "Product registered successfully", body = RegisterProductResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = FORBIDDEN, description = "User is not the owner of the specified business"),
        (status = BAD_REQUEST, description = "Invalid request data"),
        (status = NOT_FOUND, description = "Business not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn register_product(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<RegisterProductRequest>,
) -> Result<Json<RegisterProductResponse>, StatusCode> {
    // Extract user_id from JWT claims for security
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify that the business exists and get its owner_id
    let business = sqlx::query!(
        "SELECT id, owner_id FROM businesses WHERE id = ?",
        payload.business_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the requesting user is the owner of the business
    if business.owner_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Insert the new product (active defaults to TRUE in the database)
    let result = sqlx::query!(
        "INSERT INTO products (name, description, price, business_id) VALUES (?, ?, ?, ?)",
        payload.name,
        payload.description,
        payload.price,
        payload.business_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_id = result.last_insert_id() as i32;

    Ok(Json(RegisterProductResponse {
        product_id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        business_id: payload.business_id,
        active: true,
        message: "Product registered successfully".to_string(),
    }))
}
