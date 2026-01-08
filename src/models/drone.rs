use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterDroneRequest {
    /// Name of the drone
    pub name: String,
    /// Unique drone number
    pub number: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RegisterDroneResponse {
    /// ID of the newly registered drone
    pub drone_id: i32,
    /// Name of the drone
    pub name: String,
    /// Drone number
    pub number: i32,
    /// Success message
    pub message: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct Drone {
    /// Drone ID
    pub id: i32,
    /// Name of the drone
    pub name: String,
    /// Unique drone number
    pub number: i32,
    /// Owner user ID
    pub user_id: i32,
}
