use actix_cors::Cors;
use actix_web::{
    dev::Response,
    error::ErrorBadRequest,
    get,
    middleware::Logger,
    post,
    web::{self},
    App, HttpServer, Responder,
};
use anyhow::Context;
use auth::{generate_password_hash, generate_user_id};
use common::{CreateUserForm, User};
use config::CONFIG;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;
use tracing_actix_web::TracingLogger;

mod auth;
mod config;
mod db;

#[derive(Default)]
pub struct AppData {}

#[post("/register")]
async fn register(
    user: web::Json<CreateUserForm>,
    db: web::Data<PgPool>,
) -> actix_web::Result<impl Responder> {
    let user = User {
        id: generate_user_id().map_err(ErrorBadRequest)?,
        username: user.username.clone(),
        email: user.email.clone(),
        passwordhash: generate_password_hash(&user.password).map_err(|e| ErrorBadRequest(e))?,
    };

    db::create_user(user, db.get_ref())
        .await
        .context("Creating user")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Response::ok())
}

#[get("/users")]
async fn get_all_users(db: web::Data<PgPool>) -> actix_web::Result<impl Responder> {
    let users = db::get_all_users(db.get_ref())
        .await
        .context("Getting all users")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(users))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Loading .env file")?;
    tracing_subscriber::fmt::init();

    let pool: sqlx::Pool<sqlx::Postgres> = connect_db().await?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Running migrations")?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData::default()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .service(register)
            .service(get_all_users)
    })
    .bind(("0.0.0.0", 25565))?
    .run()
    .await
    .context("Running server")
}

async fn connect_db() -> anyhow::Result<PgPool> {
    let res = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.db_domain)
        .await
        .context("Connecting to postgres")?;

    info!("Connected to Postgres");

    Ok(res)
}
