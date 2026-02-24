#[derive(Clone, Copy)]
pub enum Section {
    Home,
    Projects,
    Experience,
    Systems,
    Contact,
}

pub struct App {
    pub section: Section,
}

impl App {
    pub fn new() -> Self {
        Self {
            section: Section::Home,
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
}
