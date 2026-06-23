use std::fs::File;
use std::io::BufReader;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use crate::error::{AppError, AppResult};

/// Representa un jugador de la lista FIDE
#[derive(Debug, Clone, Default)]
pub struct FidePlayer {
    pub id_fide: u32,
    pub nombre: String,
    pub federacion: String,
    pub sexo: String,
    pub titulo: String,
    pub titulo_femenino: String,
    pub titulo_arbitro: String,
    pub elo_standard: u32,
    pub partidas_standard: u32,
    pub k_standard: u32,
    pub elo_rapid: u32,
    pub partidas_rapid: u32,
    pub k_rapid: u32,
    pub elo_blitz: u32,
    pub partidas_blitz: u32,
    pub k_blitz: u32,
    pub anio_nacimiento: u32,
    pub flag: String,
}

/// Estado del parser mientras lee un <player>
#[derive(Default)]
struct PlayerBuilder {
    current_tag: String,
    current_text: String,
    player: FidePlayer,
}

impl PlayerBuilder {
    fn finish(&mut self) -> Option<FidePlayer> {
        // Validar que el player tenga ID
        if self.player.id_fide == 0 {
            return None;
        }
        Some(std::mem::take(&mut self.player))
    }

    fn apply_text(&mut self) {
        let value = self.current_text.trim().to_string();
        match self.current_tag.as_str() {
            "fideid" => {
                self.player.id_fide = value.parse().unwrap_or(0);
            }
            "name" => self.player.nombre = value,
            "country" => self.player.federacion = value,
            "sex" => self.player.sexo = value,
            "title" => self.player.titulo = value,
            "w_title" => self.player.titulo_femenino = value,
            "o_title" => self.player.titulo_arbitro = value,
            "rating" => self.player.elo_standard = value.parse().unwrap_or(0),
            "games" => self.player.partidas_standard = value.parse().unwrap_or(0),
            "k" => self.player.k_standard = value.parse().unwrap_or(0),
            "rapid_rating" => self.player.elo_rapid = value.parse().unwrap_or(0),
            "rapid_games" => self.player.partidas_rapid = value.parse().unwrap_or(0),
            "rapid_k" => self.player.k_rapid = value.parse().unwrap_or(0),
            "blitz_rating" => self.player.elo_blitz = value.parse().unwrap_or(0),
            "blitz_games" => self.player.partidas_blitz = value.parse().unwrap_or(0),
            "blitz_k" => self.player.k_blitz = value.parse().unwrap_or(0),
            "birthday" => self.player.anio_nacimiento = value.parse().unwrap_or(0),
            "flag" => self.player.flag = value,
            _ => {}
        }
        self.current_text.clear();
    }
}

pub struct FideParser;

impl FideParser {
    pub fn new() -> Self {
        Self
    }

    /// Parsea un archivo XML de FIDE usando streaming
    pub fn parse_file(&mut self, path: &str) -> AppResult<Vec<FidePlayer>> {
        let file = File::open(path)
            .map_err(|e| AppError::Io(e))?;
        
        let file_size = file.metadata()
            .map(|m| m.len())
            .unwrap_or(0);
        
        tracing::info!("Archivo XML: {} ({:.2} MB)", path, file_size as f64 / 1_048_576.0);
        
        let file = File::open(path).map_err(|e| AppError::Io(e))?;
        let reader = BufReader::new(file);
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);
        
        let mut buf = Vec::with_capacity(1024);
        let mut builder = PlayerBuilder::default();
        let mut players = Vec::with_capacity(2_000_000);
        let mut in_player = false;
        let mut skipped = 0;
        let mut total_elements = 0;
        
        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    total_elements += 1;
                    
                    if tag_name == "player" {
                        in_player = true;
                        builder = PlayerBuilder::default();
                    } else if in_player {
                        builder.current_tag = tag_name;
                        builder.current_text.clear();
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if in_player {
                        if let Ok(text) = e.unescape() {
                            builder.current_text.push_str(&text);
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    if tag_name == "player" {
                        if let Some(player) = builder.finish() {
                            players.push(player);
                        } else {
                            skipped += 1;
                        }
                        in_player = false;
                    } else if in_player && tag_name == builder.current_tag {
                        builder.apply_text();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(AppError::Validation(format!(
                        "Error XML en elemento {}: {}",
                        total_elements, e
                    )));
                }
                _ => {}
            }
            buf.clear();
            
            // Log de progreso cada 100.000 jugadores
            if players.len() > 0 && players.len() % 100_000 == 0 {
                tracing::info!("Procesados {} jugadores...", players.len());
            }
        }
        
        tracing::info!(
            "✅ Parseados {} jugadores de {} ({} elementos XML, {} jugadores inválidos)",
            players.len(),
            path,
            total_elements,
            skipped
        );
        
        Ok(players)
    }
}
