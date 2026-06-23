// src/tui/mod.rs
pub mod app;
pub mod event;
pub mod ui;
pub mod screens;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use sqlx::MySqlPool;
use std::io;

use crate::error::AppResult;
use crate::services::FederadoService;
use crate::tui::app::App;
use crate::tui::event::handle_events;
use crate::tui::ui::draw;

pub async fn run(pool: &MySqlPool) -> AppResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let service = FederadoService::new(pool);
    let mut app = App::new();
    
    match service.list_federados().await {
        Ok(federados) => app.load_all(federados),
        Err(e) => {
            app.status_message = Some(format!("Error: {}", e));
        }
    }

    while app.running {
        terminal.draw(|frame| draw(frame, &app))?;
        handle_events(&mut app)?;

        if app.search_pending {
            let query = app.search_query.clone();
            match service.search_federados(&query).await {
                Ok(found) => app.apply_search_results(found, query),
                Err(e) => app.status_message = Some(format!("Error: {}", e)),
            }
            app.search_pending = false;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
