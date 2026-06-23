mod config;
mod db;
mod error;
mod fide;
mod models;
mod repository;
mod services;
mod tui;

use std::env;
use crate::config::Config;
use crate::db::pool::create_pool;
use crate::error::AppResult;
use crate::fide::{FideParser, FideImporter};

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter("fedechess=info")
        .init();

    let config = Config::from_env()?;
    let pool = create_pool(&config.database_url).await?;

    // Modo CLI: importar archivo FIDE
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "import-fide" {
        if args.len() < 3 {
            eprintln!("Uso: fedechess import-fide <archivo.txt>");
            std::process::exit(1);
        }
        
        let file_path = &args[2];
        return import_fide_file(&pool, file_path).await;
    }

    // Modo por defecto: TUI
    if let Err(e) = tui::run(&pool).await {
        tracing::error!("Error en TUI: {}", e);
    }

    Ok(())
}

async fn import_fide_file(pool: &sqlx::MySqlPool, file_path: &str) -> AppResult<()> {
    tracing::info!("Importando archivo FIDE: {}", file_path);
    
    let mut parser = FideParser::new();
    let players = parser.parse_file(file_path)?;
    
    tracing::info!("Jugadores parseados: {}", players.len());
    
    let importer = FideImporter::new(pool);
    let imported = importer.import(&players).await?;
    
    tracing::info!("✅ Importados {} jugadores a fide_players", imported);
    
    // Sincronizar Elos con federados
    let synced = importer.sync_elos().await?;
    tracing::info!("✅ Sincronizados {} federados con Elos FIDE", synced);
    
    Ok(())
}
