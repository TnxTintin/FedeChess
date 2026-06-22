use sqlx::mysql::MySqlPool;
use crate::error::AppResult;

pub async fn create_pool(database_url: &str) -> AppResult<MySqlPool> {
    let pool = MySqlPool::connect(database_url).await?;
    tracing::info!("Conectado a MariaDB correctamente");
    Ok(pool)
}
