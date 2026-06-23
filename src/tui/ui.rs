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
        ActiveScreen::FederadoList => " [↑/↓] Nav  [Enter] Ver  [/] Buscar  [r] Reset  [Esc] Volver ",
        ActiveScreen::Search => " [Enter] Buscar  [Esc] Cancelar ",
        ActiveScreen::FederadoDetail => " [Esc] Volver ",
    };
    
    // Añadir filtro activo al mensaje de estado
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

    let rows: Vec<Row> = app.federados.iter().enumerate().map(|(i, f)| {
        let style = if i == app.selected_index {
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
        Constraint::Length(5),  // ID
        Constraint::Length(10), // FADA
        Constraint::Length(8),  // FIDE
        Constraint::Min(25),    // Apellidos, Nombre
        Constraint::Length(6),  // Fnac
        Constraint::Length(8),  // Elo FADA
        Constraint::Length(8),  // Elo Std
        Constraint::Length(8),  // Elo Rpd
        Constraint::Length(8),  // Elo Blz
        Constraint::Length(10), // Categoria
        Constraint::Length(8),  // Estado
    ];

    let filter_info = app.active_filter.as_ref()
        .map(|f| format!(" [filtro: '{}']", f))
        .unwrap_or_default();
    let title = format!(" Federados ({}){} ", app.federados.len(), filter_info);
    
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(table, area);
}

fn draw_search(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!("Buscar: {}█", app.search_query);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Busqueda "));
    frame.render_widget(paragraph, area);
}

fn draw_federado_detail(frame: &mut Frame, area: Rect, app: &App) {
    let content = if let Some(f) = app.federados.get(app.selected_index) {
        format!(
            "ID:              {}\n\
             Id. FADA:        {}\n\
             Id. FIDE:        {}\n\
             \n\
             Nombre:          {} {}\n\
             Documento:       {} {}\n\
             Fecha nacim.:    {}\n\
             Genero:          {}\n\
             \n\
             Email:           {}\n\
             Telefono:        {}\n\
             Direccion:       {}\n\
             CP:              {} {}\n\
             Provincia:       {}\n\
             \n\
             Elo Estandar:    {}\n\
             Elo Rapido:      {}\n\
             Elo Blitz:       {}\n\
             Titulo FIDE:     {}\n\
             Titulo Nac.:     {}\n\
             Categoria:       {}\n\
             \n\
             Alta Federativa: {}\n\
             Estado:          {}",
            f.id,
            f.id_fada,
            f.id_fide.map_or("-".to_string(), |e| e.to_string()),
            f.nombre, f.apellidos,
            f.tipo_documento, f.numero_documento,
            f.fnac.map_or("-".to_string(), |year| year.to_string()),
            f.genero.as_deref().unwrap_or("-"),
            f.email.as_deref().unwrap_or("-"),
            f.telefono.as_deref().unwrap_or("-"),
            f.direccion.as_deref().unwrap_or("-"),
            f.cp.as_deref().unwrap_or("-"),
            f.localidad.as_deref().unwrap_or("-"),
            f.provincia.as_deref().unwrap_or("-"),
            f.elo_standard.map_or("-".to_string(), |e| e.to_string()),
            f.elo_rapid.map_or("-".to_string(), |e| e.to_string()),
            f.elo_blitz.map_or("-".to_string(), |e| e.to_string()),
            f.titulo_fide,
            f.titulo_nacional,
            f.categoria.as_deref().unwrap_or("-"),
            f.alta_federativa.map_or("-".to_string(), |y| y.to_string()),
            if f.activo { "Activo" } else { "Baja" },
        )
    } else {
        "Sin seleccion".to_string()
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Detalle del Federado "));
    frame.render_widget(paragraph, area);
}
