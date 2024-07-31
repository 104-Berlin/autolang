use std::env;

use lazy_static::lazy_static;

pub struct Config {
    pub db_domain: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        db_domain: env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:1234@db:5432/autolang".to_string()),
    };
}
