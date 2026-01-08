use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct SetLocationRequest {
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct SetLocationResponse {
    pub location_id: i32,
    pub business_id: i32,
    pub name: Option<String>,
    pub latitude: String,
    pub longitude: String,
    pub message: String,
}
