use crate::models::Federate;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveScreen {
    Dashboard,
    FederateList,
    FederateDetail,
    Search,
}

pub struct App {
    pub running: bool,
    pub active_screen: ActiveScreen,
    pub federates: Vec<Federate>,
    pub selected_index: usize,
    pub search_query: String,
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            active_screen: ActiveScreen::Dashboard,
            federates: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
            status_message: None,
        }
    }

    pub fn next_federate(&mut self) {
        if !self.federates.is_empty() && self.selected_index < self.federates.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous_federate(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
