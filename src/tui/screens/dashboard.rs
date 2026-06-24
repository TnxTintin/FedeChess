use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use crate::tui::app::{App, ActiveScreen};

pub fn draw_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let items = vec![
        ListItem::new("  [1] Gestion de Federados"),
        ListItem::new("  [2] Gestion de Competiciones"),
        ListItem::new("  [3] Organos de la Federacion"),
        ListItem::new("  [4] Clubs"),
        ListItem::new("  [5] Informes"),
        ListItem::new("  [6] Consultas"),
        ListItem::new("  [7] Arbitros"),
        ListItem::new("  [8] Monitores"),
        ListItem::new("  [9] Ayuda"),
        ListItem::new("  [0] Contacto"),
        ListItem::new(""),
        ListItem::new("  [q] Salir"),
    ];
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Menu Principal "));
    frame.render_widget(list, area);
}
