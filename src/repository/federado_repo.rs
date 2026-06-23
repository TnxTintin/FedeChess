use sqlx::MySqlPool;
use crate::error::AppResult;
use crate::models::Federado;

pub struct FederadoRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> FederadoRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> AppResult<Vec<Federado>> {
        let federados = sqlx::query_as::<_, Federado>(
            "SELECT * FROM federados ORDER BY apellidos, nombre LIMIT 500"
        )
        .fetch_all(self.pool)
        .await?;
        Ok(federados)
    }

    pub async fn find_by_id(&self, id: u32) -> AppResult<Option<Federado>> {
        let federado = sqlx::query_as::<_, Federado>(
            "SELECT * FROM federados WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(federado)
    }

    pub async fn search(&self, query: &str) -> AppResult<Vec<Federado>> {
        let pattern = format!("%{}%", query);
        let federados = sqlx::query_as::<_, Federado>(
            "SELECT * FROM federados 
             WHERE nombre LIKE ? OR apellidos LIKE ? OR id_fada LIKE ?
             ORDER BY apellidos, nombre 
             LIMIT 100"
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(self.pool)
        .await?;
        Ok(federados)
    }
}
