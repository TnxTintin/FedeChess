use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Row, Table, Cell, List, ListItem},
};
use crate::tui::app::{App, ActiveScreen, SortColumn};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_header(frame, chunks[0]);

    match app.active_screen {
        ActiveScreen::Dashboard => draw_dashboard(frame, chunks[1]),
        ActiveScreen::FederadoList => draw_federado_list(frame, chunks[1], app),
        ActiveScreen::Search => draw_search(frame, chunks[1], app),
        ActiveScreen::FederadoDetail => draw_federado_detail(frame, chunks[1], app),
    }

    draw_footer(frame, chunks[2], app);
}

fn draw_header(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(" ♟  FedeChess - Federacion de Ajedrez  ♟")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" FedeChess "));
    frame.render_widget(title, area);
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let help = match app.active_screen {
        ActiveScreen::Dashboard => " [1] Federados  [2] Buscar  [q] Salir ",
        ActiveScreen::FederadoList => " [↑/↓] Nav  [PgUp/PgDn] Página  [Enter] Ver  [/] Buscar  [r] Reset  [n/f/i/a/N/F/e/s/p/b/c/t/l] Ordenar  [Esc] Volver ",
        ActiveScreen::Search => " [Enter] Buscar  [Esc] Cancelar ",
        ActiveScreen::FederadoDetail => " [Esc] Volver ",
    };

    let status = if let Some(filter) = &app.active_filter {
        format!("Filtro: '{}' | {}", filter, app.status_message.as_deref().unwrap_or(""))
    } else {
        app.status_message.clone().unwrap_or_else(|| "Listo".to_string())
    };

    let text = format!("{} | {}", status, help);

    let footer = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, area);
}

fn draw_dashboard(frame: &mut Frame, area: Rect) {
    let items = vec![
        ListItem::new("  [1] Gestion de Federados"),
        ListItem::new("  [2] Gestion de Competiciones"),
        ListItem::new("  [3] Organos de la Federacion"),
        ListItem::new("  [4] Clubs"),
        ListItem::new("  [5] Informes"),
        ListItem::new(""),
        ListItem::new("  [q] Salir"),
    ];
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Menu Principal "));
    frame.render_widget(list, area);
}

fn draw_federado_list(frame: &mut Frame, area: Rect, app: &App) {
    // Calcular paginación dinámica
    let table_height = area.height.saturating_sub(5) as usize;
    let page_size = table_height.max(1);

    let start = app.current_page * page_size;
    let end = (start + page_size).min(app.federados.len());
    let page_items = if start < app.federados.len() {
        &app.federados[start..end]
    } else {
        &[]
    };

    let header = Row::new(vec![
        Cell::from("ID").style(Style::default().fg(Color::Yellow)),
        Cell::from("FADA").style(Style::default().fg(Color::Yellow)),
        Cell::from("FIDE").style(Style::default().fg(Color::Yellow)),
        Cell::from("Apellidos, Nombre").style(Style::default().fg(Color::Yellow)),
        Cell::from("Fnac").style(Style::default().fg(Color::Yellow)),
        Cell::from("Elo FADA").style(Style::default().fg(Color::Yellow)),
        Cell::from("Elo Std").style(Style::default().fg(Color::Yellow)),
        Cell::from("Elo Rpd").style(Style::default().fg(Color::Yellow)),
        Cell::from("Elo Blz").style(Style::default().fg(Color::Yellow)),
        Cell::from("Categoria").style(Style::default().fg(Color::Yellow)),
        Cell::from("Estado").style(Style::default().fg(Color::Yellow)),
    ]).height(1);

    let rows: Vec<Row> = page_items.iter().enumerate().map(|(i, f)| {
        let global_index = start + i;
        let style = if global_index == app.selected_index {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::default()
        };
        let status = if f.activo { "✓ Activo" } else { "✗ Baja" };
        Row::new(vec![
            Cell::from(f.id.to_string()),
            Cell::from(f.id_fada.clone()),
            Cell::from(f.id_fide.map_or("-".to_string(), |id| id.to_string())),
            Cell::from(format!("{}, {}", f.apellidos, f.nombre)),
            Cell::from(f.fnac.map_or("-".to_string(), |y| y.to_string())),
            Cell::from(f.elo_fada.map_or("-".to_string(), |e| e.to_string())),
            Cell::from(f.elo_standard.map_or("-".to_string(), |e| e.to_string())),
            Cell::from(f.elo_rapid.map_or("-".to_string(), |e| e.to_string())),
            Cell::from(f.elo_blitz.map_or("-".to_string(), |e| e.to_string())),
            Cell::from(f.categoria.clone().unwrap_or_else(|| "-".to_string())),
            Cell::from(status),
        ]).style(style)
    }).collect();

    let widths = [
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Min(25),
        Constraint::Length(6),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(8),
    ];

    let filter_info = app.active_filter.as_ref()
        .map(|f| format!(" [filtro: '{}']", f))
        .unwrap_or_default();

    let sort_info = if let Some(col) = app.sort_column {
        let arrow = if app.sort_ascending { "↑" } else { "↓" };
        let col_name = match col {
            SortColumn::Id => "ID",
            SortColumn::IdFada => "FADA",
            SortColumn::IdFide => "FIDE",
            SortColumn::Apellidos => "Apellido",
            SortColumn::Nombre => "Nombre",
            SortColumn::Fnac => "Fnac",
            SortColumn::EloFada => "EloF",
            SortColumn::EloStandard => "EloS",
            SortColumn::EloRapid => "EloR",
            SortColumn::EloBlitz => "EloB",
            SortColumn::Categoria => "Cat",
            SortColumn::Estado => "Estado",
            SortColumn::Club => "Club",
        };
        format!(" [{}{}]", col_name, arrow)
    } else {
        String::new()
    };

    let total_pages = if app.federados.is_empty() {
        1
    } else {
        (app.federados.len() + page_size - 1) / page_size
    };
    let current_page = if total_pages == 0 { 0 } else { app.current_page.min(total_pages - 1) };

    let title = format!(
        " Federados ({}){}{} | Página {}/{} ",
        app.federados.len(),
        filter_info,
        sort_info,
        current_page + 1,
        total_pages
    );

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(table, area);
}
