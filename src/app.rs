#[derive(Clone, Copy, PartialEq)]
pub enum Section {
    Home,
    Projects,
    Experience,
    Systems,
    Contact,
}

pub struct App {
    pub section: Section,
    pub project_index: usize,
    pub projects: Vec<&'static str>,
}

impl App {
    pub fn new() -> Self {
        Self {
            section: Section::Home,
            project_index: 0,
            projects: vec!["Glazel", "GameDaddy", "DecentClang"],
        }
    }

    pub fn set_section(&mut self, index: char) {
        self.section = match index {
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
}
