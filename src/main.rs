mod config;
mod db;
mod error;
mod models;
mod repository;
mod services;
mod tui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use sqlx::MySqlPool;
use std::io;

use crate::config::Config;
use crate::db::pool::create_pool;
use crate::error::AppResult;
use crate::services::MemberService;
use crate::tui::app::{App, ActiveScreen};
use crate::tui::event::handle_events;
use crate::tui::ui::draw;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_env_filter("fedechess=info")
        .init();

    // Cargar configuración
    let config = Config::from_env()?;

    // Conectar a la BBDD
    let pool = create_pool(&config.database_url).await?;

    // Ejecutar TUI
    if let Err(e) = run_tui(&pool).await {
        tracing::error!("Error en TUI: {}", e);
    }

    Ok(())
}

async fn run_tui(pool: &MySqlPool) -> AppResult<()> {
    // Configurar terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Cargar datos iniciales
    let service = MemberService::new(pool);
    let mut app = App::new();
    match service.list_members().await {
        Ok(members) => {
            app.status_message = Some(format!("{} federados cargados", members.len()));
            app.members = members;
        }
        Err(e) => {
            app.status_message = Some(format!("Error: {}", e));
        }
    }

    // Main loop
    while app.running {
        terminal.draw(|frame| draw(frame, &app))?;
        handle_events(&mut app)?;

        // Si volvemos a MemberList desde Search, recargar resultados
        if app.active_screen == ActiveScreen::MemberList
           && !app.search_query.is_empty()
           && app.members.is_empty() {
            let results = service.search_members(&app.search_query).await?;
            app.members = results;
            app.search_query.clear();
        }
    }

    // Restaurar terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
