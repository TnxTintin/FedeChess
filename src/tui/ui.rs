use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Row, Table, Cell, List, ListItem},
};
use crate::tui::app::{App, ActiveScreen};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_header(frame, chunks[0], app);
    
    match app.active_screen {
        ActiveScreen::Dashboard => draw_dashboard(frame, chunks[1]),
        ActiveScreen::FederateList => draw_federate_list(frame, chunks[1], app),
        ActiveScreen::Search => draw_search(frame, chunks[1], app),
        ActiveScreen::FederateDetail => draw_federate_detail(frame, chunks[1], app),
    }

    draw_footer(frame, chunks[2], app);
}

fn draw_header(frame: &mut Frame, area: Rect, _app: &App) {
    let title = Paragraph::new(" ♟  FedeChess - Federación de Ajedrez  ♟")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" FedeChess "));
    frame.render_widget(title, area);
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let help = match app.active_screen {
        ActiveScreen::Dashboard => " [1] Federados  [2] Buscar  [q] Salir ",
        ActiveScreen::FederateList => " [↑/↓] Navegar  [Enter] Ver  [/] Buscar  [Esc] Volver ",
        ActiveScreen::Search => " [Enter] Buscar  [Esc] Cancelar ",
        ActiveScreen::FederateDetail => " [Esc] Volver ",
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
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Menú Principal "));
    frame.render_widget(list, area);
}

fn draw_federate_list(frame: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec![
        Cell::from("ID").style(Style::default().fg(Color::Yellow)),
        Cell::from("Cod.Fed").style(Style::default().fg(Color::Yellow)),
        Cell::from("Apellidos, Nombre").style(Style::default().fg(Color::Yellow)),
        Cell::from("Elo").style(Style::default().fg(Color::Yellow)),
        Cell::from("Categoría").style(Style::default().fg(Color::Yellow)),
        Cell::from("Estado").style(Style::default().fg(Color::Yellow)),
    ]).height(1);

    let rows: Vec<Row> = app.federates.iter().enumerate().map(|(i, f)| {
        let style = if i == app.selected_index {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::default()
        };
        let status = if f.active { "✓ Activo" } else { "✗ Baja" };
        Row::new(vec![
            Cell::from(f.id.to_string()),
            Cell::from(f.federation_code.clone()),
            Cell::from(format!("{}, {}", f.last_name, f.first_name)),
            Cell::from(f.elo_standard.map_or("-".to_string(), |e| e.to_string())),
            Cell::from(f.category.clone().unwrap_or_else(|| "-".to_string())),
            Cell::from(status),
        ]).style(style)
    }).collect();

    let widths = [
        Constraint::Length(6),
        Constraint::Length(12),
        Constraint::Percentage(40),
        Constraint::Length(8),
        Constraint::Length(12),
        Constraint::Length(10),
    ];

    let title = format!(" Federados ({}) ", app.federates.len());
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(table, area);
}

fn draw_search(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!("Buscar: {}█", app.search_query);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Búsqueda "));
    frame.render_widget(paragraph, area);
}

fn draw_federate_detail(frame: &mut Frame, area: Rect, app: &App) {
    let content = if let Some(f) = app.federates.get(app.selected_index) {
        format!(
            "ID:              {}\n\
             Cod. Federativo: {}\n\
             FIDE ID:         {}\n\
             \n\
             Nombre:          {} {}\n\
             Documento:       {} {}\n\
             Fecha nacim.:    {}\n\
             Género:          {}\n\
             \n\
             Email:           {}\n\
             Teléfono:        {}\n\
             Dirección:       {}\n\
             CP:              {} {}\n\
             Provincia:       {}\n\
             \n\
             Elo Estándar:    {}\n\
             Elo Rápido:      {}\n\
             Elo Blitz:       {}\n\
             Título FIDE:     {}\n\
             Título Nac.:     {}\n\
             Categoría:       {}\n\
             \n\
             Tipo Federac.:   {}\n\
             Año Federac.:    {}\n\
             Estado:          {}",
            f.id,
            f.federation_code,
            f.fide_id.map_or("-".to_string(), |e| e.to_string()),
            f.first_name, f.last_name,
            f.document_type, f.document_number,
            f.birth_date.map_or("-".to_string(), |d| d.format("%d/%m/%Y").to_string()),
            f.gender.as_deref().unwrap_or("-"),
            f.email.as_deref().unwrap_or("-"),
            f.phone.as_deref().unwrap_or("-"),
            f.address.as_deref().unwrap_or("-"),
            f.postal_code.as_deref().unwrap_or("-"),
            f.city.as_deref().unwrap_or("-"),
            f.province.as_deref().unwrap_or("-"),
            f.elo_standard.map_or("-".to_string(), |e| e.to_string()),
            f.elo_rapid.map_or("-".to_string(), |e| e.to_string()),
            f.elo_blitz.map_or("-".to_string(), |e| e.to_string()),
            f.fide_title,
            f.national_title,
            f.category.as_deref().unwrap_or("-"),
            f.federation_type,
            f.federation_year.map_or("-".to_string(), |y| y.to_string()),
            if f.active { "Activo" } else { "Baja" },
        )
    } else {
        "Sin selección".to_string()
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Detalle del Federado "));
    frame.render_widget(paragraph, area);
}
