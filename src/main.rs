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
use crate::services::FederateService;
use crate::tui::app::{App, ActiveScreen};
use crate::tui::event::handle_events;
use crate::tui::ui::draw;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter("fedechess=info")
        .init();

    let config = Config::from_env()?;
    let pool = create_pool(&config.database_url).await?;

    if let Err(e) = run_tui(&pool).await {
        tracing::error!("Error en TUI: {}", e);
    }

    Ok(())
}

async fn run_tui(pool: &MySqlPool) -> AppResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let service = FederateService::new(pool);
    let mut app = App::new();
    
    match service.list_federates().await {
        Ok(federates) => {
            app.status_message = Some(format!("{} federados cargados", federates.len()));
            app.federates = federates;
        }
        Err(e) => {
            app.status_message = Some(format!("Error: {}", e));
        }
    }

    while app.running {
        terminal.draw(|frame| draw(frame, &app))?;
        handle_events(&mut app)?;

        if app.active_screen == ActiveScreen::FederateList 
           && !app.search_query.is_empty() 
           && app.federates.is_empty() {
            let results = service.search_federates(&app.search_query).await?;
            app.federates = results;
            app.search_query.clear();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
