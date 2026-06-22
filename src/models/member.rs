use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Member {
    pub id: u32,
    pub federation_id: String,
    pub first_name: String,
    pub last_name: String,
    pub document_type: String,
    pub document_number: String,
    pub birth_date: Option<NaiveDate>,
    pub elo_standard: Option<u32>,   // ← cambiado de i32 a u32
    pub elo_rapid: Option<u32>,      // ← cambiado de i32 a u32
    pub elo_blitz: Option<u32>,      // ← cambiado de i32 a u32
    pub fide_id: Option<u32>,        // ← cambiado de i32 a u32
    pub title: String,
    pub club_id: Option<u32>,
    pub active: bool,
}
