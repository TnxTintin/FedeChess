use sqlx::MySqlPool;
use crate::error::AppResult;
use crate::models::Federate;
use crate::repository::FederateRepository;

pub struct FederateService<'a> {
    repo: FederateRepository<'a>,
}

impl<'a> FederateService<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self {
            repo: FederateRepository::new(pool),
        }
    }

    pub async fn list_federates(&self) -> AppResult<Vec<Federate>> {
        self.repo.find_all().await
    }

    pub async fn get_federate(&self, id: u32) -> AppResult<Federate> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound(format!("Federado {}", id)))
    }

    pub async fn search_federates(&self, query: &str) -> AppResult<Vec<Federate>> {
        if query.trim().is_empty() {
            return self.list_federates().await;
        }
        self.repo.search(query).await
    }
}
