use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Federate {
    pub id: u32,
    pub federation_code: String,
    pub fide_id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub document_type: String,
    pub document_number: String,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub elo_standard: Option<u32>,
    pub elo_rapid: Option<u32>,
    pub elo_blitz: Option<u32>,
    pub fide_title: String,
    pub national_title: String,
    pub club_id: Option<u32>,
    pub category: Option<String>,
    pub federation_type: String,
    pub federation_year: Option<u32>,
    pub active: bool,
}
