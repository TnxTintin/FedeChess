use sqlx::MySqlPool;
use crate::error::AppResult;
use crate::models::Federado;
use crate::repository::FederadoRepository;

pub struct FederadoService<'a> {
    repo: FederadoRepository<'a>,
}

impl<'a> FederadoService<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self {
            repo: FederadoRepository::new(pool),
        }
    }

    pub async fn list_federados(&self) -> AppResult<Vec<Federado>> {
        self.repo.find_all().await
    }

    pub async fn get_federado(&self, id: u32) -> AppResult<Federado> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound(format!("Federado {}", id)))
    }

    pub async fn search_federados(&self, query: &str) -> AppResult<Vec<Federado>> {
        if query.trim().is_empty() {
            return self.list_federados().await;
        }
        self.repo.search(query).await
    }
}
