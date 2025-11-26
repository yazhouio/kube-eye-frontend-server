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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    async fn test_handler() -> &'static str {
        "success"
    }

    #[tokio::test]
    async fn test_auth_with_valid_token() {
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn(simple_token_auth));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", "Bearer valid_token")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_without_token() {
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn(simple_token_auth));

        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_with_empty_header() {
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn(simple_token_auth));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", "")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Empty authorization header should still pass through the middleware
        // since we only check if the header exists
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_auth_info_serialization() {
        let auth_info = AuthInfo {
            user_id: "user123".to_string(),
        };

        let json = serde_json::to_string(&auth_info).unwrap();
        assert!(json.contains("user123"));
    }

    #[test]
    fn test_auth_info_deserialization() {
        let json = r#"{"user_id": "user456"}"#;
        let auth_info: AuthInfo = serde_json::from_str(json).unwrap();
        assert_eq!(auth_info.user_id, "user456");
    }

    #[test]
    fn test_auth_info_clone() {
        let auth_info = AuthInfo {
            user_id: "user789".to_string(),
        };
        let cloned = auth_info.clone();
        assert_eq!(auth_info.user_id, cloned.user_id);
    }
}
