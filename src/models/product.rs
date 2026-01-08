use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub business_id: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RegisterProductResponse {
    pub product_id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub business_id: i32,
    pub active: bool,
    pub message: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct UpdateProductResponse {
    pub message: String,
    pub product_id: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct DeleteProductResponse {
    pub message: String,
    pub product_id: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct Product {
    /// Product ID
    pub id: i32,
    /// Name of the product
    pub name: String,
    /// Description of the product
    pub description: Option<String>,
    /// Price of the product
    pub price: Option<String>,
    /// Business ID that owns the product
    pub business_id: i32,
    /// Whether the product is active
    pub active: i8,
}
