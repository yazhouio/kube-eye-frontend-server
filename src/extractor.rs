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
