use crate::tui::app::{ActiveScreen, App};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer/status
        ])
        .split(frame.area());

    draw_header(frame, chunks[0], app);

    match app.active_screen {
        ActiveScreen::Dashboard => draw_dashboard(frame, chunks[1]),
        ActiveScreen::MemberList => draw_member_list(frame, chunks[1], app),
        ActiveScreen::Search => draw_search(frame, chunks[1], app),
        ActiveScreen::MemberDetail => draw_member_detail(frame, chunks[1], app),
    }

    draw_footer(frame, chunks[2], app);
}

fn draw_header(frame: &mut Frame, area: Rect, _app: &App) {
    let title = Paragraph::new(" ♟  FedeChess - Federación de Ajedrez  ♟")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title(" FedeChess "));
    frame.render_widget(title, area);
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let help = match app.active_screen {
        ActiveScreen::Dashboard => " [1] Federados  [2] Buscar  [q] Salir ",
        ActiveScreen::MemberList => " [↑/↓] Navegar  [Enter] Ver  [/] Buscar  [Esc] Volver ",
        ActiveScreen::Search => " [Enter] Buscar  [Esc] Cancelar ",
        ActiveScreen::MemberDetail => " [Esc] Volver ",
    };
    let status = app.status_message.as_deref().unwrap_or("Listo");
    let text = format!("{} | {}", status, help);

    let footer = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, area);
}

fn draw_dashboard(frame: &mut Frame, area: Rect) {
    let items = vec![
        ListItem::new("  [1] Gestión de Federados"),
        ListItem::new("  [2] Gestión de Competiciones"),
        ListItem::new("  [3] Órganos de la Federación"),
        ListItem::new("  [4] Clubs"),
        ListItem::new("  [5] Informes"),
        ListItem::new(""),
        ListItem::new("  [q] Salir"),
    ];
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Menú Principal "),
    );
    frame.render_widget(list, area);
}

fn draw_member_list(frame: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec![
        Cell::from("ID").style(Style::default().fg(Color::Yellow)),
        Cell::from("Fed.ID").style(Style::default().fg(Color::Yellow)),
        Cell::from("Apellidos, Nombre").style(Style::default().fg(Color::Yellow)),
        Cell::from("Elo Std").style(Style::default().fg(Color::Yellow)),
        Cell::from("Estado").style(Style::default().fg(Color::Yellow)),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .members
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let style = if i == app.selected_index {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            let status = if m.active { "✓ Activo" } else { "✗ Baja" };
            Row::new(vec![
                Cell::from(m.id.to_string()),
                Cell::from(m.federation_id.clone()),
                Cell::from(format!("{}, {}", m.last_name, m.first_name)),
                Cell::from(m.elo_standard.map_or("-".to_string(), |e| e.to_string())),
                Cell::from(status),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(6),
        Constraint::Length(12),
        Constraint::Percentage(50),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let title = format!(" Federados ({}) ", app.members.len());
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(table, area);
}

fn draw_search(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!("Buscar: {}█", app.search_query);
    let paragraph =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Búsqueda "));
    frame.render_widget(paragraph, area);
}

fn draw_member_detail(frame: &mut Frame, area: Rect, app: &App) {
    let content = if let Some(m) = app.members.get(app.selected_index) {
        format!(
            "ID:          {}\n\
             Fed.ID:      {}\n\
             Nombre:      {} {}\n\
             Documento:   {} {}\n\
             Elo Std:     {}\n\
             Elo Rápido:  {}\n\
             Elo Blitz:   {}\n\
             Título:      {}\n\
             FIDE ID:     {}\n\
             Estado:      {}",
            m.id,
            m.federation_id,
            m.first_name,
            m.last_name,
            m.document_type,
            m.document_number,
            m.elo_standard.map_or("-".to_string(), |e| e.to_string()),
            m.elo_rapid.map_or("-".to_string(), |e| e.to_string()),
            m.elo_blitz.map_or("-".to_string(), |e| e.to_string()),
            m.title,
            m.fide_id.map_or("-".to_string(), |e| e.to_string()),
            if m.active { "Activo" } else { "Baja" },
        )
    } else {
        "Sin selección".to_string()
    };

    let paragraph = Paragraph::new(content).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Detalle del Federado "),
    );
    frame.render_widget(paragraph, area);
}
