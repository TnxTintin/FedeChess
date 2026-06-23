use crate::error::{AppError, AppResult};
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let db_host = env::var("DB_HOST")
            .map_err(|_| AppError::Config("DB_HOST no definida en .env".into()))?;
        
        let db_port = env::var("DB_PORT")
            .unwrap_or_else(|_| "3307".to_string());
        
        let db_user = env::var("DB_USER")
            .map_err(|_| AppError::Config("DB_USER no definida en .env".into()))?;
        
        let db_password = env::var("DB_PASSWORD")
            .map_err(|_| AppError::Config("DB_PASSWORD no definida en .env".into()))?;
        
        let db_name = env::var("DB_NAME")
            .map_err(|_| AppError::Config("DB_NAME no definida en .env".into()))?;

        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );

        Ok(Self { database_url })
    }
}
