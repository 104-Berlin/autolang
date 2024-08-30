use core::fmt;
use gloo_net::Error as NetworkError;
use serde_json::error::Error as JsonError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
}

#[derive(Debug)]
pub enum ApiError {
    NetworkError(NetworkError),
    JsonError(JsonError),
    RequestFailed(String),
}

impl StdError for Error {}
impl StdError for ApiError {}

impl From<ApiError> for Error {
    fn from(error: ApiError) -> Self {
        Error::ApiError(error)
    }
}

impl From<NetworkError> for Error {
    fn from(error: NetworkError) -> Self {
        Self::ApiError(ApiError::NetworkError(error))
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Self::ApiError(ApiError::JsonError(error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(api_error) => write!(f, "API ERROR: {}", api_error),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::JsonError(json_error) => write!(f, "JSON ERROR: {}", json_error),
            Self::NetworkError(network_error) => write!(f, "NETWORK ERROR: {}", network_error),
            Self::RequestFailed(name) => write!(f, "Request failed: {}", name),
        }
    }
}
