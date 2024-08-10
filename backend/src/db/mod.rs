use chrono::{DateTime, Utc};
use common::schemas::{Session, User};
use lazy_static::lazy_static;
use sqlx::PgPool;

use crate::{auth::hash_token, config};

lazy_static! {
    pub static ref DB_POOL: PgPool = PgPool::connect_lazy(config::DATABASE_URL.as_str())
        .expect("Failed to create connection pool");
}

pub async fn find_user_by_username(username: &str) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_optional(&*DB_POOL)
        .await?;

    Ok(user)
}

pub async fn find_user_by_email(email: &str) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(&*DB_POOL)
        .await?;

    Ok(user)
}

pub async fn create_user(user: User) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO users (id, username, email, passwordhash) VALUES ($1, $2, $3, $4)",
        user.id,
        user.username,
        user.email,
        user.passwordhash
    )
    .execute(&*DB_POOL)
    .await?;

    Ok(())
}

pub async fn get_all_users() -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&*DB_POOL)
        .await?;

    Ok(users)
}

pub async fn add_session(user_id: &str, token: &str, expiry: DateTime<Utc>) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO session (user_id, token, expiry) VALUES ($1, $2, $3)",
        user_id,
        token,
        expiry.naive_utc()
    )
    .execute(&*DB_POOL)
    .await?;

    Ok(())
}

pub async fn validate_session(token: &str) -> anyhow::Result<bool> {
    let hashed_token = hash_token(token);

    let queue = sqlx::query_as!(
        Session,
        "SELECT * FROM session WHERE token = $1",
        hashed_token
    );

    match queue.fetch_optional(&*DB_POOL).await? {
        Some(item) => Ok(item.expiry.and_utc() > Utc::now()),
        None => Ok(false),
    }
}
