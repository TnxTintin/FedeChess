use crate::models::Federado;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveScreen {
    Dashboard,
    FederadoList,
    FederadoDetail,
    Search,
}

pub struct App {
    pub running: bool,
    pub active_screen: ActiveScreen,
    pub all_federados: Vec<Federado>,    // Lista completa (sin filtros)
    pub federados: Vec<Federado>,        // Vista actual (puede estar filtrada)
    pub selected_index: usize,
    pub search_query: String,
    pub active_filter: Option<String>,   // Último criterio aplicado (para mostrar)
    pub search_pending: bool,            // Flag para disparar búsqueda async
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            active_screen: ActiveScreen::Dashboard,
            all_federados: Vec::new(),
            federados: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
            active_filter: None,
            search_pending: false,
            status_message: None,
        }
    }

    pub fn next_federado(&mut self) {
        if !self.federados.is_empty() && self.selected_index < self.federados.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous_federado(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Cargar datos iniciales (todos los federados)
    pub fn load_all(&mut self, federados: Vec<Federado>) {
        let n = federados.len();
        self.all_federados = federados.clone();
        self.federados = federados;
        self.status_message = Some(format!("{} federados cargados", n));
    }

    /// Resetear filtro: volver a ver todos
    pub fn reset_filter(&mut self) {
        self.federados = self.all_federados.clone();
        self.active_filter = None;
        self.selected_index = 0;
        self.status_message = Some(format!("{} federados (sin filtro)", self.federados.len()));
    }

    /// Aplicar resultados de búsqueda
    pub fn apply_search_results(&mut self, results: Vec<Federado>, query: String) {
        let n = results.len();
        self.federados = results;
        self.selected_index = 0;
        self.active_filter = Some(query);
        self.status_message = Some(format!("{} resultados encontrados", n));
    }
}
