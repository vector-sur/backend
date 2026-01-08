use crate::middleware::auth::Claims;
use crate::models::trip::{RegisterTripRequest, RegisterTripResponse};
use crate::routes::users::login::AppState;
use axum::{Json, extract::State, http::StatusCode};

/// Register a new trip
///
/// Creates a new trip for a drone delivery.
/// The authenticated user must be the owner of the drone specified in the request.
/// The trip is created with state 'Requested' by default.
/// All nullable fields (packing_time, battery_init, etc.) are set to NULL initially.
#[utoipa::path(
    post,
    path = "/trips/register",
    tag = "Trips",
    request_body = RegisterTripRequest,
    responses(
        (status = OK, description = "Trip registered successfully", body = RegisterTripResponse),
        (status = UNAUTHORIZED, description = "Invalid or missing JWT token"),
        (status = FORBIDDEN, description = "User is not the owner of the specified drone"),
        (status = BAD_REQUEST, description = "Invalid request data"),
        (status = NOT_FOUND, description = "Drone, order, or location not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn register_trip(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<RegisterTripRequest>,
) -> Result<Json<RegisterTripResponse>, StatusCode> {
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

    // Validate input values
    if payload.weight <= 0.0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Verify that the drone exists and belongs to the authenticated user
    let drone = sqlx::query!(
        "SELECT id, user_id, active FROM drones WHERE id = ?",
        payload.drone_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if the user is the owner of the drone
    if drone.user_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check if drone is active
    if drone.active == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Verify that the order exists
    sqlx::query!("SELECT id FROM orders WHERE id = ?", payload.order_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Get the business location from the order's products
    // We assume all products in an order belong to the same business
    let business_location = sqlx::query!(
        r#"
        SELECT b.location_id
        FROM order_details od
        JOIN products p ON od.product_id = p.id
        JOIN businesses b ON p.business_id = b.id
        WHERE od.order_id = ?
        LIMIT 1
        "#,
        payload.order_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Verify that the business has a location assigned
    let to_location_id = business_location
        .location_id
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Create from_location based on drone's current coordinates
    let from_location_result = sqlx::query!(
        "INSERT INTO locations (name, latitude, longitude) VALUES (?, ?, ?)",
        format!("Drone {} start position", payload.drone_id),
        payload.from_latitude,
        payload.from_longitude
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let from_location_id = from_location_result.last_insert_id() as i32;

    // TODO: implement logic - Calculate distance based on from_location and to_location coordinates
    let distance: f64 = 10.0;

    // TODO: implement logic - Calculate estimated time based on distance and drone capabilities
    let est_time: f64 = 10.0;

    // Insert the trip
    let trip_result = sqlx::query!(
        r#"
        INSERT INTO trips 
        (weight, distance, est_time, order_id, from_location_id, to_location_id, drone_id) 
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        payload.weight,
        distance,
        est_time,
        payload.order_id,
        from_location_id,
        to_location_id,
        payload.drone_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let trip_id = trip_result.last_insert_id() as i32;

    Ok(Json(RegisterTripResponse {
        trip_id,
        weight: payload.weight,
        distance,
        est_time,
        state: "Requested".to_string(),
        order_id: payload.order_id,
        from_location_id,
        to_location_id,
        drone_id: payload.drone_id,
        message: "Trip registered successfully".to_string(),
    }))
}
