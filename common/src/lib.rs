use serde::{Deserialize, Serialize};

pub mod schemas;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}
