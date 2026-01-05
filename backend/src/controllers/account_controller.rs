use axum::extract::State;
use axum::Json;
use sqlx::MySqlPool;
use serde::Serialize;
use http::StatusCode;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

pub async fn delete_account(
    State(pool): State<MySqlPool>,
) -> Result<(StatusCode, Json<ApiResponse>), StatusCode> {
    let user_id = 210;

    match sqlx::query!("DELETE FROM user WHERE id = ?", user_id)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                message: "Compte supprimé avec succès".to_string(),
            }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
