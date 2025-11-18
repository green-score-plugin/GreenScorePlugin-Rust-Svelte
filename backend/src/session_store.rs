use async_trait::async_trait;
use sqlx::MySqlPool;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_sessions::session_store::Error as SessionError;

#[derive(Clone)]
pub struct MySqlStore {
    pool: MySqlPool,
}

impl MySqlStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    fn now_unix() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_secs() as i64
    }
}

impl fmt::Debug for MySqlStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MySqlStore")
            .field("pool", &"<MySqlPool>")
            .finish()
    }
}

#[async_trait]
impl tower_sessions::SessionStore for MySqlStore {
    async fn create(&self, record: &mut tower_sessions::session::Record) -> Result<(), SessionError> {
        let id = record.id.to_string();
        let data = serde_json::to_vec(record)
            .map_err(|e| SessionError::Encode(e.to_string()))?;
        let expires_at = record.expiry_date.unix_timestamp();

        sqlx::query!(
            "INSERT INTO sessions (id, data, expires_at) VALUES (?, ?, ?)",
            id,
            data,
            expires_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| SessionError::Backend(e.to_string()))?;
        Ok(())
    }

    async fn save(&self, record: &tower_sessions::session::Record) -> Result<(), SessionError> {
        let id = record.id.to_string();
        let data = serde_json::to_vec(record)
            .map_err(|e| SessionError::Encode(e.to_string()))?;
        let expires_at = record.expiry_date.unix_timestamp();

        sqlx::query!(
            r#"
            INSERT INTO sessions (id, data, expires_at)
            VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE data = VALUES(data), expires_at = VALUES(expires_at)
            "#,
            id,
            data,
            expires_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| SessionError::Backend(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, session_id: &tower_sessions::session::Id) -> Result<Option<tower_sessions::session::Record>, SessionError> {
        let id_str = session_id.to_string();
        let row = sqlx::query!("SELECT data, expires_at FROM sessions WHERE id = ? LIMIT 1", id_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| SessionError::Backend(e.to_string()))?;

        if let Some(r) = row {
            if let Some(exp) = r.expires_at {
                if exp <= Self::now_unix() {
                    let _ = sqlx::query!("DELETE FROM sessions WHERE id = ?", id_str)
                        .execute(&self.pool)
                        .await;
                    return Ok(None);
                }
            }
            let record = serde_json::from_slice(&r.data)
                .map_err(|e| SessionError::Decode(e.to_string()))?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &tower_sessions::session::Id) -> Result<(), SessionError> {
        let id_str = session_id.to_string();
        sqlx::query!("DELETE FROM sessions WHERE id = ?", id_str)
            .execute(&self.pool)
            .await
            .map_err(|e| SessionError::Backend(e.to_string()))?;
        Ok(())
    }
}
