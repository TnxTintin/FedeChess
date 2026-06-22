use sqlx::MySqlPool;
use crate::error::AppResult;
use crate::models::Member;
use crate::repository::MemberRepository;

/// Servicio de federados.
/// NOTA: No sabe nada de TUI ni de Web. Solo lógica de negocio.
pub struct MemberService<'a> {
    repo: MemberRepository<'a>,
}

impl<'a> MemberService<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self {
            repo: MemberRepository::new(pool),
        }
    }

    pub async fn list_members(&self) -> AppResult<Vec<Member>> {
        // Aquí podrías añadir lógica: permisos, filtros, ordenación, etc.
        self.repo.find_all().await
    }

    pub async fn get_member(&self, id: u32) -> AppResult<Member> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound(format!("Federado {}", id)))
    }

    pub async fn search_members(&self, query: &str) -> AppResult<Vec<Member>> {
        if query.trim().is_empty() {
            return self.list_members().await;
        }
        self.repo.search(query).await
    }
}
