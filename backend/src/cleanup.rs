use sqlx::MySqlPool;
use std::time::Duration;

pub async fn cleanup_expired_sessions(pool: MySqlPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));

    loop {
        interval.tick().await;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let _ = sqlx::query!("DELETE FROM sessions WHERE expires_at < ?", now)
            .execute(&pool)
            .await;
    }
}
