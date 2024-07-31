use common::User;
use gloo_net::http::{Method, Request};

use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(user: &CreateUser) -> Result<User, ApiError> {
    let user = Request::new("/api/v1/")
        .method(Method::POST)
        .json(user)?
        .send()
        .await?
        .json()
        .await?;

    Ok(user)
}
