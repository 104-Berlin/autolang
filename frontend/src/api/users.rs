use common::schemas::User;
use gloo_net::http::{Method, Request};

use crate::error::{ApiError, Error};

pub async fn users_get() -> Result<Vec<User>, Error> {
    let response = Request::new("/api/v1/users")
        .method(Method::GET)
        .send()
        .await?;

    if response.ok() {
        let users = response.json().await?;
        Ok(users)
    } else {
        Err(Error::ApiError(ApiError::RequestFailed(
            "GET /users".to_string(),
        )))
    }
}
