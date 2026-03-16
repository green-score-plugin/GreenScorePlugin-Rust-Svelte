use axum::{
    extract::FromRequestParts,
    http::StatusCode,
    middleware::Next,
    response::Response,
    extract::Request,
};
use axum::http::request::Parts;
use tower_sessions::Session;
use crate::dto::user_full::UserFull;

pub struct AuthenticatedUser(pub UserFull);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = parts
            .extensions
            .get::<Session>()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(serde_json::json!({
                    "success": false,
                    "message": "Session extension missing",
                })),
            ))?;

        let user_full: UserFull = session
            .get("user_full")
            .await
            .map_err(|_| (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(serde_json::json!({
                    "success": false,
                    "message": "Session error",
                })),
            ))?
            .ok_or((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "success": false,
                    "message": "errors.auth.unauthenticated",
                })),
            ))?;

        Ok(AuthenticatedUser(user_full))
    }
}

pub async fn require_login_middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    let session = request
        .extensions()
        .get::<Session>()
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "message": "Session extension missing",
            })),
        ))?;

    let user: Option<UserFull> = session
        .get("user_full")
        .await
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "message": "Session error",
            })),
        ))?;

    if user.is_some() {
        Ok(next.run(request).await)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "success": false,
                "message": "errors.auth.unauthenticated",
            })),
        ))
    }
}
