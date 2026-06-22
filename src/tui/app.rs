use crate::models::Member;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveScreen {
    Dashboard,
    MemberList,
    MemberDetail,
    Search,
}

pub struct App {
    pub running: bool,
    pub active_screen: ActiveScreen,
    pub members: Vec<Member>,
    pub selected_index: usize,
    pub search_query: String,
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            active_screen: ActiveScreen::Dashboard,
            members: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
            status_message: None,
        }
    }

    pub fn next_member(&mut self) {
        if !self.members.is_empty() && self.selected_index < self.members.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous_member(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
