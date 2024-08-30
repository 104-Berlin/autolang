use actix_cors::Cors;
use actix_web::{
    dev::Response,
    error::ErrorBadRequest,
    middleware::Logger,
    web::{self},
    App, HttpServer, Responder,
};
use anyhow::Context;
use auth::{generate_password_hash, generate_user_id};
use common::{schemas::User, CreateUserForm};
use tracing_actix_web::TracingLogger;

mod auth;
mod config;
mod db;

#[derive(Default)]
pub struct AppData {}

async fn register(user: web::Json<CreateUserForm>) -> actix_web::Result<impl Responder> {
    let user = User {
        id: generate_user_id().map_err(ErrorBadRequest)?,
        username: user.username.clone(),
        email: user.email.clone(),
        passwordhash: generate_password_hash(&user.password).map_err(|e| ErrorBadRequest(e))?,
    };

    db::create_user(user)
        .await
        .context("Creating user")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Response::ok())
}

async fn get_all_users() -> actix_web::Result<impl Responder> {
    let users = db::get_all_users()
        .await
        .context("Getting all users")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(users))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Loading .env file")?;
    tracing_subscriber::fmt::init();

    sqlx::migrate!("./migrations")
        .run(&*db::DB_POOL)
        .await
        .context("Running migrations")?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData::default()))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .service(web::resource("/user").route(web::get().to(get_all_users)))
            .service(web::resource("/register").route(web::post().to(register)))
    })
    .bind(("0.0.0.0", 25565))?
    .run()
    .await
    .context("Running server")
}
