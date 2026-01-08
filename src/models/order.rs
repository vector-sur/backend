use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct OrderDetailRequest {
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterOrderRequest {
    pub order_details: Vec<OrderDetailRequest>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RegisterOrderResponse {
    pub order_id: i32,
    pub flight_number: String,
    pub total_price: f64,
    pub approved: bool,
    pub message: String,
}
