use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Stats {
    pub total_trips: i32,
    pub today_trips: i32,
    pub weekend_trips: i32,
    pub monthly_trips: i32,
    pub avg_delivery_time: f64,
    pub avg_packing_time: f64,
    pub avg_battery_consumption_per_km: f64,
    pub cancellation_rate: f64,
    pub active_accounts: i32,
    pub inactive_accounts: i32,
    pub total_accounts: i32,
}
