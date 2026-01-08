use crate::middleware::auth::Claims;
use crate::models::order::{RegisterOrderRequest, RegisterOrderResponse};
use crate::routes::users::login::AppState;
use axum::{Json, extract::State, http::StatusCode};

/// Register a new order
///
/// Creates a new order with multiple order details (products).
/// Only authenticated users with valid JWT can place orders.
/// Only products from verified businesses are allowed.
/// The order total_price is calculated from the sum of (product price * amount) for each order detail.
/// The flight_number is auto-generated based on the order count.
#[utoipa::path(
    post,
    path = "/orders/register",
    tag = "Orders",
    request_body = RegisterOrderRequest,
    responses(
        (status = OK, description = "Order registered successfully", body = RegisterOrderResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = BAD_REQUEST, description = "Invalid request data or products from unverified business"),
        (status = NOT_FOUND, description = "Product not found or user not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn register_order(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<RegisterOrderRequest>,
) -> Result<Json<RegisterOrderResponse>, StatusCode> {
    // Extract user_id from JWT claims
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify user exists and is active
    let user = sqlx::query!("SELECT id, active FROM users WHERE id = ?", user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if user.active == 0 {
        return Err(StatusCode::FORBIDDEN);
    }

    // Validate that the order has at least one product
    if payload.order_details.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Start a transaction
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Calculate total price and validate all products
    let mut total_price: f64 = 0.0;
    let mut order_details_data: Vec<(i32, i32, f64)> = Vec::new();

    for detail in &payload.order_details {
        // Fetch product and verify it exists, is active, and belongs to a verified business
        let product_info = sqlx::query!(
            r#"
            SELECT p.id, CAST(p.price AS CHAR) as price, p.active, b.verified
            FROM products p
            JOIN businesses b ON p.business_id = b.id
            WHERE p.id = ?
            "#,
            detail.product_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

        // Verify product is active
        if product_info.active == 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        // Verify business is verified (only products from verified businesses are allowed)
        if product_info.verified == 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        // Verify amount is positive
        if detail.amount <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        // Calculate price for this detail
        let price = product_info
            .price
            .as_ref()
            .and_then(|p| p.parse::<f64>().ok())
            .unwrap_or(0.0);
        let detail_total = price * detail.amount as f64;
        total_price += detail_total;

        // Store for later insertion
        order_details_data.push((detail.product_id, detail.amount, price));
    }

    // Generate flight_number (simple implementation: use order count + 1)
    let order_count = sqlx::query!("SELECT COUNT(*) as count FROM orders")
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: we need to sync with BD to check what number give
    // format: AA999
    let flight_number = format!("FL{:03}", (order_count.count + 1) % 1000);

    // Insert the order
    let order_result = sqlx::query!(
        "INSERT INTO orders (flight_number, total_price, user_id) VALUES (?, ?, ?)",
        flight_number,
        total_price,
        user_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let order_id = order_result.last_insert_id() as i32;

    // Insert order details
    for (product_id, amount, price) in order_details_data {
        sqlx::query!(
            "INSERT INTO order_details (order_id, product_id, amount, price) VALUES (?, ?, ?, ?)",
            order_id,
            product_id,
            amount,
            price
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Commit transaction
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RegisterOrderResponse {
        order_id,
        flight_number,
        total_price,
        approved: false, // Default value
        message: "Order registered successfully".to_string(),
    }))
}
