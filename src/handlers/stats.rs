use sqlx::MySqlPool;

/// Increment total_accounts and active_accounts by 1 when a new user is created
pub async fn increment_user_stats(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE stats SET total_accounts = total_accounts + 1, active_accounts = active_accounts + 1 WHERE id = 1"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Decrement active_accounts and increment inactive_accounts when a user is deactivated
pub async fn deactivate_user_stats(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE stats SET active_accounts = active_accounts - 1, inactive_accounts = inactive_accounts + 1 WHERE id = 1"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
