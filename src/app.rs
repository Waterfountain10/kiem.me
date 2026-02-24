#[derive(Clone, Copy, PartialEq)]
pub enum Section {
    Home,
    Projects,
    Experience,
    Systems,
    Contact,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Normal,
    ProjectDetail,
}

pub struct App {
    pub section: Section,
    pub view_mode: ViewMode,
    pub project_index: usize,
    pub projects: Vec<&'static str>,
    pub scroll: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            section: Section::Home,
            view_mode: ViewMode::Normal,
            project_index: 0,
            projects: vec!["Glazel", "GameDaddy", "DecentClang"],
            scroll: 0,
        }
    }

    pub fn set_section(&mut self, key: char) {
        self.view_mode = ViewMode::Normal;
        self.section = match key {
            '1' => Section::Home,
            '2' => Section::Projects,
            '3' => Section::Experience,
            '4' => Section::Systems,
            '5' => Section::Contact,
            _ => self.section,
        };
    }

    pub fn next_project(&mut self) {
        if self.project_index + 1 < self.projects.len() {
            self.project_index += 1;
        }
    }

    pub fn previous_project(&mut self) {
        if self.project_index > 0 {
            self.project_index -= 1;
        }
    }

    pub fn open_project(&mut self) {
        if self.section == Section::Projects {
            self.view_mode = ViewMode::ProjectDetail;
        }
    }

    pub fn back(&mut self) {
        self.view_mode = ViewMode::Normal;
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }
}
