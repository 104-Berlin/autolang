use std::env;

use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    pub static ref DOMAIN: Url =
        Url::parse(&env::var("DOMAIN").unwrap_or("http://localhost:8080".to_string()))
            .expect("Invalid domain");
    pub static ref COOKIE_DOMAIN: String =
        env::var("COOKIE_DOMAIN").unwrap_or("localhost".to_string());
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres:1234@db:5432/autolang".to_string());
    pub static ref PRODUCTION: bool = env::var("PRODUCTION")
        .unwrap_or("false".to_string())
        .parse()
        .expect("Invalid PRODUCTION value. Expected true or false");
}
