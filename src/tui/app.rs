use crate::models::Federado;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveScreen {
    Dashboard,
    FederadoList,
    FederadoDetail,
    Search,
    Consultas,
    Arbitros,
    Monitores,
    Ayuda,
    Contacto,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    Id,
    IdFada,
    IdFide,
    Apellidos,
    Nombre,
    Fnac,
    EloFada,
    EloStandard,
    EloRapid,
    EloBlitz,
    Categoria,
    Estado,
    Club,
}

pub struct App {
    pub running: bool,
    pub active_screen: ActiveScreen,
    pub all_federados: Vec<Federado>,
    pub federados: Vec<Federado>,
    pub selected_index: usize,
    pub search_query: String,
    pub active_filter: Option<String>,
    pub search_pending: bool,
    pub status_message: Option<String>,
    
    // Nuevos campos para paginación
    pub page_size: usize,
    pub current_page: usize,
    
    // Nuevos campos para ordenamiento
    pub sort_column: Option<SortColumn>,
    pub sort_ascending: bool,
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
            page_size: 20,
            current_page: 0,
            sort_column: None,
            sort_ascending: true,
        }
    }

    pub fn next_federado(&mut self) {
        let max_index = self.get_page_end().min(self.federados.len()) - 1;
        if self.selected_index < max_index {
            self.selected_index += 1;
        }
    }

    pub fn previous_federado(&mut self) {
        let min_index = self.get_page_start();
        if self.selected_index > min_index {
            self.selected_index -= 1;
        }
    }

    pub fn page_down(&mut self) {
        let total_pages = self.total_pages();
        if self.current_page < total_pages - 1 {
            self.current_page += 1;
            self.selected_index = self.get_page_start();
        }
    }

    pub fn page_up(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.selected_index = self.get_page_start();
        }
    }

    pub fn get_page_start(&self) -> usize {
        self.current_page * self.page_size
    }

    pub fn get_page_end(&self) -> usize {
        (self.current_page + 1) * self.page_size
    }

    pub fn total_pages(&self) -> usize {
        if self.federados.is_empty() {
            1
        } else {
            (self.federados.len() + self.page_size - 1) / self.page_size
        }
    }

    pub fn sort_by(&mut self, column: SortColumn) {
        if self.sort_column == Some(column) {
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column = Some(column);
            self.sort_ascending = true;
        }

        self.federados.sort_by(|a, b| {
            let cmp = match column {
                SortColumn::Id => a.id.cmp(&b.id),
                SortColumn::IdFada => a.id_fada.cmp(&b.id_fada),
                SortColumn::IdFide => a.id_fide.cmp(&b.id_fide),
                SortColumn::Apellidos => a.apellidos.cmp(&b.apellidos),
                SortColumn::Nombre => a.nombre.cmp(&b.nombre),
                SortColumn::Fnac => a.fnac.cmp(&b.fnac),
                SortColumn::EloFada => a.elo_fada.cmp(&b.elo_fada),
                SortColumn::EloStandard => a.elo_standard.cmp(&b.elo_standard),
                SortColumn::EloRapid => a.elo_rapid.cmp(&b.elo_rapid),
                SortColumn::EloBlitz => a.elo_blitz.cmp(&b.elo_blitz),
                SortColumn::Categoria => a.categoria.cmp(&b.categoria),
                SortColumn::Estado => a.activo.cmp(&b.activo),
                SortColumn::Club => a.id_club.cmp(&b.id_club),
            };
            
            if self.sort_ascending { cmp } else { cmp.reverse() }
        });

        self.current_page = 0;
        self.selected_index = 0;
    }

    pub fn load_all(&mut self, federados: Vec<Federado>) {
        let n = federados.len();
        self.all_federados = federados.clone();
        self.federados = federados;
        self.current_page = 0;
        self.selected_index = 0;
        self.status_message = Some(format!("{} federados cargados", n));
    }

    pub fn reset_filter(&mut self) {
        self.federados = self.all_federados.clone();
        self.active_filter = None;
        self.current_page = 0;
        self.selected_index = 0;
        self.status_message = Some(format!("{} federados (sin filtro)", self.federados.len()));
    }

    pub fn apply_search_results(&mut self, results: Vec<Federado>, query: String) {
        let n = results.len();
        self.federados = results;
        self.selected_index = 0;
        self.current_page = 0;
        self.active_filter = Some(query);
        self.status_message = Some(format!("{} resultados encontrados", n));
    }
}
