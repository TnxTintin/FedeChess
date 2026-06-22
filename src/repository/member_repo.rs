use sqlx::MySqlPool;
use crate::error::AppResult;
use crate::models::Member;

pub struct MemberRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MemberRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> AppResult<Vec<Member>> {
        let members = sqlx::query_as::<_, Member>(
            "SELECT * FROM members ORDER BY last_name, first_name LIMIT 500"
        )
        .fetch_all(self.pool)
        .await?;
        Ok(members)
    }

    pub async fn find_by_id(&self, id: u32) -> AppResult<Option<Member>> {
        let member = sqlx::query_as::<_, Member>(
            "SELECT * FROM members WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(member)
    }

    pub async fn search(&self, query: &str) -> AppResult<Vec<Member>> {
        let pattern = format!("%{}%", query);
        let members = sqlx::query_as::<_, Member>(
            "SELECT * FROM members
             WHERE first_name LIKE ? OR last_name LIKE ? OR federation_id LIKE ?
             ORDER BY last_name, first_name
             LIMIT 100"
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(self.pool)
        .await?;
        Ok(members)
    }
}
