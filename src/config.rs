use crate::error::{AppError, AppResult};
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL no definida en .env".into()))?;

        Ok(Self { database_url })
    }
}
