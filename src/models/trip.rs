use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterTripRequest {
    /// Weight of the package in kg
    pub weight: f64,
    /// Order ID associated with this trip
    pub order_id: i32,
    /// Drone's current latitude (starting point)
    pub from_latitude: f64,
    /// Drone's current longitude (starting point)
    pub from_longitude: f64,
    /// Drone ID that will perform the trip
    pub drone_id: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RegisterTripResponse {
    pub trip_id: i32,
    pub weight: f64,
    pub distance: f64,
    pub est_time: f64,
    pub state: String,
    pub order_id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub drone_id: i32,
    pub message: String,
}
