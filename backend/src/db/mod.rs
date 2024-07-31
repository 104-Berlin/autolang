use actix_web::web;
use common::User;
use sqlx::PgPool;

pub async fn find_user_by_username(username: &str, db: &PgPool) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_optional(db)
        .await?;

    Ok(user)
}

pub async fn find_user_by_email(email: &str, db: &PgPool) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await?;

    Ok(user)
}

pub async fn create_user(user: User, db: &PgPool) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO users (id, username, email, passwordhash) VALUES ($1, $2, $3, $4)",
        user.id,
        user.username,
        user.email,
        user.passwordhash
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn get_all_users(db: &PgPool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db)
        .await?;

    Ok(users)
}
