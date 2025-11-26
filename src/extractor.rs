use std::ops::Deref;

use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use snafu::ResultExt;

use crate::error::{Error as ApiError, InvalidJsonBodySnafu, Result};

pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .context(InvalidJsonBodySnafu)?;
        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::Request,
        routing::post,
        Router,
    };
    use serde::{Deserialize, Serialize};
    use tower::ServiceExt;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestPayload {
        name: String,
        value: i32,
    }

    async fn test_handler(ValidatedJson(payload): ValidatedJson<TestPayload>) -> String {
        format!("{}-{}", payload.name, payload.value)
    }

    #[tokio::test]
    async fn test_validated_json_with_valid_payload() {
        let app = Router::new().route("/test", post(test_handler));

        let payload = TestPayload {
            name: "test".to_string(),
            value: 42,
        };

        let request = Request::builder()
            .uri("/test")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_validated_json_with_invalid_json() {
        let app = Router::new().route("/test", post(test_handler));

        let request = Request::builder()
            .uri("/test")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from("invalid json"))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Should return an error status
        assert!(response.status().is_client_error() || response.status().is_server_error());
    }

    #[tokio::test]
    async fn test_validated_json_with_missing_fields() {
        let app = Router::new().route("/test", post(test_handler));

        let request = Request::builder()
            .uri("/test")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "test"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Should return an error status due to missing 'value' field
        assert!(response.status().is_client_error() || response.status().is_server_error());
    }

    #[test]
    fn test_validated_json_deref() {
        let payload = TestPayload {
            name: "test".to_string(),
            value: 100,
        };
        let validated = ValidatedJson(payload);
        
        assert_eq!(validated.name, "test");
        assert_eq!(validated.value, 100);
    }

    #[tokio::test]
    async fn test_validated_json_with_empty_body() {
        let app = Router::new().route("/test", post(test_handler));

        let request = Request::builder()
            .uri("/test")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert!(response.status().is_client_error() || response.status().is_server_error());
    }

    #[tokio::test]
    async fn test_validated_json_with_extra_fields() {
        let app = Router::new().route("/test", post(test_handler));

        let request = Request::builder()
            .uri("/test")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "test", "value": 42, "extra": "ignored"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Should succeed, extra fields are ignored by default in serde
        assert_eq!(response.status(), 200);
    }
}
