use anyhow::Context;
use argon2::{hash_encoded, Config};
use hex::encode;
use sha2::{Digest, Sha256};

/// Generate a 32 byte token as hex string
pub fn generate_token() -> anyhow::Result<String> {
    let mut token = [0u8; 32];
    getrandom::getrandom(&mut token)?;
    Ok(hex::encode(token))
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    encode(hasher.finalize())
}

pub fn generate_password_hash(password: &str) -> anyhow::Result<String> {
    let config = Config::default();
    let mut salt = [0u8; 32];
    getrandom::getrandom(&mut salt)?;
    hash_encoded(password.as_bytes(), &salt, &config).context("Hasing password")
}

pub fn generate_user_id() -> anyhow::Result<String> {
    generate_token()
}
