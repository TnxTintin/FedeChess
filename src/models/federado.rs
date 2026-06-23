use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Federado {
    pub id: u32,
    pub id_fada: String,
    pub id_fide: Option<u32>,
    pub nombre: String,
    pub apellidos: String,
    pub tipo_documento: String,
    pub numero_documento: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub genero: Option<String>,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direecion: Option<String>,
    pub cp: Option<String>,
    pub localidad: Option<String>,
    pub provincia: Option<String>,
    pub elo_standard: Option<u32>,
    pub elo_rapid: Option<u32>,
    pub elo_blitz: Option<u32>,
    pub titulo_fide: String,
    pub titulo_nacional: String,
    pub id_club: Option<u32>,
    pub categoria: Option<String>,
    pub alta_federativa: Option<u32>,
    pub activo: bool,
}
