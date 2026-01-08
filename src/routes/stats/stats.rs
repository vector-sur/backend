use crate::models::stats::Stats;
use crate::routes::users::login::AppState;
use axum::{extract::State, http::StatusCode, Json};

/// Get global statistics
/// 
/// Returns the global statistics including trip counts, averages, and account information.
/// This is a public endpoint that doesn't require authentication.
#[utoipa::path(
    get,
    path = "/stats",
    responses(
        (status = OK, description = "Statistics retrieved successfully", body = Stats),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<Stats>, StatusCode> {
    // Query the stats table (should only have one row with id = 1)
    let stats = sqlx::query_as!(
        Stats,
        r#"
        SELECT 
            id,
            COALESCE(total_trips, 0) as total_trips,
            COALESCE(today_trips, 0) as today_trips,
            COALESCE(weekend_trips, 0) as weekend_trips,
            COALESCE(monthly_trips, 0) as monthly_trips,
            CAST(COALESCE(avg_delivery_time, 0) AS DOUBLE) as "avg_delivery_time!",
            CAST(COALESCE(avg_packing_time, 0) AS DOUBLE) as "avg_packing_time!",
            CAST(COALESCE(avg_battery_consumption_per_km, 0) AS DOUBLE) as "avg_battery_consumption_per_km!",
            CAST(COALESCE(cancellation_rate, 0) AS DOUBLE) as "cancellation_rate!",
            COALESCE(active_accounts, 0) as active_accounts,
            COALESCE(inactive_accounts, 0) as inactive_accounts,
            COALESCE(total_accounts, 0) as total_accounts
        FROM stats 
        WHERE id = 1
        "#
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stats))
}
