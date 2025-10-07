use eframe::{self, egui};
use mysql::*;
use mysql::prelude::*;
use std::error::Error as StdError;

fn main() -> std::result::Result<(), Box<dyn StdError>> {
    // Configuración de la conexión a la base de datos
    let db_url = "mysql://fedechess:FedTin2025@localhost:3306/fedechess";
    let pool = Pool::new(db_url)?;
    
    // Crear la aplicación GUI
    let options = eframe::NativeOptions::default();
    
    // eframe::run_native no devuelve Result, así que no usamos ?
    eframe::run_native(
        "Gestión de Jugadores",
        options,
        Box::new(|_cc| Box::new(App::new(pool))),
    );
    
    Ok(())
}

struct App {
    id_fada: String,
    id_fide: String,
    player: String,
    title: String,
    mensaje: String,
    db_pool: Pool,
}

impl App {
    fn new(db_pool: Pool) -> Self {
        Self {
            id_fada: String::new(),
            id_fide: String::new(),
            player: String::new(),
            title: String::new(),
            mensaje: String::new(),
            db_pool,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Agregar Nuevo Jugador");

            // Campos de entrada
            ui.label("ID FADA:");
            ui.text_edit_singleline(&mut self.id_fada);

            ui.label("ID FIDE:");
            ui.text_edit_singleline(&mut self.id_fide);

            ui.label("Nombre del Jugador:");
            ui.text_edit_singleline(&mut self.player);

            ui.label("Título:");
            ui.text_edit_singleline(&mut self.title);

            // Botón para guardar
            if ui.button("Guardar").clicked() {
                // Validar que todos los campos estén completos
                if self.id_fada.is_empty()
                    || self.id_fide.is_empty()
                    || self.player.is_empty()
                    || self.title.is_empty()
                {
                    self.mensaje = "Por favor, completa todos los campos.".to_string();
                } else {
                    // Insertar en la base de datos
                    match self.db_pool.get_conn() {
                        Ok(mut conn) => {
                            match conn.exec_drop(
                                "INSERT INTO players (id_fada, id_fide, player, title) VALUES (?, ?, ?, ?)",
                                (&self.id_fada, &self.id_fide, &self.player, &self.title),
                            ) {
                                Ok(_) => {
                                    self.mensaje = "¡Jugador agregado exitosamente!".to_string();
                                    // Limpiar los campos después de guardar
                                    self.id_fada.clear();
                                    self.id_fide.clear();
                                    self.player.clear();
                                    self.title.clear();
                                }
                                Err(e) => self.mensaje = format!("Error al insertar: {}", e),
                            }
                        }
                        Err(e) => self.mensaje = format!("Error al conectar con la base de datos: {}", e),
                    }
                }
            }

            // Mostrar mensaje de estado
            ui.label(&self.mensaje);
        });
    }
}
