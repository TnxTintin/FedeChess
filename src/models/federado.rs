use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Federado {
    pub id: u32,
    pub id_fada: String,
    pub id_fide: Option<u32>,
    pub nombre: String,
    pub apellidos: String,
    pub fnac: Option<u16>,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub genero: Option<String>,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub cp: Option<String>,
    pub localidad: Option<String>,
    pub provincia: Option<String>,
    pub elo_fada: Option<u32>,     
    pub elo_standard: Option<u32>,
    pub elo_rapid: Option<u32>,
    pub elo_blitz: Option<u32>,
    pub titulo_fide: String,
    pub titulo_nacional: String,
    pub id_club: Option<u32>,
    pub categoria: Option<String>,
    pub alta_federativa: Option<u32>,
    pub activo: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
