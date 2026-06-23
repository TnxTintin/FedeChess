use sqlx::MySqlPool;
use crate::error::AppResult;
use crate::models::Federate;

pub struct FederateRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> FederateRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> AppResult<Vec<Federate>> {
        let federates = sqlx::query_as::<_, Federate>(
            "SELECT * FROM federates ORDER BY last_name, first_name LIMIT 500"
        )
        .fetch_all(self.pool)
        .await?;
        Ok(federates)
    }

    pub async fn find_by_id(&self, id: u32) -> AppResult<Option<Federate>> {
        let federate = sqlx::query_as::<_, Federate>(
            "SELECT * FROM federates WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(federate)
    }

    pub async fn search(&self, query: &str) -> AppResult<Vec<Federate>> {
        let pattern = format!("%{}%", query);
        let federates = sqlx::query_as::<_, Federate>(
            "SELECT * FROM federates 
             WHERE first_name LIKE ? OR last_name LIKE ? OR federation_code LIKE ?
             ORDER BY last_name, first_name 
             LIMIT 100"
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(self.pool)
        .await?;
        Ok(federates)
    }
}
