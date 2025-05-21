use axum::{extract::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

use crate::error::Error as ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    pub user_id: String,
}

// 中间件：验证 token
pub async fn simple_token_auth(req: Request, next: Next) -> Result<Response, ApiError> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::MissingAuth)?;

    Ok(next.run(req).await)
}
