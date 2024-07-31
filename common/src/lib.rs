use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub passwordhash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}
