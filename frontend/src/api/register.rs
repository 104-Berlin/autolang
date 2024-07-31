use common::CreateUserForm;
use gloo_net::http::{Method, Request};

use crate::error::{ApiError, Error};

pub async fn register_post(user: &CreateUserForm) -> Result<(), Error> {
    let response = Request::new("/api/v1/register")
        .method(Method::POST)
        .json(user)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        Err(Error::ApiError(ApiError::RequestFailed(
            "POST /register".to_string(),
        )))
    }
}
