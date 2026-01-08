use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterBusinessRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RegisterBusinessResponse {
    pub business_id: i32,
    pub name: String,
    pub description: String,
    pub owner_id: i32,
    pub verified: bool,
    pub active: bool,
    pub message: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateBusinessRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct UpdateBusinessResponse {
    pub message: String,
    pub business_id: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct DeleteBusinessResponse {
    pub message: String,
    pub business_id: i32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct Business {
    /// Business ID
    pub id: i32,
    /// Name of the business
    pub name: String,
    /// Description of the business
    pub description: Option<String>,
    /// Owner user ID
    pub owner_id: i32,
    /// Whether the business is verified
    pub verified: i8,
    /// Whether the business is active
    pub active: i8,
}
