use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::tui::app::{App, ActiveScreen, SortColumn};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(true);
            }

            match app.active_screen {
                ActiveScreen::Dashboard => handle_dashboard_keys(app, key.code),
                ActiveScreen::FederadoList => handle_federado_list_keys(app, key.code),
                ActiveScreen::Search => handle_search_keys(app, key.code),
                ActiveScreen::FederadoDetail => handle_detail_keys(app, key.code),
            }
        }
    }
    Ok(true)
}

fn handle_dashboard_keys(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
        KeyCode::Char('1') => app.active_screen = ActiveScreen::FederadoList,
        KeyCode::Char('2') => app.active_screen = ActiveScreen::Search,
        _ => {}
    }
}

fn handle_federado_list_keys(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.active_screen = ActiveScreen::Dashboard,
        KeyCode::Down | KeyCode::Char('j') => app.next_federado(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_federado(),
        KeyCode::PageDown => app.page_down(),
        KeyCode::PageUp => app.page_up(),
        KeyCode::Enter => app.active_screen = ActiveScreen::FederadoDetail,
        KeyCode::Char('/') => {
            app.search_query.clear();
            app.active_screen = ActiveScreen::Search;
        }
        KeyCode::Char('r') => {
            if app.active_filter.is_some() {
                app.reset_filter();
            }
        }
        // Teclas de ordenamiento
        KeyCode::Char('n') => app.sort_by(SortColumn::Id),
        KeyCode::Char('f') => app.sort_by(SortColumn::IdFada),
        KeyCode::Char('i') => app.sort_by(SortColumn::IdFide),
        KeyCode::Char('a') => app.sort_by(SortColumn::Apellidos),
        KeyCode::Char('N') => app.sort_by(SortColumn::Nombre),
        KeyCode::Char('F') => app.sort_by(SortColumn::Fnac),
        KeyCode::Char('e') => app.sort_by(SortColumn::EloFada),
        KeyCode::Char('s') => app.sort_by(SortColumn::EloStandard),
        KeyCode::Char('p') => app.sort_by(SortColumn::EloRapid),
        KeyCode::Char('b') => app.sort_by(SortColumn::EloBlitz),
        KeyCode::Char('c') => app.sort_by(SortColumn::Categoria),
        KeyCode::Char('t') => app.sort_by(SortColumn::Estado),
        KeyCode::Char('l') => app.sort_by(SortColumn::Club),
        _ => {}
    }
}

fn handle_search_keys(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => app.active_screen = ActiveScreen::FederadoList,
        KeyCode::Enter => {
            app.search_pending = true;
            app.active_screen = ActiveScreen::FederadoList;
        }
        KeyCode::Backspace => { app.search_query.pop(); }
        KeyCode::Char(c) => app.search_query.push(c),
        _ => {}
    }
}

fn handle_detail_keys(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc | KeyCode::Char('q') => app.active_screen = ActiveScreen::FederadoList,
        _ => {}
    }
}
